//! High-level logic for full-pipeline parsing.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use thiserror::Error;

use crate::{
    SszSchema,
    ast::{
        self, AssignExpr, Module, ModuleEntry, ModuleManager, ParseError, TyArgSpec, TyExprSpec,
    },
    schema::{self, SchemaError},
    token::{self, TokenError},
    token_tree::{self, ToktrError},
    ty_resolver::{CrossModuleTypeMap, ModuleTypeMap, ResolverError},
};

/// Represents an error from any of the phases of parsing a raw schema.
#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum SszError {
    /// Error from the tokenizer.
    #[error("tokenizer: {0}")]
    Token(#[from] TokenError),

    /// Error from the token tree parser.
    #[error("treeizer: {0}")]
    TokenTree(#[from] ToktrError),

    /// Error from the AST parser.
    #[error("parser: {0}")]
    Parser(#[from] ParseError),

    /// Error from the type resolver.
    #[error("type resolution: {0}")]
    TyResolver(#[from] ResolverError),

    /// Error from the schema generator.
    #[error("schema generation: {0}")]
    SchemaGen(#[from] SchemaError),
}

/// Helper struct for topological sorting of modules.
struct TopoSort {
    modules: HashMap<PathBuf, Module>,
    deps: HashMap<PathBuf, HashSet<PathBuf>>,
    in_progress: HashSet<PathBuf>,
    sorted: Vec<(PathBuf, Module)>,
}

impl TopoSort {
    fn new(modules: HashMap<PathBuf, Module>) -> Self {
        // Build a mapping from normalized paths (without extension) to actual paths.
        // This handles the case where imports use "state" but modules are stored as "state.ssz".
        let mut normalized_to_actual: HashMap<PathBuf, PathBuf> = HashMap::new();
        for path in modules.keys() {
            let normalized = path.with_extension("");
            normalized_to_actual.insert(normalized, path.clone());
        }

        // Build dependency graph: for each module, find what it imports.
        let mut deps: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();
        for (path, module) in &modules {
            let mut actual_deps = HashSet::new();
            for import_path in get_module_imports(module) {
                // Try to find the actual module path (with or without extension)
                if modules.contains_key(&import_path) {
                    actual_deps.insert(import_path);
                } else if let Some(actual) = normalized_to_actual.get(&import_path) {
                    actual_deps.insert(actual.clone());
                } else {
                    // Try with .ssz extension
                    let ssz_path = import_path.with_extension("ssz");
                    if modules.contains_key(&ssz_path) {
                        actual_deps.insert(ssz_path);
                    }
                }
            }
            deps.insert(path.clone(), actual_deps);
        }

        Self {
            modules,
            deps,
            in_progress: HashSet::new(),
            sorted: Vec::new(),
        }
    }

    /// Performs DFS visit for topological sort.
    fn visit(&mut self, path: PathBuf) {
        // If already processed (removed from modules), skip.
        if !self.modules.contains_key(&path) {
            return;
        }
        // Cycle detection: if we're currently processing this path, skip.
        if self.in_progress.contains(&path) {
            return;
        }

        self.in_progress.insert(path.clone());

        // Visit all dependencies first.
        if let Some(module_deps) = self.deps.get(&path).cloned() {
            for dep_path in module_deps {
                self.visit(dep_path);
            }
        }

        self.in_progress.remove(&path);

        // Remove from modules and add to sorted list (avoids cloning).
        if let Some(module) = self.modules.remove(&path) {
            self.sorted.push((path, module));
        }
    }

    /// Consumes self and returns the topologically sorted modules.
    fn sort(mut self) -> Vec<(PathBuf, Module)> {
        let paths: Vec<_> = self.modules.keys().cloned().collect();
        for path in paths {
            self.visit(path);
        }
        self.sorted
    }
}

/// Collects import paths from a type expression specification.
fn collect_imports_from_ty_expr(spec: &TyExprSpec, imports: &mut HashSet<PathBuf>) {
    match spec {
        TyExprSpec::Imported(imported) => {
            imports.insert(imported.module_path().clone());
        }
        TyExprSpec::Complex(complex) => {
            for arg in complex.args() {
                collect_imports_from_ty_arg(arg, imports);
            }
        }
        TyExprSpec::Simple(_) | TyExprSpec::None => {}
    }
}

/// Collects import paths from a type argument specification.
fn collect_imports_from_ty_arg(spec: &TyArgSpec, imports: &mut HashSet<PathBuf>) {
    match spec {
        TyArgSpec::Imported(imported) => {
            imports.insert(imported.module_path().clone());
        }
        TyArgSpec::Complex(complex) => {
            for arg in complex.args() {
                collect_imports_from_ty_arg(arg, imports);
            }
        }
        TyArgSpec::Ident(_) | TyArgSpec::IntLiteral(_) | TyArgSpec::None => {}
    }
}

/// Extracts the import paths from a module's entries by looking at all imported type references.
fn get_module_imports(module: &Module) -> HashSet<PathBuf> {
    let mut imports = HashSet::new();

    for entry in module.entries() {
        match entry {
            ModuleEntry::Assignment(assign) => match assign.value() {
                AssignExpr::Imported(imported) => {
                    imports.insert(imported.module_path().clone());
                }
                AssignExpr::Complex(complex) => {
                    for arg in complex.args() {
                        collect_imports_from_ty_arg(arg, &mut imports);
                    }
                }
                AssignExpr::Name(_) | AssignExpr::Value(_) | AssignExpr::SymbolicBinop(_, _, _) => {
                }
            },
            ModuleEntry::Class(class) => {
                collect_imports_from_ty_expr(class.parent_ty(), &mut imports);
                for field in class.fields() {
                    collect_imports_from_ty_expr(field.ty(), &mut imports);
                }
            }
        }
    }

    imports
}

/// High-level parse function.
pub fn parse_str_schema(
    files: &HashMap<PathBuf, String>,
    external_modules: &[&str],
) -> Result<(Vec<PathBuf>, HashMap<PathBuf, SszSchema>), SszError> {
    let mut module_manager = ModuleManager::new(external_modules);

    for (path, content) in files {
        // Only parse if the module hasn't been added yet (e.g., by an import from another entry
        // point)
        if module_manager.add_module_to_front(path.clone()) {
            let chars = content.chars().collect::<Vec<_>>();
            let tokens = token::parse_char_array_to_tokens(&chars)?;
            let toktrs = token_tree::parse_tokens_to_toktrs(&tokens)?;
            ast::parse_module_from_toktrs(&toktrs, path, &mut module_manager, Some(files))?;
        }
    }

    let mut schema_map = HashMap::new();
    let mut cross_module_types = CrossModuleTypeMap::new();

    // Pre-register external modules before any schema conversion occurs.
    // This ensures that when schema conversion tries to resolve types from external modules,
    // they are already registered in cross_module_types.
    for external_module in external_modules {
        let path = PathBuf::from(external_module);
        cross_module_types.insert(path, ModuleTypeMap::External);
    }

    // Collect all modules and sort them topologically so dependencies come before dependents.
    // This ensures that when we convert a module to a schema, all its imported modules have
    // already been processed and their types are available in cross_module_types.
    let topo_sort = TopoSort::new(module_manager.into_modules());
    let sorted_modules = topo_sort.sort();

    let mut parsing_order = Vec::new();
    for (path, module) in sorted_modules {
        if module.is_external() {
            cross_module_types.insert(path.clone(), ModuleTypeMap::External);
            continue;
        }
        let (schema, idents) = schema::conv_module_to_schema(&module, &cross_module_types)?;
        parsing_order.push(path.clone());
        cross_module_types.insert(path.clone(), ModuleTypeMap::Internal(idents));
        schema_map.insert(path, schema);
    }

    Ok((parsing_order, schema_map))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use crate::pipeline::parse_str_schema;

    /*fn make_ident(s: &str) -> Identifier {
        Identifier::try_from(s.to_owned()).expect("test: make ident")
    }*/

    #[test]
    fn test_pipeline_simple() {
        const SCHEMA: &str = r"
class Point2d(Container):
  x_coord: uint32
  y_coord: uint32
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_beacon_deposit_request() {
        // This is kinda bodging it, I just wanted to take a "real example".
        const SCHEMA: &str = r"
BLSPubkey = List[byte, 96]
BLSSignature = List[byte, 96]
Gwei = uint256

class DepositRequest(Container):
    pubkey: BLSPubkey
    withdrawal_credentials: Bytes32
    amount: Gwei
    signature: BLSSignature
    index: uint64
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_aliases() {
        const SCHEMA: &str = r"
OMG = 3
Epoch = uint32
SomeVec = List[Epoch, 1337]

class Header(Container):
    slot: uint64
    epoch: Epoch
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_parent_aliases() {
        // I don't even know if we want to support this, but hey we do now!
        const SCHEMA: &str = r"
MagicStable = StableContainer[32]

class MagicFoo(MagicStable):
    foo: Optional[uint32]
    bar: Optional[uint64]
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_imports() {
        const SCHEMA_AS: &str = r"
import import_test as test
import ssz_external as external

TestA = test.A
TestB = test.B
TestC = test.C
TestD = external.D

VAL_A = 12
VAL_B = VAL_A
TEST_CONST = test.D

class Header(test.A):
    a: Union[null, test.B]
    b: test.B
    c: test.C
    d: uint8

f = List[test.A, TEST_CONST]
";

        const SCHEMA: &str = r"
import import_test
import ssz_external.module_a

TestA = import_test.A
TestB = import_test.B
TestC = import_test.C
TestD = module_a.D

VAL_A = 12
VAL_B = VAL_A
TEST_CONST = import_test.D

class Header(import_test.A):
    a: Union[null, import_test.B]
    b: import_test.B
    c: import_test.C
    d: uint8

f = List[import_test.A, TEST_CONST]
";

        let files = HashMap::from([(
            Path::new("tests/non_existent").to_path_buf(),
            SCHEMA_AS.to_string(),
        )]);
        let schema = parse_str_schema(&files, &["ssz_external"]).expect("test: parse schema");

        eprintln!("{schema:#?}");

        let files = HashMap::from([(
            Path::new("tests/non_existent").to_path_buf(),
            SCHEMA.to_string(),
        )]);
        let schema = parse_str_schema(&files, &["ssz_external"]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_external_module_pre_registration() {
        // Test that external modules are pre-registered and can be referenced
        // without UnknownImport errors, even when referenced early in schema conversion
        const SCHEMA: &str = r"
import external_crate

class ContainerWithExternal(Container):
    field: external_crate.SomeType
    list_field: List[external_crate.OtherType, 10]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        // This should not panic with UnknownImport
        let result = parse_str_schema(&files, &["external_crate"]);
        assert!(
            result.is_ok(),
            "External module should be pre-registered and resolvable"
        );
    }

    #[test]
    fn test_external_crate_path_construction() {
        // Test that external crate paths are constructed correctly
        // This verifies that external crates can be imported and referenced
        // without UnknownImport errors, which was the main bug fixed
        const SCHEMA: &str = r"
import external_crate
import external_crate.module_a.module_b as mod_b

TestA = external_crate.TypeA
TestB = mod_b.TypeB
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        // The main goal is that this doesn't fail with UnknownImport
        let result = parse_str_schema(&files, &["external_crate"]);
        assert!(
            result.is_ok(),
            "External crate paths should be constructed correctly and not cause UnknownImport"
        );
    }

    #[test]
    fn test_imported_generic_types_parse() {
        const SCHEMA: &str = r"
import external_crate

Foo = external_crate.Vector[uint8, 32]

class Example(Container):
    bytes: external_crate.List[uint8, 16]
    nested: external_crate.Vector[external_crate.List[uint8, 4], 2]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &["external_crate"]);
        assert!(
            result.is_ok(),
            "Imported generic types should parse successfully"
        );
    }

    #[test]
    fn test_schema_without_trailing_newline() {
        // Test that parsing works when the file doesn't end with a newline
        // This was causing an infinite loop in parse_class_body
        const SCHEMA: &str = "class Point2d(Container):\n  x_coord: uint32\n  y_coord: uint32";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Schema without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_assignment_without_trailing_newline() {
        // Test that parsing works when an assignment doesn't end with a newline
        const SCHEMA: &str = "MyAlias = uint32";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Assignment without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_import_without_trailing_newline() {
        // Test that parsing works when an import doesn't end with a newline
        const SCHEMA: &str = "import ssz_external";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &["ssz_external"]);
        assert!(
            result.is_ok(),
            "Import without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_complex_schema_without_trailing_newline() {
        // Test a more complex schema without trailing newline
        const SCHEMA: &str = r"
Epoch = uint32
SomeVec = List[Epoch, 1337]

class Header(Container):
    slot: uint64
    epoch: Epoch
    vec: SomeVec";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Complex schema without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_add_sub_operators() {
        // Test the add and subtract operators with both literal and symbolic operands
        const SCHEMA: &str = r"
BASE = 1024
PLUS_ONE = BASE + 1
MINUS_ONE = BASE - 1
LITERAL_ADD = 10 + 5
LITERAL_SUB = 10 - 5

class MyContainer(Container):
    field_a: List[byte, PLUS_ONE]
    field_b: List[byte, MINUS_ONE]
    field_c: List[byte, LITERAL_ADD]
    field_d: List[byte, LITERAL_SUB]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let (_, schema_map) =
            parse_str_schema(&files, &[]).expect("test: parse schema with add/sub operators");

        let schema = schema_map
            .get(Path::new("test.ssz"))
            .expect("test: get schema");

        // Verify we have all the constants
        let constants = schema.constants();
        assert_eq!(constants.len(), 5, "Should have 5 constants");

        // Check BASE constant
        assert_eq!(constants[0].name().0, "BASE");
        assert_eq!(constants[0].value().eval(), 1024);

        // Check PLUS_ONE constant (1024 + 1 = 1025)
        assert_eq!(constants[1].name().0, "PLUS_ONE");
        assert_eq!(constants[1].value().eval(), 1025);

        // Check MINUS_ONE constant (1024 - 1 = 1023)
        assert_eq!(constants[2].name().0, "MINUS_ONE");
        assert_eq!(constants[2].value().eval(), 1023);

        // Check LITERAL_ADD constant (10 + 5 = 15)
        assert_eq!(constants[3].name().0, "LITERAL_ADD");
        assert_eq!(constants[3].value().eval(), 15);

        // Check LITERAL_SUB constant (10 - 5 = 5)
        assert_eq!(constants[4].name().0, "LITERAL_SUB");
        assert_eq!(constants[4].value().eval(), 5);

        // Verify the class was created
        let classes = schema.classes();
        assert_eq!(classes.len(), 1, "Should have 1 class");
        assert_eq!(classes[0].name().0, "MyContainer");
        assert_eq!(classes[0].fields().len(), 4, "Should have 4 fields");

        eprintln!("Successfully tested add/sub operators with evaluated values: {schema:#?}");
    }

    #[test]
    fn test_issue_49_example() {
        // Test the actual example from issue #49
        const SCHEMA: &str = r"
### Maximum length of the predicate condition bytes
MAX_CONDITION_LEN = 1 << 10

### One additional byte for the PredicateTypeId
MAX_PREDICATE_LEN = MAX_CONDITION_LEN + 1

class Predicate(Container):
    condition: List[byte, MAX_CONDITION_LEN]
    full_data: List[byte, MAX_PREDICATE_LEN]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let (_, schema_map) =
            parse_str_schema(&files, &[]).expect("test: parse schema with issue #49 example");

        let schema = schema_map
            .get(Path::new("test.ssz"))
            .expect("test: get schema");

        // Verify constants
        let constants = schema.constants();
        assert_eq!(constants.len(), 2, "Should have 2 constants");

        // Check MAX_CONDITION_LEN constant (1 << 10 = 1024)
        assert_eq!(constants[0].name().0, "MAX_CONDITION_LEN");
        assert_eq!(constants[0].value().eval(), 1024);

        // Check MAX_PREDICATE_LEN constant (1024 + 1 = 1025)
        assert_eq!(constants[1].name().0, "MAX_PREDICATE_LEN");
        assert_eq!(constants[1].value().eval(), 1025);

        // Verify the class
        let classes = schema.classes();
        assert_eq!(classes.len(), 1, "Should have 1 class");
        assert_eq!(classes[0].name().0, "Predicate");
        assert_eq!(classes[0].fields().len(), 2, "Should have 2 fields");

        eprintln!("Successfully parsed issue #49 example: {schema:#?}");
    }

    #[test]
    fn test_diamond_dependency() {
        // Test case mimicking snark-acct-types scenario:
        // - state.ssz defines base types
        // - update.ssz imports state.ssz
        // - proof.ssz imports both state.ssz and update.ssz
        //
        // Without topological sorting, this could fail with UnknownImport if
        // update.ssz is processed before state.ssz.

        const STATE: &str = r"
### Base state type
class State(Container):
    value: uint64
    hash: Bytes32
";

        const UPDATE: &str = r"
import state

### Update references state
class Update(Container):
    prev: state.State
    next: state.State
    delta: uint32
";

        const PROOF: &str = r"
import state
import update

### Proof references both state and update
class Proof(Container):
    cur_state: state.State
    new_state: state.State
    upd: update.Update
";

        let files = HashMap::from([
            (Path::new("state.ssz").to_path_buf(), STATE.to_string()),
            (Path::new("update.ssz").to_path_buf(), UPDATE.to_string()),
            (Path::new("proof.ssz").to_path_buf(), PROOF.to_string()),
        ]);

        let (parsing_order, schema_map) =
            parse_str_schema(&files, &[]).expect("diamond dependency should parse successfully");

        // Verify all three modules were parsed
        assert_eq!(schema_map.len(), 3, "Should have 3 schemas");

        // Verify state.ssz was processed before update.ssz and proof.ssz
        let state_idx = parsing_order
            .iter()
            .position(|p| p.to_str() == Some("state.ssz"))
            .expect("state.ssz should be in parsing order");
        let update_idx = parsing_order
            .iter()
            .position(|p| p.to_str() == Some("update.ssz"))
            .expect("update.ssz should be in parsing order");
        let proof_idx = parsing_order
            .iter()
            .position(|p| p.to_str() == Some("proof.ssz"))
            .expect("proof.ssz should be in parsing order");

        assert!(
            state_idx < update_idx,
            "state.ssz should be processed before update.ssz"
        );
        assert!(
            state_idx < proof_idx,
            "state.ssz should be processed before proof.ssz"
        );
        assert!(
            update_idx < proof_idx,
            "update.ssz should be processed before proof.ssz"
        );

        // Verify the schemas have the expected classes
        let state_schema = schema_map.get(Path::new("state.ssz")).unwrap();
        assert_eq!(state_schema.classes().len(), 1);
        assert_eq!(state_schema.classes()[0].name().0, "State");

        let update_schema = schema_map.get(Path::new("update.ssz")).unwrap();
        assert_eq!(update_schema.classes().len(), 1);
        assert_eq!(update_schema.classes()[0].name().0, "Update");

        let proof_schema = schema_map.get(Path::new("proof.ssz")).unwrap();
        assert_eq!(proof_schema.classes().len(), 1);
        assert_eq!(proof_schema.classes()[0].name().0, "Proof");
    }

    #[test]
    fn test_multiple_modules_shared_imports() {
        // Test 3 consumer modules that each import a mix of shared and non-shared dependencies.
        // Modeled after snark-acct-types where:
        // - external_crate: external Rust crate (like strata_acct_types) imported by all
        // - primitives.ssz: internal module with simple types, imported by all
        // - unique_a.ssz: only imported by consumer_a
        // - unique_b.ssz: only imported by consumer_b
        // - unique_c.ssz: only imported by consumer_c
        // - consumer_a.ssz: imports external_crate + primitives + unique_a
        // - consumer_b.ssz: imports external_crate + primitives + unique_b
        // - consumer_c.ssz: imports external_crate + primitives + unique_c + consumer_a +
        //   consumer_b
        //
        // This extends the diamond pattern to 3 consumers with:
        // - One external crate imported by all (like strata_acct_types)
        // - One internal SSZ module imported by all (like primitives/types)
        // - Unique modules for each consumer
        // - Cross-consumer imports (consumer_c imports consumer_a and consumer_b)

        // Internal module with simple types - imported by all other modules
        const PRIMITIVES: &str = r"
### Simple primitive wrapper types
class Amount(Container):
    value: uint64

class Hash(Container):
    data: Bytes32
";

        const UNIQUE_A: &str = r"
import external_crate
import primitives

### Type unique to consumer A
class UniqueA(Container):
    value_a: uint32
    amount: primitives.Amount
    ext: external_crate.SomeType
";

        const UNIQUE_B: &str = r"
import external_crate
import primitives

### Type unique to consumer B
class UniqueB(Container):
    value_b: uint64
    hash: primitives.Hash
    ext: external_crate.SomeType
";

        const UNIQUE_C: &str = r"
import external_crate
import primitives

### Type unique to consumer C
class UniqueC(Container):
    value_c: Bytes32
    amount: primitives.Amount
    ext: external_crate.SomeType
";

        const CONSUMER_A: &str = r"
import external_crate
import primitives
import unique_a

### Consumer A imports external_crate + primitives + unique_a
class ConsumerA(Container):
    amount: primitives.Amount
    extra: unique_a.UniqueA
    ext: external_crate.SomeType
";

        const CONSUMER_B: &str = r"
import external_crate
import primitives
import unique_b

### Consumer B imports external_crate + primitives + unique_b
class ConsumerB(Container):
    hash: primitives.Hash
    extra: unique_b.UniqueB
    ext: external_crate.SomeType
";

        const CONSUMER_C: &str = r"
import external_crate
import primitives
import unique_c
import consumer_a
import consumer_b

### Consumer C imports external_crate + primitives + unique_c + both other consumers
class ConsumerC(Container):
    amount: primitives.Amount
    extra: unique_c.UniqueC
    ref_a: consumer_a.ConsumerA
    ref_b: consumer_b.ConsumerB
    ext: external_crate.SomeType
";

        let files = HashMap::from([
            (
                Path::new("primitives.ssz").to_path_buf(),
                PRIMITIVES.to_string(),
            ),
            (
                Path::new("unique_a.ssz").to_path_buf(),
                UNIQUE_A.to_string(),
            ),
            (
                Path::new("unique_b.ssz").to_path_buf(),
                UNIQUE_B.to_string(),
            ),
            (
                Path::new("unique_c.ssz").to_path_buf(),
                UNIQUE_C.to_string(),
            ),
            (
                Path::new("consumer_a.ssz").to_path_buf(),
                CONSUMER_A.to_string(),
            ),
            (
                Path::new("consumer_b.ssz").to_path_buf(),
                CONSUMER_B.to_string(),
            ),
            (
                Path::new("consumer_c.ssz").to_path_buf(),
                CONSUMER_C.to_string(),
            ),
        ]);

        // external_crate is registered as an external crate (like strata_acct_types)
        let (parsing_order, schema_map) = parse_str_schema(&files, &["external_crate"])
            .expect("multiple modules with shared imports should parse successfully");

        // Verify all 7 modules were parsed
        assert_eq!(schema_map.len(), 7, "Should have 7 schemas");

        // Helper to get index in parsing order
        let get_idx = |name: &str| {
            parsing_order
                .iter()
                .position(|p| p.to_str() == Some(name))
                .unwrap_or_else(|| panic!("{name} should be in parsing order"))
        };

        let primitives_idx = get_idx("primitives.ssz");
        let unique_a_idx = get_idx("unique_a.ssz");
        let unique_b_idx = get_idx("unique_b.ssz");
        let unique_c_idx = get_idx("unique_c.ssz");
        let consumer_a_idx = get_idx("consumer_a.ssz");
        let consumer_b_idx = get_idx("consumer_b.ssz");
        let consumer_c_idx = get_idx("consumer_c.ssz");

        // Verify dependency ordering
        // primitives must come before all modules that import it
        assert!(
            primitives_idx < unique_a_idx,
            "primitives.ssz should be processed before unique_a.ssz"
        );
        assert!(
            primitives_idx < unique_b_idx,
            "primitives.ssz should be processed before unique_b.ssz"
        );
        assert!(
            primitives_idx < unique_c_idx,
            "primitives.ssz should be processed before unique_c.ssz"
        );
        assert!(
            primitives_idx < consumer_a_idx,
            "primitives.ssz should be processed before consumer_a.ssz"
        );
        assert!(
            primitives_idx < consumer_b_idx,
            "primitives.ssz should be processed before consumer_b.ssz"
        );
        assert!(
            primitives_idx < consumer_c_idx,
            "primitives.ssz should be processed before consumer_c.ssz"
        );

        // unique modules must come before their consumers
        assert!(
            unique_a_idx < consumer_a_idx,
            "unique_a.ssz should be processed before consumer_a.ssz"
        );
        assert!(
            unique_b_idx < consumer_b_idx,
            "unique_b.ssz should be processed before consumer_b.ssz"
        );
        assert!(
            unique_c_idx < consumer_c_idx,
            "unique_c.ssz should be processed before consumer_c.ssz"
        );

        // consumer_a and consumer_b must come before consumer_c
        assert!(
            consumer_a_idx < consumer_c_idx,
            "consumer_a.ssz should be processed before consumer_c.ssz"
        );
        assert!(
            consumer_b_idx < consumer_c_idx,
            "consumer_b.ssz should be processed before consumer_c.ssz"
        );

        // Verify the schemas have the expected classes
        let primitives_schema = schema_map.get(Path::new("primitives.ssz")).unwrap();
        assert_eq!(primitives_schema.classes().len(), 2);
        assert_eq!(primitives_schema.classes()[0].name().0, "Amount");
        assert_eq!(primitives_schema.classes()[1].name().0, "Hash");

        assert_eq!(
            schema_map
                .get(Path::new("consumer_a.ssz"))
                .unwrap()
                .classes()[0]
                .name()
                .0,
            "ConsumerA"
        );
        assert_eq!(
            schema_map
                .get(Path::new("consumer_b.ssz"))
                .unwrap()
                .classes()[0]
                .name()
                .0,
            "ConsumerB"
        );
        assert_eq!(
            schema_map
                .get(Path::new("consumer_c.ssz"))
                .unwrap()
                .classes()[0]
                .name()
                .0,
            "ConsumerC"
        );
    }
}
