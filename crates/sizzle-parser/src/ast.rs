//! AST types

use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{
    Identifier, SrcPos, TaggedToktr,
    gobbler::Gobbler,
    token_tree::SrcToktr,
    tysys::{Binop, ConstValue},
};

pub(crate) type Modules = HashMap<PathBuf, Module>;

/// Manages buffering of comments to attach to AST structures
#[derive(Default)]
struct CommentBuffer {
    doc_comment: Option<String>,
    pragmas: Vec<String>,
}

impl CommentBuffer {
    fn new() -> Self {
        Self {
            doc_comment: None,
            pragmas: Vec::new(),
        }
    }

    fn set_doc_comment(&mut self, doc_comment: Option<String>) {
        self.doc_comment = doc_comment;
    }

    fn add_pragma(&mut self, pragma: String) {
        self.pragmas.push(pragma);
    }

    fn take_doc_comment(&mut self) -> Option<String> {
        self.doc_comment.take()
    }

    fn take_pragmas(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pragmas)
    }

    fn clear(&mut self) {
        self.doc_comment = None;
        self.pragmas.clear();
    }
}

/// A module file containing a list of definitions.
#[derive(Clone, Debug)]
pub(crate) enum Module {
    External,
    Internal(Vec<ModuleEntry>),
}

impl Module {
    pub(crate) fn new_external() -> Self {
        Self::External
    }

    pub(crate) fn new_internal(entry: Vec<ModuleEntry>) -> Self {
        Self::Internal(entry)
    }

    pub(crate) fn is_external(&self) -> bool {
        matches!(self, Self::External)
    }

    pub(crate) fn entries(&self) -> &[ModuleEntry] {
        match self {
            Self::External => &[],
            Self::Internal(entries) => entries,
        }
    }

    pub(crate) fn mut_entries(&mut self) -> &mut Vec<ModuleEntry> {
        match self {
            Self::External => panic!("external module has no entries"),
            Self::Internal(entries) => entries,
        }
    }
}

/// A definition within a module, in the order it was listed.
#[derive(Clone, Debug)]
pub(crate) enum ModuleEntry {
    /// An assignment with some name being assigned to a value.  This could be
    /// declaring a const or declaring a type alias.
    Assignment(AssignEntry),

    /// A class data structure.
    Class(ClassDefEntry),
}

impl ModuleEntry {
    pub(crate) fn name(&self) -> &Identifier {
        match self {
            ModuleEntry::Assignment(d) => d.name(),
            ModuleEntry::Class(d) => d.name(),
        }
    }
}

/// A const definition.
#[derive(Clone, Debug)]
pub(crate) struct AssignEntry {
    name: Identifier,
    value: AssignExpr,
}

impl AssignEntry {
    pub(crate) fn new(name: Identifier, value: AssignExpr) -> Self {
        Self { name, value }
    }

    pub(crate) fn name(&self) -> &Identifier {
        &self.name
    }

    pub(crate) fn value(&self) -> &AssignExpr {
        &self.value
    }
}

/// An expression that we can assign to a name.
#[derive(Clone, Debug)]
pub(crate) enum AssignExpr {
    /// A name.
    ///
    /// This could be another const name or a type expression.
    Name(Identifier),

    /// An imported name.
    ///
    /// This could be another const name or a type expression.
    Imported(ImportedTySpec),

    /// An imported name with type arguments.
    ImportedComplex(ImportedComplexTySpec),

    /// A type expression.
    Complex(ComplexTySpec),

    /// An integer literal.
    Value(ConstValue),

    /// A binary operation with a named operand (e.g., MAX_LEN + 1)
    SymbolicBinop(Binop, Identifier, u64),
}

/// A class definition.
///
/// Classes must always have parent types.
#[derive(Clone, Debug)]
pub(crate) struct ClassDefEntry {
    name: Identifier,
    parent_ty: TyExprSpec,
    doc: Option<String>,
    doc_comment: Option<String>,
    pragmas: Vec<String>,
    fields: Vec<FieldDef>,
}

impl ClassDefEntry {
    pub(crate) fn new(
        name: Identifier,
        parent_ty: TyExprSpec,
        doc: Option<String>,
        fields: Vec<FieldDef>,
    ) -> Self {
        Self {
            name,
            parent_ty,
            doc,
            doc_comment: None,
            pragmas: Vec::new(),
            fields,
        }
    }

    pub(crate) fn name(&self) -> &Identifier {
        &self.name
    }

    pub(crate) fn parent_ty(&self) -> &TyExprSpec {
        &self.parent_ty
    }

    pub(crate) fn doc(&self) -> Option<&str> {
        self.doc.as_ref().map(|s| s.as_ref())
    }

    pub(crate) fn doc_comment(&self) -> Option<&str> {
        self.doc_comment.as_ref().map(|s| s.as_ref())
    }

    pub(crate) fn pragmas(&self) -> &[String] {
        &self.pragmas
    }

    pub(crate) fn set_doc_comment(&mut self, doc_comment: Option<String>) {
        self.doc_comment = doc_comment;
    }

    pub(crate) fn set_pragmas(&mut self, pragmas: Vec<String>) {
        self.pragmas = pragmas;
    }

    pub(crate) fn fields(&self) -> &[FieldDef] {
        &self.fields
    }
}

/// A field definition within a class.
#[derive(Clone, Debug)]
pub(crate) struct FieldDef {
    name: Identifier,
    ty: TyExprSpec,
    doc_comment: Option<String>,
    pragmas: Vec<String>,
}

impl FieldDef {
    pub(crate) fn new(name: Identifier, ty: TyExprSpec) -> Self {
        Self {
            name,
            ty,
            doc_comment: None,
            pragmas: Vec::new(),
        }
    }

    pub(crate) fn name(&self) -> &Identifier {
        &self.name
    }

    pub(crate) fn ty(&self) -> &TyExprSpec {
        &self.ty
    }

    pub(crate) fn doc_comment(&self) -> Option<&str> {
        self.doc_comment.as_ref().map(|s| s.as_ref())
    }

    pub(crate) fn pragmas(&self) -> &[String] {
        &self.pragmas
    }

    pub(crate) fn set_doc_comment(&mut self, doc_comment: Option<String>) {
        self.doc_comment = doc_comment;
    }

    pub(crate) fn add_pragma(&mut self, pragma: String) {
        self.pragmas.push(pragma);
    }
}

/// A type specification.
///
/// This needs to be further resolved to figure out ambiguous identifiers.
#[derive(Clone, Debug)]
pub(crate) enum TyExprSpec {
    /// This is an imported type.
    Imported(ImportedTySpec),

    /// This is an imported type with arguments.
    ImportedComplex(ImportedComplexTySpec),

    /// This is just a single identifier.  It could refer to a type or a const.
    Simple(Identifier),

    /// This is an identifier and type parameters.  It probably refers to a
    /// type, but we need to sanity check all of this.
    Complex(ComplexTySpec),

    /// None type for unit variants in Unions
    None,
}

impl TyExprSpec {
    pub(crate) fn base_name(&self) -> &Identifier {
        match self {
            TyExprSpec::Simple(name) => name,
            TyExprSpec::Complex(spec) => &spec.base_name,
            TyExprSpec::Imported(spec) => &spec.base_name,
            TyExprSpec::ImportedComplex(spec) => spec.base_name(),
            TyExprSpec::None => panic!("None type has no base name"),
        }
    }
}

/// An imported type.
///
/// No verification is done at this stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportedTySpec {
    module_path: PathBuf,
    module_name: Identifier,
    base_name: Identifier,
}

impl ImportedTySpec {
    pub fn new(module_path: PathBuf, base_name: Identifier) -> Self {
        let module_name = module_path
            .to_str()
            .unwrap()
            .split(std::path::MAIN_SEPARATOR_STR)
            .last()
            .unwrap()
            .to_string();
        Self {
            module_path,
            module_name: Identifier(module_name),
            base_name,
        }
    }

    pub fn module_path(&self) -> &PathBuf {
        &self.module_path
    }

    pub fn module_name(&self) -> &Identifier {
        &self.module_name
    }

    pub fn base_name(&self) -> &Identifier {
        &self.base_name
    }

    pub fn full_name(&self) -> Identifier {
        Identifier(
            self.module_path
                .to_str()
                .unwrap()
                .replace(std::path::MAIN_SEPARATOR, ".")
                + "."
                + self.base_name.0.as_str(),
        )
    }
}

/// An imported type with generic arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportedComplexTySpec {
    imported: ImportedTySpec,
    args: Vec<TyArgSpec>,
}

impl ImportedComplexTySpec {
    pub fn new(module_path: PathBuf, base_name: Identifier, args: Vec<TyArgSpec>) -> Self {
        Self {
            imported: ImportedTySpec::new(module_path, base_name),
            args,
        }
    }

    pub fn module_path(&self) -> &PathBuf {
        self.imported.module_path()
    }

    pub fn module_name(&self) -> &Identifier {
        self.imported.module_name()
    }

    pub fn base_name(&self) -> &Identifier {
        self.imported.base_name()
    }

    pub fn full_name(&self) -> Identifier {
        self.imported.full_name()
    }

    pub fn args(&self) -> &[TyArgSpec] {
        &self.args
    }
}

/// An instantiated generic type.
///
/// At this stage we have not verified that `base_name` is actually a type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexTySpec {
    base_name: Identifier,
    args: Vec<TyArgSpec>,
}

impl ComplexTySpec {
    pub fn new(base_name: Identifier, args: Vec<TyArgSpec>) -> Self {
        Self { base_name, args }
    }

    pub fn base_name(&self) -> &Identifier {
        &self.base_name
    }

    pub fn args(&self) -> &[TyArgSpec] {
        &self.args
    }
}

/// Type arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TyArgSpec {
    /// An imported type.
    Imported(ImportedTySpec),

    /// An imported type with arguments.
    ImportedComplex(ImportedComplexTySpec),

    /// An identifier, which could be a constant or a type name.
    Ident(Identifier),

    /// An identifier (presumably a type) with a list of arguments.
    Complex(ComplexTySpec),

    /// An literal integer.
    IntLiteral(u64),

    /// None type for Unions
    None,
}

/// Errors that can occur during AST parsing.
#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParseError {
    /// A malformed block structure was encountered at the given source position.
    #[error("malformed def at {0}")]
    MalformedBlock(SrcPos),

    /// An unexpected token was encountered at the given source position.
    #[error("unexpected token at {0}")]
    UnexpectedToken(SrcPos),

    /// Multiple docstrings were found in a single definition (only one docstring is allowed per
    /// class).
    #[error("multiple docstrings in def")]
    MultipleDocStrings,

    /// A standalone docstring was encountered at the module level (docstrings must be inside class
    /// bodies).
    #[error("standalone docstring at {0}")]
    StandaloneDocstring(SrcPos),

    /// A standalone doc comment was encountered at the module level (doc comments must precede a
    /// class, assignment, or import).
    #[error("standalone doc comment")]
    StandaloneDocComment,

    /// The parser reached the end of input unexpectedly.
    #[error("unexpected end of input")]
    UnexpectedEnd,

    /// A feature that is not yet implemented was encountered.
    #[error("not yet implemented")]
    Unimplemented,

    /// An unhandled error occurred with the given message.
    #[error("unhandled other error '{0}'")]
    Other(String),
}

/// Manager struct for importing modules
#[derive(Debug)]
pub(crate) struct ModuleManager {
    /// The modules that have been imported.
    modules: Modules,
    /// The order of the modules that have been imported.
    import_order: Vec<PathBuf>,
    /// External modules that can be imported.
    external_modules: Vec<String>,
}

impl ModuleManager {
    /// Creates a new module manager for the given path.
    pub(crate) fn new(external_modules: &[&str]) -> Self {
        Self {
            modules: Modules::new(),
            import_order: Vec::new(),
            external_modules: external_modules.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Adds a module to the front of the import order.
    pub(crate) fn add_module_to_front<P: AsRef<Path>>(&mut self, path: P) -> bool {
        if self.modules.contains_key(path.as_ref()) {
            return false;
        }
        let path = path.as_ref().to_path_buf();
        self.modules
            .insert(path.clone(), Module::new_internal(Vec::new()));
        self.import_order.insert(0, path);
        true
    }

    /// Adds a module to the manager.
    pub(crate) fn add_module<P: AsRef<Path>>(&mut self, path: P, is_external: bool) -> bool {
        if self.modules.contains_key(path.as_ref()) {
            return false;
        }
        let path = path.as_ref().to_path_buf();
        let module = if is_external {
            Module::new_external()
        } else {
            Module::new_internal(Vec::new())
        };
        self.modules.insert(path.clone(), module);
        self.import_order.push(path);
        true
    }

    /// Consumes the manager and returns all modules.
    pub(crate) fn into_modules(self) -> Modules {
        self.modules
    }

    /// Returns a mutable module by path.
    pub(crate) fn get_module_mut<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut Module> {
        self.modules.get_mut(path.as_ref())
    }
}

/// Parses a module from a sequence of tokens.
pub(crate) fn parse_module_from_toktrs<P: AsRef<Path>>(
    toktrs: &[SrcToktr],
    path: P,
    module_manager: &mut ModuleManager,
    entry_point_files: Option<&HashMap<PathBuf, String>>,
) -> Result<(), ParseError> {
    let path = path.as_ref();
    let mut gob = Gobbler::new(toktrs);
    let mut import_map = HashMap::new();

    let mut comment_buffer = CommentBuffer::new();
    let mut consecutive_newlines = 0;

    while let Some(cur) = gob.get() {
        match cur {
            // Discard newlines.
            TaggedToktr::Newline(_) => {
                consecutive_newlines += 1;
                gob.gobble_one();
                // Clear comment buffer if there are multiple blank lines
                // (comments should be right before what they document, not separated by blank
                // lines)
                // If we have a doc comment or pragma and hit multiple newlines, it's standalone
                if consecutive_newlines > 1
                    && (comment_buffer.take_doc_comment().is_some()
                        || !comment_buffer.take_pragmas().is_empty())
                {
                    return Err(ParseError::StandaloneDocComment);
                }
            }

            // Discard regular comments
            TaggedToktr::Comment(_, _) => {
                consecutive_newlines = 0;
                gob.gobble_one();
            }

            // Collect doc comments and pragmas
            TaggedToktr::DocComment(_, text) => {
                consecutive_newlines = 0;
                comment_buffer.set_doc_comment(Some(text.clone()));
                gob.gobble_one();
            }

            TaggedToktr::PragmaComment(_, text) => {
                consecutive_newlines = 0;
                comment_buffer.add_pragma(text.clone());
                gob.gobble_one();
            }

            // Lines that start with "import" are imports.
            TaggedToktr::Import(_) => {
                consecutive_newlines = 0;
                comment_buffer.clear(); // Clear comments before imports
                parse_import(
                    &mut gob,
                    path,
                    module_manager,
                    &mut import_map,
                    entry_point_files,
                )?;
            }

            // Lines that start with identifiers are probably assignments.
            TaggedToktr::Identifier(_, _) => {
                consecutive_newlines = 0;
                comment_buffer.clear(); // Clear comments before assignments
                let cd = parse_assignment(&mut gob, &import_map)?;
                module_manager
                    .get_module_mut(path)
                    .unwrap()
                    .mut_entries()
                    .push(ModuleEntry::Assignment(cd));
            }

            // Lines that start with "class" are always classes.
            TaggedToktr::Class(_) => {
                consecutive_newlines = 0;
                // Comments should have been collected into comment_buffer
                let mut cd = parse_class(&mut gob, &import_map)?;
                // Attach collected comments to the class
                if let Some(doc) = comment_buffer.take_doc_comment() {
                    cd.set_doc_comment(Some(doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                if !pragmas.is_empty() {
                    cd.set_pragmas(pragmas);
                }
                module_manager
                    .get_module_mut(path)
                    .unwrap()
                    .mut_entries()
                    .push(ModuleEntry::Class(cd));
            }

            // Standalone docstrings are not allowed at module level
            TaggedToktr::DocString(sp, _) => {
                return Err(ParseError::StandaloneDocstring(*sp));
            }

            t => {
                return Err(ParseError::UnexpectedToken(*t.tag()));
            }
        }
    }

    // Check if there are any standalone doc comments or pragmas left in the buffer
    // (not attached to any class, assignment, or import)
    if comment_buffer.take_doc_comment().is_some() || !comment_buffer.take_pragmas().is_empty() {
        // If we have doc comments or pragmas but no following element, it's an error
        // We need to find the source position - we'll use the last token's position
        // Since we've consumed all tokens, we'll report an error at the end
        // For now, we'll use a dummy position - in practice this shouldn't happen
        // as the comment buffer should be cleared when encountering newlines before non-class
        // elements
        return Err(ParseError::StandaloneDocComment);
    }

    Ok(())
}

/// Parses a const definition out of the gobbler.
fn parse_assignment(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<AssignEntry, ParseError> {
    use TaggedToktr::*;

    let ident = match gob.gobble_slice_up_to_or_end(is_toktr_eq) {
        [Identifier(_, ident)] => ident.clone(),
        _ => return Err(ParseError::UnexpectedEnd),
    };

    // Skip over the `=`.
    let eq = gob.get();
    assert!(
        matches!(eq, Some(TaggedToktr::Eq(_))),
        "ast: missing expected eq"
    );
    gob.gobble_one();

    let expr_slice = gob.gobble_slice_up_to_or_end(is_toktr_newline);
    let val = parse_assign_expr(expr_slice, import_map)?;

    Ok(AssignEntry::new(ident, val))
}

/// Parses a slice of tokens as an expression.
///
/// Since we don't support arbitrary expressions, this can be actually pretty simple!
fn parse_assign_expr(
    toktrs: &[SrcToktr],
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<AssignExpr, ParseError> {
    let expr = match toktrs {
        // This is probably an alias.
        [TaggedToktr::Identifier(_, name)] => AssignExpr::Name(name.clone()),

        [
            TaggedToktr::Identifier(_, name),
            TaggedToktr::BracketBlock(_, arg_toks),
        ] => {
            let mut gob = Gobbler::new(arg_toks.children());
            let args = parse_ty_args(&mut gob, import_map)?;
            AssignExpr::Complex(ComplexTySpec::new(name.clone(), args))
        }

        [
            TaggedToktr::Identifier(_, module_name),
            TaggedToktr::Dot(_),
            TaggedToktr::Identifier(_, ident),
            TaggedToktr::BracketBlock(_, arg_toks),
        ] => {
            let module_path = import_map.get(module_name).unwrap();
            let mut gob = Gobbler::new(arg_toks.children());
            let args = parse_ty_args(&mut gob, import_map)?;
            AssignExpr::ImportedComplex(ImportedComplexTySpec::new(
                module_path.clone(),
                ident.clone(),
                args,
            ))
        }

        // Simple integer expression.
        [TaggedToktr::IntegerLiteral(_, v)] => AssignExpr::Value(ConstValue::Int(*v)),

        // This is a shl value.
        [
            TaggedToktr::IntegerLiteral(_, v),
            TaggedToktr::Shl(_),
            TaggedToktr::IntegerLiteral(_, shl_v),
        ] => AssignExpr::Value(ConstValue::Binop(Binop::Shl, *v, *shl_v)),

        [
            TaggedToktr::IntegerLiteral(_, mul_l),
            TaggedToktr::Mul(_),
            TaggedToktr::IntegerLiteral(_, mul_r),
        ] => AssignExpr::Value(ConstValue::Binop(Binop::Mul, *mul_l, *mul_r)),

        // Addition: Integer + Integer
        [
            TaggedToktr::IntegerLiteral(_, add_l),
            TaggedToktr::Add(_),
            TaggedToktr::IntegerLiteral(_, add_r),
        ] => AssignExpr::Value(ConstValue::Binop(Binop::Add, *add_l, *add_r)),

        // Subtraction: Integer - Integer
        [
            TaggedToktr::IntegerLiteral(_, sub_l),
            TaggedToktr::Sub(_),
            TaggedToktr::IntegerLiteral(_, sub_r),
        ] => AssignExpr::Value(ConstValue::Binop(Binop::Sub, *sub_l, *sub_r)),

        // Addition: Identifier + Integer (e.g., MAX_LEN + 1)
        [
            TaggedToktr::Identifier(_, name),
            TaggedToktr::Add(_),
            TaggedToktr::IntegerLiteral(_, v),
        ] => AssignExpr::SymbolicBinop(Binop::Add, name.clone(), *v),

        // Subtraction: Identifier - Integer (e.g., MAX_LEN - 1)
        [
            TaggedToktr::Identifier(_, name),
            TaggedToktr::Sub(_),
            TaggedToktr::IntegerLiteral(_, v),
        ] => AssignExpr::SymbolicBinop(Binop::Sub, name.clone(), *v),

        [
            TaggedToktr::Identifier(_, module_name),
            TaggedToktr::Dot(_),
            TaggedToktr::Identifier(_, ident),
        ] => {
            let module_path = import_map.get(module_name).unwrap();
            AssignExpr::Imported(ImportedTySpec::new(module_path.clone(), ident.clone()))
        }

        _ => return Err(ParseError::UnexpectedEnd),
    };

    Ok(expr)
}

/// Parses a class definition out of a gobbler.
fn parse_class(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<ClassDefEntry, ParseError> {
    use TaggedToktr::*;

    let sp = *gob.get_expect().tag();

    // Collect comments before the class definition
    let mut comment_buffer = CommentBuffer::new();

    // Look backwards for doc comments and pragmas (but we're parsing forward, so we'll handle this
    // differently) For now, we'll collect comments from the current position backwards
    // conceptually but in practice, comments should be collected before we reach this function
    // Let's collect them after the class keyword is found
    match gob.view() {
        [
            Class(_),
            Identifier(_, name),
            ParenBlock(_, ty_data),
            Colon(_),
            ..,
        ] => {
            // Parse basic information out of the header.
            let name = name.clone();
            let mut ty_gob = Gobbler::new(ty_data.children());
            let parent_ty = parse_ty(&mut ty_gob, import_map)?;

            // Then extract the body and parse it.
            gob.gobble_until(is_toktr_newline);
            gob.gobble_until(is_toktr_not_newline);

            let body = match gob.get() {
                Some(IndentBlock(_, d)) => {
                    let mut body_gob = Gobbler::new(d.children());
                    let body = parse_class_body(&mut body_gob, import_map)?;
                    gob.gobble_one();
                    body
                }
                Some(t) => return Err(ParseError::UnexpectedToken(*t.tag())),
                Option::None => {
                    // Empty body - only allowed for Union types (unit variants)
                    let is_union =
                        matches!(parent_ty, TyExprSpec::Simple(ref id) if id.0.as_str() == "Union");
                    if is_union {
                        ClassBody {
                            doc: None,
                            fields: Vec::new(),
                        }
                    } else {
                        return Err(ParseError::UnexpectedEnd);
                    }
                }
            };

            let mut cd = ClassDefEntry::new(name, parent_ty, body.doc, body.fields);
            // Attach comments if any were collected
            if let Some(doc) = comment_buffer.take_doc_comment() {
                cd.set_doc_comment(Some(doc));
            }
            let pragmas = comment_buffer.take_pragmas();
            if !pragmas.is_empty() {
                cd.set_pragmas(pragmas);
            }
            Ok(cd)
        }

        _ => Err(ParseError::MalformedBlock(sp)),
    }
}

/// Parses a type specification out of a gobbler.
fn parse_ty(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<TyExprSpec, ParseError> {
    let first_tok = gob.get().ok_or(ParseError::UnexpectedEnd)?;
    let TaggedToktr::Identifier(_, first_ident) = first_tok else {
        return Err(ParseError::UnexpectedToken(*first_tok.tag()));
    };

    let first_ident = first_ident.clone();
    gob.gobble_one();

    let ty = match gob.get() {
        Some(TaggedToktr::BracketBlock(_, data)) => {
            let mut sub_gob = Gobbler::new(data.children());
            let args = parse_ty_args(&mut sub_gob, import_map)?;
            gob.gobble_one();
            TyExprSpec::Complex(ComplexTySpec::new(first_ident, args))
        }
        Some(TaggedToktr::Dot(_)) => {
            gob.gobble_one();
            let second_tok = gob.get().ok_or(ParseError::UnexpectedEnd)?;
            let TaggedToktr::Identifier(_, second_ident) = second_tok else {
                return Err(ParseError::UnexpectedToken(*second_tok.tag()));
            };

            let second_ident = second_ident.clone();
            gob.gobble_one();

            let module_path = import_map.get(&first_ident).unwrap();
            match gob.get() {
                Some(TaggedToktr::BracketBlock(_, data)) => {
                    let mut args_gob = Gobbler::new(data.children());
                    let args = parse_ty_args(&mut args_gob, import_map)?;
                    gob.gobble_one();
                    TyExprSpec::ImportedComplex(ImportedComplexTySpec::new(
                        module_path.clone(),
                        second_ident,
                        args,
                    ))
                }
                _ => TyExprSpec::Imported(ImportedTySpec::new(module_path.clone(), second_ident)),
            }
        }

        Some(t) => {
            return Err(ParseError::UnexpectedToken(*t.tag()));
        }

        None => TyExprSpec::Simple(first_ident),
    };

    Ok(ty)
}

/// Parses type args out of a gobbler.  The gobbler must be exactly the sequence
/// of type arguments, with commas separating arguments.
fn parse_ty_args(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<Vec<TyArgSpec>, ParseError> {
    let mut args = Vec::new();

    while gob.has_entry() {
        // Parsing the arg is easy.
        args.push(parse_ty_arg(gob, import_map)?);

        // Try to consume a comma.
        let Some(next) = gob.get() else {
            break;
        };

        match next {
            TaggedToktr::Comma(_) => {
                gob.gobble_one();
                continue;
            }

            _t => {
                return Err(ParseError::UnexpectedToken(*next.tag()));
            }
        }
    }

    Ok(args)
}

fn parse_ty_arg(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<TyArgSpec, ParseError> {
    match gob.get() {
        Some(TaggedToktr::Null(_)) => {
            gob.gobble_one();
            Ok(TyArgSpec::None)
        }

        // An identifier could be a type or a const, we'll resolve that later.
        Some(TaggedToktr::Identifier(_, first_ident)) => {
            let first_ident = first_ident.clone();
            gob.gobble_one();
            match gob.get() {
                // Have generic arguments we also want to consume.
                Some(TaggedToktr::BracketBlock(_, data)) => {
                    let mut args_gob = Gobbler::new(data.children());
                    let args = parse_ty_args(&mut args_gob, import_map)?;
                    gob.gobble_one();
                    Ok(TyArgSpec::Complex(ComplexTySpec::new(first_ident, args)))
                }

                Some(TaggedToktr::Dot(_)) => {
                    gob.gobble_one();
                    let second_tok = gob.get().ok_or(ParseError::UnexpectedEnd)?;
                    let TaggedToktr::Identifier(_, second_ident) = second_tok else {
                        return Err(ParseError::UnexpectedToken(*second_tok.tag()));
                    };

                    let second_ident = second_ident.clone();
                    gob.gobble_one();

                    let module_path = import_map.get(&first_ident).unwrap();
                    match gob.get() {
                        Some(TaggedToktr::BracketBlock(_, data)) => {
                            let mut args_gob = Gobbler::new(data.children());
                            let args = parse_ty_args(&mut args_gob, import_map)?;
                            gob.gobble_one();
                            Ok(TyArgSpec::ImportedComplex(ImportedComplexTySpec::new(
                                module_path.clone(),
                                second_ident,
                                args,
                            )))
                        }
                        _ => Ok(TyArgSpec::Imported(ImportedTySpec::new(
                            module_path.clone(),
                            second_ident,
                        ))),
                    }
                }

                // This would be the next item.
                Some(TaggedToktr::Comma(_)) => Ok(TyArgSpec::Ident(first_ident)),

                // Other cases, we shouldn't have these.
                Some(t) => Err(ParseError::UnexpectedToken(*t.tag())),

                // At the end of the block.
                None => Ok(TyArgSpec::Ident(first_ident)),
            }
        }

        // A literal integer, which is like a const.
        Some(TaggedToktr::IntegerLiteral(_, v)) => {
            let v = *v;
            gob.gobble_one();
            Ok(TyArgSpec::IntLiteral(v))
        }

        // Other cases.
        Some(t) => Err(ParseError::UnexpectedToken(*t.tag())),
        None => Err(ParseError::UnexpectedEnd),
    }
}

struct ClassBody {
    doc: Option<String>,
    fields: Vec<FieldDef>,
}

/// Parses a class body out of a gobbler.  The entries must be the full gobbler.
fn parse_class_body(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<ClassBody, ParseError> {
    use TaggedToktr::*;

    let mut doc = None;
    let mut fields = Vec::new();
    let mut comment_buffer = CommentBuffer::new();

    while gob.has_entry() {
        // Collect comments before the field definition
        match gob.get() {
            Some(DocComment(_, text)) => {
                comment_buffer.set_doc_comment(Some(text.clone()));
                gob.gobble_one();
                continue;
            }
            Some(PragmaComment(_, text)) => {
                comment_buffer.add_pragma(text.clone());
                gob.gobble_one();
                continue;
            }
            Some(Comment(_, _)) => {
                // Regular comments are discarded (per documentation)
                gob.gobble_one();
                continue;
            }
            Some(Newline(_)) => {
                gob.gobble_one();
                continue;
            }
            _ => {}
        }

        match gob.gobble_slice_up_to_or_end(is_toktr_newline) {
            [DocString(_, d)] => {
                if doc.is_some() {
                    return Err(ParseError::MultipleDocStrings);
                }
                doc = Some(d.clone());
            }

            [
                Identifier(_, fname),
                Colon(_),
                Identifier(_, tyname),
                BracketBlock(_, tyarg_data),
            ] => {
                let mut arg_gob = Gobbler::new(tyarg_data.children());
                let ty_args = parse_ty_args(&mut arg_gob, import_map)?;
                let ty = TyExprSpec::Complex(ComplexTySpec::new(tyname.clone(), ty_args));
                let mut field = FieldDef::new(fname.clone(), ty);
                // Attach comments to the field
                if let Some(field_doc) = comment_buffer.take_doc_comment() {
                    field.set_doc_comment(Some(field_doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                for pragma in pragmas {
                    field.add_pragma(pragma);
                }
                fields.push(field);
                comment_buffer.clear();
            }

            [
                Identifier(_, fname),
                Colon(_),
                Identifier(_, first_ident),
                Dot(_),
                Identifier(_, second_ident),
                BracketBlock(_, tyarg_data),
            ] => {
                let module_path = import_map.get(first_ident).unwrap();
                let mut arg_gob = Gobbler::new(tyarg_data.children());
                let ty_args = parse_ty_args(&mut arg_gob, import_map)?;
                let ty = TyExprSpec::ImportedComplex(ImportedComplexTySpec::new(
                    module_path.clone(),
                    second_ident.clone(),
                    ty_args,
                ));
                let mut field = FieldDef::new(fname.clone(), ty);
                // Attach comments to the field
                if let Some(field_doc) = comment_buffer.take_doc_comment() {
                    field.set_doc_comment(Some(field_doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                for pragma in pragmas {
                    field.add_pragma(pragma);
                }
                fields.push(field);
                comment_buffer.clear();
            }

            [
                Identifier(_, fname),
                Colon(_),
                Identifier(_, first_ident),
                Dot(_),
                Identifier(_, second_ident),
            ] => {
                let module_path = import_map.get(first_ident).unwrap();
                let ty = TyExprSpec::Imported(ImportedTySpec::new(
                    module_path.clone(),
                    second_ident.clone(),
                ));
                let mut field = FieldDef::new(fname.clone(), ty);
                // Attach comments to the field
                if let Some(field_doc) = comment_buffer.take_doc_comment() {
                    field.set_doc_comment(Some(field_doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                for pragma in pragmas {
                    field.add_pragma(pragma);
                }
                fields.push(field);
                comment_buffer.clear();
            }

            [Identifier(_, fname), Colon(_), Identifier(_, tyname)] => {
                let ty = TyExprSpec::Simple(tyname.clone());
                let mut field = FieldDef::new(fname.clone(), ty);
                // Attach comments to the field
                if let Some(field_doc) = comment_buffer.take_doc_comment() {
                    field.set_doc_comment(Some(field_doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                for pragma in pragmas {
                    field.add_pragma(pragma);
                }
                fields.push(field);
                comment_buffer.clear();
            }

            [Identifier(_, fname)] => {
                // Unit variant in Union (no associated data - uses None type)
                let ty = TyExprSpec::None;
                let mut field = FieldDef::new(fname.clone(), ty);
                // Attach comments to the field
                if let Some(field_doc) = comment_buffer.take_doc_comment() {
                    field.set_doc_comment(Some(field_doc));
                }
                let pragmas = comment_buffer.take_pragmas();
                for pragma in pragmas {
                    field.add_pragma(pragma);
                }
                fields.push(field);
                comment_buffer.clear();
            }

            [t, ..] => return Err(ParseError::UnexpectedToken(*t.tag())),

            [] => {
                // Empty slice means we hit a sequence of newlines or reached the end
                // Just continue to the next iteration
                comment_buffer.clear();
                continue;
            }
        }

        // Ignore extra newlines.
        gob.gobble_until(is_toktr_not_newline);
    }

    Ok(ClassBody { doc, fields })
}

/// Parses import statements by reading the imported module and parsing it
fn parse_import<P: AsRef<Path>>(
    gob: &mut Gobbler<'_, SrcToktr>,
    path: P,
    module_manager: &mut ModuleManager,
    import_map: &mut HashMap<Identifier, PathBuf>,
    entry_point_files: Option<&HashMap<PathBuf, String>>,
) -> Result<(), ParseError> {
    use TaggedToktr::*;

    let path = path.as_ref();
    let sp = *gob.get_expect().tag();

    match gob.view() {
        [Import(_), ..] => {
            gob.gobble_one();
            let path_tokens = gob.gobble_slice_up_to_or_end(is_toktr_newline);

            let mut path = path
                .parent()
                .expect("import: path must have a parent")
                .to_path_buf();
            let import_alias;

            // Parse the path of import module
            let mut path_gob = Gobbler::new(path_tokens);
            let mut is_first_tok = true;
            let mut is_external = false;
            loop {
                match path_gob.view() {
                    [Identifier(_, name), Dot(_), ..] => {
                        if is_first_tok {
                            is_external = module_manager.external_modules.contains(&name.0);
                            if is_external {
                                path = PathBuf::from(&name.0);
                            } else {
                                path = path.join(&name.0);
                            }
                        } else {
                            path = path.join(&name.0);
                        }
                        path_gob.gobble_exact(2);
                    }
                    [Dot(_), Dot(_), ..] => {
                        path = path
                            .parent()
                            .expect("import: path must have a parent")
                            .to_path_buf();
                        path_gob.gobble_exact(2);
                    }
                    [Identifier(_, name), As(_), Identifier(_, alias)] => {
                        if is_first_tok {
                            is_external = module_manager.external_modules.contains(&name.0);
                            if is_external {
                                path = PathBuf::from(&name.0);
                            } else {
                                path = path.join(&name.0);
                            }
                        } else {
                            path = path.join(&name.0);
                        }
                        import_alias = alias.clone();
                        path_gob.gobble_exact(3);
                        break;
                    }
                    [Identifier(_, name)] => {
                        if is_first_tok {
                            is_external = module_manager.external_modules.contains(&name.0);
                            if is_external {
                                path = PathBuf::from(&name.0);
                            } else {
                                path = path.join(&name.0);
                            }
                        } else {
                            path = path.join(&name.0);
                        }
                        import_alias = name.clone();
                        path_gob.gobble_one();
                        break;
                    }
                    [t, ..] => return Err(ParseError::UnexpectedToken(*t.tag())),
                    _ => return Err(ParseError::UnexpectedEnd),
                }
                is_first_tok = false;
            }

            // Check if this is an entry point first, then check filesystem
            let ssz_path = path.with_extension("ssz");
            // Normalize path for comparison (remove extension, as entry points are stored without
            // .ssz)
            let path_normalized = path.with_extension("");
            let (final_path, has_schema, file_content_opt) = if let Some(entry_files) =
                entry_point_files
            {
                // Check if the resolved path matches any entry point
                // Entry points may be stored with or without .ssz extension, so check both
                // Use get() for O(1) lookup instead of iterating (more efficient and deterministic)
                // Track which key matched so we use the same path format
                let (matched_path, matching_entry) =
                    if let Some(content) = entry_files.get(&path_normalized) {
                        (path_normalized.clone(), Some(content))
                    } else if let Some(content) = entry_files.get(&ssz_path) {
                        (ssz_path.clone(), Some(content))
                    } else {
                        (path_normalized.clone(), None)
                    };

                if let Some(file_content) = matching_entry {
                    // Use the matched path (preserving the extension format used by entry_files)
                    (matched_path, true, Some(file_content.clone()))
                } else if ssz_path.exists() {
                    // Fall back to filesystem check
                    (path.clone(), true, None)
                } else if !is_external {
                    // If .ssz file doesn't exist and not external, this is an existing Rust
                    // module at crate root. Strip parent directories to
                    // get just the module name (e.g., "ssz/ol" -> "ol")
                    let module_name = path
                        .file_name()
                        .expect("module path should have a file name");
                    let stripped_path = PathBuf::from(module_name);
                    (stripped_path, false, None)
                } else {
                    // External module without .ssz file - keep full path
                    (path.clone(), false, None)
                }
            } else if ssz_path.exists() {
                (path.clone(), true, None)
            } else if !is_external {
                // If .ssz file doesn't exist and not external, this is an existing Rust module
                // at crate root. Strip parent directories to get just the
                // module name (e.g., "ssz/ol" -> "ol")
                let module_name = path
                    .file_name()
                    .expect("module path should have a file name");
                let stripped_path = PathBuf::from(module_name);
                (stripped_path, false, None)
            } else {
                // External module without .ssz file - keep full path
                (path.clone(), false, None)
            };

            // Update import_map with the final resolved path
            // This ensures that when types reference this import, they use the correct path
            // (e.g., "state.ssz" instead of just "state")
            if import_map
                .insert(import_alias.clone(), final_path.clone())
                .is_some()
            {
                panic!("import: duplicate import alias: {import_alias:?}");
            }

            let add_module_result = module_manager.add_module(&final_path, is_external);
            if !add_module_result || is_external {
                return Ok(());
            }

            // Parse .ssz file if it exists
            if has_schema {
                let file_content = if let Some(content) = file_content_opt {
                    content
                } else {
                    std::fs::read_to_string(&ssz_path).expect("Failed to read import module file")
                };
                let chars = file_content.chars().collect::<Vec<_>>();
                let toks = crate::token::parse_char_array_to_tokens(&chars)
                    .expect("import: tokenize string");
                let tt = crate::token_tree::parse_tokens_to_toktrs(&toks)
                    .expect("import: treeize tokens");
                parse_module_from_toktrs(&tt, &final_path, module_manager, entry_point_files)
                    .expect("import: parse toktrs");
            }

            // Return the import module
            Ok(())
        }

        _ => Err(ParseError::MalformedBlock(sp)),
    }
}

/// Utility function to clean up gobble calls.
fn is_toktr_newline<T>(t: &TaggedToktr<T>) -> bool {
    matches!(t, TaggedToktr::Newline(_))
}

/// Utility function to clean up gobble calls.
fn is_toktr_eq<T>(t: &TaggedToktr<T>) -> bool {
    matches!(t, TaggedToktr::Eq(_))
}

/// Utility function to clean up gobble calls.
fn is_toktr_not_newline<T>(t: &TaggedToktr<T>) -> bool {
    !is_toktr_newline(t)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        ast::{AssignExpr, ModuleManager, ParseError, parse_module_from_toktrs},
        token::parse_char_array_to_tokens,
        token_tree::parse_tokens_to_toktrs,
        tysys::ConstValue,
    };

    #[test]
    fn test_ast_parse_consts() {
        let s = r"
FOO_BAR = 123
BAZ_QUUX = 1 << 42
FARB_NORB = 4 * 8
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");
        eprintln!("tree {tt:#?}");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");
        eprintln!("module {module_manager:#?}");
    }

    #[test]
    fn test_ast_parse_class_simple() {
        let s = r"
class Foo(Container):
    x: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");
        eprintln!("tree {tt:#?}");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");
        eprintln!("module {module_manager:#?}");
    }

    #[test]
    fn test_ast_parse_class_stable_container() {
        let s = r"
class Foo(StableContainer[16]):


    x_coordinate: Optional[uint32]




    y_coordinate: Optional[uint64]
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");
        eprintln!("tree {tt:#?}");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");
        eprintln!("module {module_manager:#?}");
    }

    #[test]
    fn test_ast_parse_import() {
        let s = r"
import import_test as test

A = test.A

class Foo(test.A):
    b: uint8

";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");
        eprintln!("tree {tt:#?}");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new("tests/non_existent"), false);
        parse_module_from_toktrs(
            &tt,
            Path::new("tests/non_existent"),
            &mut module_manager,
            None,
        )
        .expect("test: parse toktrs");
        eprintln!("module {module_manager:#?}");
    }

    #[test]
    fn test_ast_parse_class_with_doc_comment() {
        let s = r"
### This is a doc comment for the class
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            assert!(class_def.doc_comment().is_some());
            assert_eq!(
                class_def.doc_comment().unwrap(),
                " This is a doc comment for the class"
            );
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_with_pragma() {
        let s = r"
#~# some-directive value
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let pragmas = class_def.pragmas();
            assert_eq!(pragmas.len(), 1);
            // Pragma comments are trimmed, so leading space is removed
            assert_eq!(pragmas[0], "some-directive value");
        } else {
            panic!("Expected Class entry");
        }
    }

    // Note: Docstring parsing tests are skipped until docstring tokenization is implemented
    // The docstring functionality in the AST is ready, but tokenizer support for """..."""
    // docstrings needs to be added first.

    // #[test]
    // fn test_ast_parse_class_with_docstring() {
    //     // Test will be enabled once docstring tokenization is implemented
    // }

    // #[test]
    // fn test_ast_parse_class_multiple_docstrings_error() {
    //     // Test will be enabled once docstring tokenization is implemented
    // }

    #[test]
    fn test_ast_parse_field_with_comments() {
        let s = r"
class Point(Container):
    ### This is a doc comment for the field
    x: int32
    #~# field-pragma
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let fields = class_def.fields();
            assert_eq!(fields.len(), 2);

            // Check first field has doc comment
            assert!(fields[0].doc_comment().is_some());
            assert_eq!(
                fields[0].doc_comment().unwrap(),
                " This is a doc comment for the field"
            );

            // Check second field has pragma
            let pragmas = fields[1].pragmas();
            assert_eq!(pragmas.len(), 1);
            // Pragma comments are trimmed
            assert_eq!(pragmas[0], "field-pragma");
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_with_multiline_doc_comment() {
        let s = r"
### First line of doc comment
### Second line of doc comment
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let doc_comment = class_def.doc_comment().expect("Expected doc comment");
            // Should contain both lines merged with newline
            assert!(doc_comment.contains("First line"));
            assert!(doc_comment.contains("Second line"));
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_with_multiple_pragmas() {
        let s = r"
#~# first-pragma value1
#~# second-pragma value2
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let pragmas = class_def.pragmas();
            assert_eq!(pragmas.len(), 2);
            assert_eq!(pragmas[0], "first-pragma value1");
            assert_eq!(pragmas[1], "second-pragma value2");
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_regular_comment_discarded() {
        let s = r"
# This is a regular comment and should be discarded
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            // Regular comments should not be attached as doc_comment or pragmas
            assert!(class_def.doc_comment().is_none());
            assert_eq!(class_def.pragmas().len(), 0);
            // Class should still be parsed correctly
            assert_eq!(class_def.name().0, "Point");
            assert_eq!(class_def.fields().len(), 2);
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_with_both_doc_comment_and_pragma() {
        let s = r"
### This is a doc comment
#~# some-pragma directive
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            assert!(class_def.doc_comment().is_some());
            assert_eq!(class_def.doc_comment().unwrap(), " This is a doc comment");
            let pragmas = class_def.pragmas();
            assert_eq!(pragmas.len(), 1);
            assert_eq!(pragmas[0], "some-pragma directive");
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_multiple_fields_with_comments() {
        let s = r"
class Point(Container):
    ### Doc for field x
    x: int32
    ### Doc for field y
    y: int32
    #~# Pragma for field z
    z: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let fields = class_def.fields();
            assert_eq!(fields.len(), 3);

            // First field has doc comment
            assert!(fields[0].doc_comment().is_some());
            assert_eq!(fields[0].doc_comment().unwrap(), " Doc for field x");
            assert_eq!(fields[0].pragmas().len(), 0);

            // Second field has doc comment
            assert!(fields[1].doc_comment().is_some());
            assert_eq!(fields[1].doc_comment().unwrap(), " Doc for field y");
            assert_eq!(fields[1].pragmas().len(), 0);

            // Third field has pragma
            assert!(fields[2].doc_comment().is_none());
            assert_eq!(fields[2].pragmas().len(), 1);
            assert_eq!(fields[2].pragmas()[0], "Pragma for field z");
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_with_empty_doc_comment() {
        let s = r"
###
class Point(Container):
    x: int32
    y: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            // Empty doc comment should still be present but empty string
            let doc_comment = class_def.doc_comment();
            assert!(doc_comment.is_some());
            assert_eq!(doc_comment.unwrap(), "");
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_class_doc_comment_preserves_newlines() {
        let s = r"
### Line one
### Line two
### Line three
class Point(Container):
    x: int32
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        if let crate::ast::ModuleEntry::Class(class_def) = &entries[0] {
            let doc_comment = class_def.doc_comment().expect("Expected doc comment");
            assert!(doc_comment.contains("Line one"));
            assert!(doc_comment.contains("Line two"));
            assert!(doc_comment.contains("Line three"));
            // Should contain newlines between them
            assert!(doc_comment.contains('\n'));
        } else {
            panic!("Expected Class entry");
        }
    }

    #[test]
    fn test_ast_parse_standalone_docstring_error() {
        let s = r#"
"""
This is a standalone docstring
"""
class Foo(Container):
    x: uint8
"#;

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        let result = parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::StandaloneDocstring(_) => {
                // Expected error
            }
            other => panic!("Expected StandaloneDocstring error, got: {:?}", other),
        }
    }

    #[test]
    fn test_ast_parse_standalone_doccomment_error() {
        let s = r#"
### This is a standalone docstring

class Foo(Container):
    x: uint8
"#;

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        let result = parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::StandaloneDocComment => {
                // Expected error
            }
            other => panic!("Expected StandaloneDocComment error, got: {:?}", other),
        }
    }
    #[test]
    fn test_ast_parse_standalone_docstring_with_newlines_error() {
        let s = r#"
"""
This is a standalone docstring

With multiple lines
"""
class Foo(Container):
    x: uint8
"#;

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        let result = parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::StandaloneDocstring(_) => {
                // Expected error
            }
            other => panic!("Expected StandaloneDocstring error, got: {:?}", other),
        }
    }

    #[test]
    fn test_ast_parse_const_add_sub() {
        let s = r"
BASE = 1024
PLUS_ONE = BASE + 1
MINUS_ONE = BASE - 1
LITERAL_ADD = 10 + 5
LITERAL_SUB = 10 - 5
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");
        eprintln!("tree {tt:#?}");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        // Verify we parsed all constants
        assert_eq!(entries.len(), 5);

        // Check BASE constant
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[0] {
            assert_eq!(assign.name().0, "BASE");
            match assign.value() {
                AssignExpr::Value(ConstValue::Int(v)) => assert_eq!(*v, 1024),
                _ => panic!("Expected integer value for BASE"),
            }
        } else {
            panic!("Expected assignment for BASE");
        }

        // Check PLUS_ONE constant (BASE + 1)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[1] {
            assert_eq!(assign.name().0, "PLUS_ONE");
            match assign.value() {
                AssignExpr::SymbolicBinop(op, ident, v) => {
                    assert_eq!(*op, crate::tysys::Binop::Add);
                    assert_eq!(ident.0, "BASE");
                    assert_eq!(*v, 1);
                }
                _ => panic!("Expected symbolic binop for PLUS_ONE"),
            }
        } else {
            panic!("Expected assignment for PLUS_ONE");
        }

        // Check MINUS_ONE constant (BASE - 1)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[2] {
            assert_eq!(assign.name().0, "MINUS_ONE");
            match assign.value() {
                AssignExpr::SymbolicBinop(op, ident, v) => {
                    assert_eq!(*op, crate::tysys::Binop::Sub);
                    assert_eq!(ident.0, "BASE");
                    assert_eq!(*v, 1);
                }
                _ => panic!("Expected symbolic binop for MINUS_ONE"),
            }
        } else {
            panic!("Expected assignment for MINUS_ONE");
        }

        // Check LITERAL_ADD constant (10 + 5)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[3] {
            assert_eq!(assign.name().0, "LITERAL_ADD");
            match assign.value() {
                AssignExpr::Value(ConstValue::Binop(op, l, r)) => {
                    assert_eq!(*op, crate::tysys::Binop::Add);
                    assert_eq!(*l, 10);
                    assert_eq!(*r, 5);
                }
                _ => panic!("Expected value binop for LITERAL_ADD"),
            }
        } else {
            panic!("Expected assignment for LITERAL_ADD");
        }

        // Check LITERAL_SUB constant (10 - 5)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[4] {
            assert_eq!(assign.name().0, "LITERAL_SUB");
            match assign.value() {
                AssignExpr::Value(ConstValue::Binop(op, l, r)) => {
                    assert_eq!(*op, crate::tysys::Binop::Sub);
                    assert_eq!(*l, 10);
                    assert_eq!(*r, 5);
                }
                _ => panic!("Expected value binop for LITERAL_SUB"),
            }
        } else {
            panic!("Expected assignment for LITERAL_SUB");
        }

        eprintln!("module {module_manager:#?}");
    }

    /// Test the actual example from issue #49.
    #[test]
    fn test_ast_parse_issue_49_example() {
        let s = r"
### Maximum length of the predicate condition bytes
MAX_CONDITION_LEN = 1 << 10

### One additional byte for the PredicateTypeId
MAX_PREDICATE_LEN = MAX_CONDITION_LEN + 1
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None)
            .expect("test: parse toktrs");

        let entries = module_manager
            .get_module_mut(Path::new(""))
            .unwrap()
            .mut_entries();

        // Verify we parsed both constants
        assert_eq!(entries.len(), 2);

        // Check MAX_CONDITION_LEN constant (1 << 10)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[0] {
            assert_eq!(assign.name().0, "MAX_CONDITION_LEN");
            match assign.value() {
                AssignExpr::Value(ConstValue::Binop(op, l, r)) => {
                    assert_eq!(*op, crate::tysys::Binop::Shl);
                    assert_eq!(*l, 1);
                    assert_eq!(*r, 10);
                }
                _ => panic!("Expected value binop for MAX_CONDITION_LEN"),
            }
        } else {
            panic!("Expected assignment for MAX_CONDITION_LEN");
        }

        // Check MAX_PREDICATE_LEN constant (MAX_CONDITION_LEN + 1)
        if let crate::ast::ModuleEntry::Assignment(assign) = &entries[1] {
            assert_eq!(assign.name().0, "MAX_PREDICATE_LEN");
            match assign.value() {
                AssignExpr::SymbolicBinop(op, ident, v) => {
                    assert_eq!(*op, crate::tysys::Binop::Add);
                    assert_eq!(ident.0, "MAX_CONDITION_LEN");
                    assert_eq!(*v, 1);
                }
                _ => panic!("Expected symbolic binop for MAX_PREDICATE_LEN"),
            }
        } else {
            panic!("Expected assignment for MAX_PREDICATE_LEN");
        }

        eprintln!("Successfully parsed issue #49 example: {module_manager:#?}");
    }

    #[test]
    fn test_ast_parse_standalone_docstring_with_comments_error() {
        let s = r#"
### This comment




"""
This is a standalone docstring
"""
class Foo(Container):
    x: uint8
"#;

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");
        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        let mut module_manager = ModuleManager::new(&[]);
        module_manager.add_module(Path::new(""), false);
        let result = parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager, None);

        assert!(result.is_err());
        // The doc comment is detected first as standalone due to multiple newlines
        match result.unwrap_err() {
            ParseError::StandaloneDocComment => {
                // Expected error - doc comment is detected first
            }
            ParseError::StandaloneDocstring(_) => {
                // Also acceptable - docstring detected if comment wasn't standalone
            }
            other => panic!(
                "Expected StandaloneDocComment or StandaloneDocstring error, got: {:?}",
                other
            ),
        }
    }
}
