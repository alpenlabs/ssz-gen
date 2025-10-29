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

/// A module file containing a list of definitions.
#[derive(Clone, Debug)]
pub(crate) enum Module {
    External,
    Internal(Vec<ModuleEntry>, Vec<String>), // entries, module_derives
}

impl Module {
    pub(crate) fn new_external() -> Self {
        Self::External
    }

    pub(crate) fn new_internal(entry: Vec<ModuleEntry>) -> Self {
        Self::Internal(entry, Vec::new())
    }

    pub(crate) fn is_external(&self) -> bool {
        matches!(self, Self::External)
    }

    pub(crate) fn entries(&self) -> &[ModuleEntry] {
        match self {
            Self::External => &[],
            Self::Internal(entries, _) => entries,
        }
    }

    pub(crate) fn mut_entries(&mut self) -> &mut Vec<ModuleEntry> {
        match self {
            Self::External => panic!("external module has no entries"),
            Self::Internal(entries, _) => entries,
        }
    }

    pub(crate) fn module_derives(&self) -> &[String] {
        match self {
            Self::External => &[],
            Self::Internal(_, module_derives) => module_derives,
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

    /// A type expression.
    Complex(ComplexTySpec),

    /// An integer literal.
    Value(ConstValue),
}

/// A class definition.
///
/// Classes must always have parent types.
#[derive(Clone, Debug)]
pub(crate) struct ClassDefEntry {
    name: Identifier,
    parent_ty: TyExprSpec,
    fields: Vec<FieldDef>,
    derives: Vec<String>,
}

impl ClassDefEntry {
    pub(crate) fn new(name: Identifier, parent_ty: TyExprSpec, fields: Vec<FieldDef>) -> Self {
        Self {
            name,
            parent_ty,
            fields,
            derives: Vec::new(),
        }
    }

    pub(crate) fn new_with_derives(
        name: Identifier,
        parent_ty: TyExprSpec,
        fields: Vec<FieldDef>,
        derives: Vec<String>,
    ) -> Self {
        Self {
            name,
            parent_ty,
            fields,
            derives,
        }
    }

    pub(crate) fn name(&self) -> &Identifier {
        &self.name
    }

    pub(crate) fn parent_ty(&self) -> &TyExprSpec {
        &self.parent_ty
    }

    pub(crate) fn fields(&self) -> &[FieldDef] {
        &self.fields
    }

    pub(crate) fn derives(&self) -> &[String] {
        &self.derives
    }
}

/// A field definition within a class.
#[derive(Clone, Debug)]
pub(crate) struct FieldDef {
    name: Identifier,
    ty: TyExprSpec,
}

impl FieldDef {
    pub(crate) fn new(name: Identifier, ty: TyExprSpec) -> Self {
        Self { name, ty }
    }

    pub(crate) fn name(&self) -> &Identifier {
        &self.name
    }

    pub(crate) fn ty(&self) -> &TyExprSpec {
        &self.ty
    }
}

/// A type specification.
///
/// This needs to be further resolved to figure out ambiguous identifiers.
#[derive(Clone, Debug)]
pub(crate) enum TyExprSpec {
    /// This is an imported type.
    Imported(ImportedTySpec),

    /// This is just a single identifier.  It could refer to a type or a const.
    Simple(Identifier),

    /// This is an identifier and type parameters.  It probably refers to a
    /// type, but we need to sanity check all of this.
    Complex(ComplexTySpec),
}

impl TyExprSpec {
    pub(crate) fn base_name(&self) -> &Identifier {
        match self {
            TyExprSpec::Simple(name) => name,
            TyExprSpec::Complex(spec) => &spec.base_name,
            TyExprSpec::Imported(spec) => &spec.base_name,
        }
    }
}

/// An imported type.
///
/// No verification is done at this stage.
#[derive(Clone, Debug)]
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

/// An instantiated generic type.
///
/// At this stage we have not verified that `base_name` is actually a type.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum TyArgSpec {
    /// An imported type.
    Imported(ImportedTySpec),

    /// An identifier, which could be a constant or a type name.
    Ident(Identifier),

    /// An identifier (presumably a type) with a list of arguments.
    Complex(ComplexTySpec),

    /// An literal integer.
    IntLiteral(u64),

    /// None type for Unions
    None,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("malformed def at {0}")]
    MalformedBlock(SrcPos),

    #[error("unexpected token at {0}")]
    UnexpectedToken(SrcPos),

    #[error("unexpected end of input")]
    UnexpectedEnd,

    #[error("not yet implemented")]
    Unimplemented,

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

    /// Removes and returns the last module from the manager.
    ///
    /// Returns the module if it was removed, None if there are no more modules.
    pub(crate) fn pop_module(&mut self) -> Option<(PathBuf, Module)> {
        let path = self.import_order.pop()?;
        let module = self.modules.remove(&path)?;
        Some((path, module))
    }

    /// Returns a module by path.
    pub(crate) fn _get_module<P: AsRef<Path>>(&self, path: P) -> Option<&Module> {
        self.modules.get(path.as_ref())
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
) -> Result<(), ParseError> {
    let path = path.as_ref();
    let mut gob = Gobbler::new(toktrs);
    let mut import_map = HashMap::new();
    let mut module_derives = Vec::new();

    while let Some(cur) = gob.get() {
        match cur {
            // Discard newlines.
            TaggedToktr::Newline(_) => gob.gobble_one(),

            // Lines that start with "import" are imports.
            TaggedToktr::Import(_) => {
                parse_import(&mut gob, path, module_manager, &mut import_map)?;
            }

            // Lines that start with identifiers are probably assignments.
            TaggedToktr::Identifier(_, _) => {
                let cd = parse_assignment(&mut gob, &import_map)?;
                module_manager
                    .get_module_mut(path)
                    .unwrap()
                    .mut_entries()
                    .push(ModuleEntry::Assignment(cd));
            }

            // Lines that start with "class" are always classes.
            TaggedToktr::Class(_) => {
                let cd = parse_class(&mut gob, &import_map)?;
                module_manager
                    .get_module_mut(path)
                    .unwrap()
                    .mut_entries()
                    .push(ModuleEntry::Class(cd));
            }

            // Handle decorators
            TaggedToktr::Decorator(_, content) => {
                let decorator_content = content.trim();
                if decorator_content.starts_with("@module_derive(")
                    && decorator_content.ends_with(")")
                {
                    // Parse module-level derives
                    let derives_str = &decorator_content[15..decorator_content.len() - 1]; // Remove "@module_derive(" and ")"
                    let derives: Vec<String> = derives_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    module_derives.extend(derives);
                    gob.gobble_one();
                } else {
                    // This is a per-class decorator, don't consume it
                    // Let parse_class handle it
                    let cd = parse_class(&mut gob, &import_map)?;
                    module_manager
                        .get_module_mut(path)
                        .unwrap()
                        .mut_entries()
                        .push(ModuleEntry::Class(cd));
                }
            }

            t => return Err(ParseError::UnexpectedToken(*t.tag())),
        }
    }

    // Update the module with module-level derives
    if !module_derives.is_empty()
        && let Some(module) = module_manager.get_module_mut(path)
    {
        match module {
            Module::Internal(_, existing_derives) => {
                existing_derives.extend(module_derives);
            }
            Module::External => {
                // This shouldn't happen for internal modules
            }
        }
    }

    Ok(())
}

/// Parses a const definition out of the gobbler.
fn parse_assignment(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<AssignEntry, ParseError> {
    use TaggedToktr::*;

    let ident = match gob.gobble_slice_up_to(is_toktr_eq) {
        Some([Identifier(_, ident)]) => ident.clone(),
        _ => return Err(ParseError::UnexpectedEnd),
    };

    // Skip over the `=`.
    let eq = gob.get();
    assert!(
        matches!(eq, Some(TaggedToktr::Eq(_))),
        "ast: missing expected eq"
    );
    gob.gobble_one();

    let Some(expr_slice) = gob.gobble_slice_up_to(is_toktr_newline) else {
        return Err(ParseError::UnexpectedEnd);
    };
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
    use TaggedToktr::*;

    let expr = match toktrs {
        // This is probably an alias.
        [Identifier(_, name)] => AssignExpr::Name(name.clone()),

        [Identifier(_, name), BracketBlock(_, arg_toks)] => {
            let mut gob = Gobbler::new(arg_toks.children());
            let args = parse_ty_args(&mut gob, import_map)?;
            AssignExpr::Complex(ComplexTySpec::new(name.clone(), args))
        }

        // Simple integer expression.
        [IntegerLiteral(_, v)] => AssignExpr::Value(ConstValue::Int(*v)),

        // This is a shl value.
        [IntegerLiteral(_, v), Shl(_), IntegerLiteral(_, shl_v)] => {
            AssignExpr::Value(ConstValue::Binop(Binop::Shl, *v, *shl_v))
        }

        [IntegerLiteral(_, mul_l), Mul(_), IntegerLiteral(_, mul_r)] => {
            AssignExpr::Value(ConstValue::Binop(Binop::Mul, *mul_l, *mul_r))
        }

        [Identifier(_, module_name), Dot(_), Identifier(_, ident)] => {
            let module_path = import_map.get(module_name).unwrap();
            AssignExpr::Imported(ImportedTySpec::new(module_path.clone(), ident.clone()))
        }

        _ => return Err(ParseError::UnexpectedEnd),
    };

    Ok(expr)
}

/// Parses decorators before a class definition
fn parse_class_decorators(gob: &mut Gobbler<'_, SrcToktr>) -> Result<Vec<String>, ParseError> {
    let mut derives = Vec::new();

    // Look ahead to see if there are decorators before the class
    while let Some(cur) = gob.get() {
        match cur {
            TaggedToktr::Decorator(_, content) => {
                let decorator_content = content.trim();
                if decorator_content.starts_with("@derive(") && decorator_content.ends_with(")") {
                    // Parse class-level derives
                    let derives_str = &decorator_content[8..decorator_content.len() - 1]; // Remove "@derive(" and ")"
                    let class_derives: Vec<String> = derives_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    derives.extend(class_derives);
                }
                gob.gobble_one();
            }
            TaggedToktr::Newline(_) => {
                gob.gobble_one();
            }
            _ => break, // Not a decorator, stop looking
        }
    }

    Ok(derives)
}

/// Parses a class definition out of a gobbler.
fn parse_class(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<ClassDefEntry, ParseError> {
    use TaggedToktr::*;

    // First, look for decorators before the class
    let derives = parse_class_decorators(gob)?;

    let sp = *gob.get_expect().tag();

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

            let body_data = match gob.get() {
                Some(IndentBlock(_, d)) => d,
                Some(t) => return Err(ParseError::UnexpectedToken(*t.tag())),
                Option::None => return Err(ParseError::UnexpectedEnd),
            };

            let mut body_gob = Gobbler::new(body_data.children());
            let fields = parse_class_body(&mut body_gob, import_map)?;
            gob.gobble_one();

            let cd = if derives.is_empty() {
                ClassDefEntry::new(name, parent_ty, fields)
            } else {
                ClassDefEntry::new_with_derives(name, parent_ty, fields, derives)
            };
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
            TyExprSpec::Imported(ImportedTySpec::new(module_path.clone(), second_ident))
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

                    Ok(TyArgSpec::Imported(ImportedTySpec::new(
                        module_path.clone(),
                        second_ident,
                    )))
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

/// Parses a class body out of a gobbler.  The entries must be the full gobbler.
fn parse_class_body(
    gob: &mut Gobbler<'_, SrcToktr>,
    import_map: &HashMap<Identifier, PathBuf>,
) -> Result<Vec<FieldDef>, ParseError> {
    use TaggedToktr::*;

    let mut fields = Vec::new();

    while gob.has_entry() {
        match gob.gobble_slice_up_to(is_toktr_newline) {
            Some(
                [
                    Identifier(_, fname),
                    Colon(_),
                    Identifier(_, tyname),
                    BracketBlock(_, tyarg_data),
                ],
            ) => {
                let mut arg_gob = Gobbler::new(tyarg_data.children());
                let ty_args = parse_ty_args(&mut arg_gob, import_map)?;
                let ty = TyExprSpec::Complex(ComplexTySpec::new(tyname.clone(), ty_args));
                fields.push(FieldDef::new(fname.clone(), ty));
            }

            Some(
                [
                    Identifier(_, fname),
                    Colon(_),
                    Identifier(_, first_ident),
                    Dot(_),
                    Identifier(_, second_ident),
                ],
            ) => {
                let module_path = import_map.get(first_ident).unwrap();
                let ty = TyExprSpec::Imported(ImportedTySpec::new(
                    module_path.clone(),
                    second_ident.clone(),
                ));
                fields.push(FieldDef::new(fname.clone(), ty));
            }

            Some([Identifier(_, fname), Colon(_), Identifier(_, tyname)]) => {
                let ty = TyExprSpec::Simple(tyname.clone());
                fields.push(FieldDef::new(fname.clone(), ty));
            }

            Some([t, ..]) => return Err(ParseError::UnexpectedToken(*t.tag())),

            _ => return Err(ParseError::UnexpectedEnd),
        }

        // Ignore extra newlines.
        gob.gobble_until(is_toktr_not_newline);
    }

    Ok(fields)
}

/// Parses import statements by reading the imported module and parsing it
fn parse_import<P: AsRef<Path>>(
    gob: &mut Gobbler<'_, SrcToktr>,
    path: P,
    module_manager: &mut ModuleManager,
    import_map: &mut HashMap<Identifier, PathBuf>,
) -> Result<(), ParseError> {
    use TaggedToktr::*;

    let path = path.as_ref();
    let sp = *gob.get_expect().tag();

    match gob.view() {
        [Import(_), ..] => {
            gob.gobble_one();
            let path_tokens = match gob.gobble_slice_up_to(is_toktr_newline) {
                Some(p) => Ok(p),
                _ => return Err(ParseError::UnexpectedEnd),
            }?;

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
                                path = PathBuf::new();
                            }
                        }
                        path = path.join(&name.0);
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
                                path = PathBuf::new();
                            }
                        }
                        path = path.join(&name.0);
                        import_alias = alias.clone();
                        path_gob.gobble_exact(3);
                        break;
                    }
                    [Identifier(_, name)] => {
                        if is_first_tok {
                            is_external = module_manager.external_modules.contains(&name.0);
                            if is_external {
                                path = PathBuf::new();
                            }
                        }
                        path = path.join(&name.0);
                        import_alias = name.clone();
                        path_gob.gobble_one();
                        break;
                    }
                    [t, ..] => return Err(ParseError::UnexpectedToken(*t.tag())),
                    _ => return Err(ParseError::UnexpectedEnd),
                }
                is_first_tok = false;
            }

            if import_map
                .insert(import_alias.clone(), path.clone())
                .is_some()
            {
                panic!("import: duplicate import alias: {import_alias:?}");
            }
            let add_module_result = module_manager.add_module(&path, is_external);
            if !add_module_result || is_external {
                return Ok(());
            }

            // Read the import module file
            let file_content = std::fs::read_to_string(path.with_extension("ssz"))
                .expect("Failed to read import module file");

            // Parse the import module file into module entries
            let chars = file_content.chars().collect::<Vec<_>>();
            let toks =
                crate::token::parse_char_array_to_tokens(&chars).expect("import: tokenize string");
            let tt =
                crate::token_tree::parse_tokens_to_toktrs(&toks).expect("import: treeize tokens");
            parse_module_from_toktrs(&tt, &path, module_manager).expect("import: parse toktrs");

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
        ast::{ModuleManager, parse_module_from_toktrs},
        token::parse_char_array_to_tokens,
        token_tree::parse_tokens_to_toktrs,
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
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager)
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
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager)
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
        parse_module_from_toktrs(&tt, Path::new(""), &mut module_manager)
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
        parse_module_from_toktrs(&tt, Path::new("tests/non_existent"), &mut module_manager)
            .expect("test: parse toktrs");
        eprintln!("module {module_manager:#?}");
    }
}
