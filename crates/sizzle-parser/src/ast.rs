//! AST types

use std::fmt::Debug;

use thiserror::Error;

use crate::{
    Identifier, SrcPos, TaggedToktr,
    gobbler::Gobbler,
    token_tree::SrcToktr,
    tysys::{Binop, ConstValue},
};

/// A module file containing a list of definitions.
#[derive(Clone, Debug)]
pub(crate) struct Module {
    entries: Vec<ModuleEntry>,
}

impl Module {
    pub(crate) fn new(entry: Vec<ModuleEntry>) -> Self {
        Self { entries: entry }
    }

    pub(crate) fn entries(&self) -> &[ModuleEntry] {
        &self.entries
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
}

impl ClassDefEntry {
    pub(crate) fn new(name: Identifier, parent_ty: TyExprSpec, fields: Vec<FieldDef>) -> Self {
        Self {
            name,
            parent_ty,
            fields,
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
        }
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

/// Parses a module from a sequence of tokens.
pub(crate) fn parse_module_from_toktrs(toktrs: &[SrcToktr]) -> Result<Module, ParseError> {
    let mut gob = Gobbler::new(toktrs);

    let mut defs = Vec::new();

    while let Some(cur) = gob.get() {
        match cur {
            // Discard newlines.
            TaggedToktr::Newline(_) => gob.gobble_one(),

            // Lines that start with identifiers are probably assignments.
            TaggedToktr::Identifier(_, _) => {
                let cd = parse_assignment(&mut gob)?;
                defs.push(ModuleEntry::Assignment(cd));
            }

            // Lines that start with "class" are always classes.
            TaggedToktr::Class(_) => {
                let cd = parse_class(&mut gob)?;
                defs.push(ModuleEntry::Class(cd));
            }

            t => return Err(ParseError::UnexpectedToken(*t.tag())),
        }
    }

    Ok(Module::new(defs))
}

/// Parses a const definition out of the gobbler.
fn parse_assignment(gob: &mut Gobbler<'_, SrcToktr>) -> Result<AssignEntry, ParseError> {
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
    let val = parse_assign_expr(expr_slice)?;

    Ok(AssignEntry::new(ident, val))
}

/// Parses a slice of tokens as an expression.
///
/// Since we don't support arbitrary expressions, this can be actually pretty simple!
fn parse_assign_expr(toktrs: &[SrcToktr]) -> Result<AssignExpr, ParseError> {
    use TaggedToktr::*;

    let expr = match toktrs {
        // This is probably an alias.
        [Identifier(_, name)] => AssignExpr::Name(name.clone()),

        [Identifier(_, name), BracketBlock(_, arg_toks)] => {
            let mut gob = Gobbler::new(arg_toks.children());
            let args = parse_ty_args(&mut gob)?;
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

        [t, ..] => return Err(ParseError::UnexpectedToken(*t.tag())),

        _ => return Err(ParseError::UnexpectedEnd),
    };

    Ok(expr)
}

/// Parses a class definition out of a gobbler.
fn parse_class(gob: &mut Gobbler<'_, SrcToktr>) -> Result<ClassDefEntry, ParseError> {
    use TaggedToktr::*;

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
            let parent_ty = parse_ty(&mut ty_gob)?;

            // Then extract the body and parse it.
            gob.gobble_until(is_toktr_newline);
            gob.gobble_until(is_toktr_not_newline);

            let body_data = match gob.get() {
                Some(IndentBlock(_, d)) => d,
                Some(t) => return Err(ParseError::UnexpectedToken(*t.tag())),
                Option::None => return Err(ParseError::UnexpectedEnd),
            };

            let mut body_gob = Gobbler::new(body_data.children());
            let fields = parse_class_body(&mut body_gob)?;
            gob.gobble_one();

            let cd = ClassDefEntry::new(name, parent_ty, fields);
            Ok(cd)
        }

        _ => Err(ParseError::MalformedBlock(sp)),
    }
}

/// Parses a type specification out of a gobbler.
fn parse_ty(gob: &mut Gobbler<'_, SrcToktr>) -> Result<TyExprSpec, ParseError> {
    let first_tok = gob.get().ok_or(ParseError::UnexpectedEnd)?;
    let TaggedToktr::Identifier(_, ty_name) = first_tok else {
        return Err(ParseError::UnexpectedToken(*first_tok.tag()));
    };

    let ty_name = ty_name.clone();
    gob.gobble_one();

    let ty = match gob.get() {
        Some(TaggedToktr::BracketBlock(_, data)) => {
            let mut sub_gob = Gobbler::new(data.children());
            let args = parse_ty_args(&mut sub_gob)?;
            gob.gobble_one();
            TyExprSpec::Complex(ComplexTySpec::new(ty_name, args))
        }

        Some(t) => {
            return Err(ParseError::UnexpectedToken(*t.tag()));
        }

        None => TyExprSpec::Simple(ty_name),
    };

    Ok(ty)
}

/// Parses type args out of a gobbler.  The gobbler must be exactly the sequence
/// of type arguments, with commas separating arguments.
fn parse_ty_args(gob: &mut Gobbler<'_, SrcToktr>) -> Result<Vec<TyArgSpec>, ParseError> {
    let mut args = Vec::new();

    while gob.has_entry() {
        // Parsing the arg is easy.
        args.push(parse_ty_arg(gob)?);

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

fn parse_ty_arg(gob: &mut Gobbler<'_, SrcToktr>) -> Result<TyArgSpec, ParseError> {
    match gob.get() {
        Some(TaggedToktr::Null(_)) => {
            gob.gobble_one();
            Ok(TyArgSpec::None)
        }

        // An identifier could be a type or a const, we'll resolve that later.
        Some(TaggedToktr::Identifier(_, ident)) => {
            let ident = ident.clone();
            gob.gobble_one();
            match gob.get() {
                // Have generic arguments we also want to consume.
                Some(TaggedToktr::BracketBlock(_, data)) => {
                    let mut args_gob = Gobbler::new(data.children());
                    let args = parse_ty_args(&mut args_gob)?;
                    gob.gobble_one();
                    Ok(TyArgSpec::Complex(ComplexTySpec::new(ident, args)))
                }

                // This would be the next item.
                Some(TaggedToktr::Comma(_)) => Ok(TyArgSpec::Ident(ident)),

                // Other cases, we shouldn't have these.
                Some(t) => Err(ParseError::UnexpectedToken(*t.tag())),

                // At the end of the block.
                None => Ok(TyArgSpec::Ident(ident)),
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
fn parse_class_body(gob: &mut Gobbler<'_, SrcToktr>) -> Result<Vec<FieldDef>, ParseError> {
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
                let ty_args = parse_ty_args(&mut arg_gob)?;
                let ty = TyExprSpec::Complex(ComplexTySpec::new(tyname.clone(), ty_args));
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
    use crate::{
        ast::parse_module_from_toktrs, token::parse_char_array_to_tokens,
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

        let m = parse_module_from_toktrs(&tt).expect("test: parse toktrs");
        eprintln!("module {m:#?}");
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

        let m = parse_module_from_toktrs(&tt).expect("test: parse toktrs");
        eprintln!("module {m:#?}");
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

        let m = parse_module_from_toktrs(&tt).expect("test: parse toktrs");
        eprintln!("module {m:#?}");
    }
}
