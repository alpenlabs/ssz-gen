//! Type system resolver.
//!
//! Does the weird bookkeeping to figure out if schema types are well-formed.

use std::collections::HashMap;

use thiserror::Error;

use crate::{
    Identifier,
    ast::{TyArgSpec, TyExprSpec},
    tysys::{ConstValue, Ty, TyExpr},
};

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("unknown type '{0:?}'")]
    UnknownType(Identifier),

    #[error("unknown identifier '{0:?}'")]
    UnknownIdent(Identifier),

    #[error("mismatched arg in type '{0:?}' (wanted {1:?}, got {2:?})")]
    MismatchedArg(Identifier, CtorArg, TyArgSpec),

    #[error("mismatched arg (in type based on '{0:?}')")]
    MismatchedArgKind(Identifier),

    #[error("mismatched arity for type '{0:?}'")]
    MismatchTypeArity(Identifier),

    #[error("used args on const identifier '{0:?}'")]
    ArgsOnConst(Identifier),

    #[error("tried to redeclare identifier '{0:?}'")]
    RedeclareIdentifier(Identifier),
}

/// Describes information for a concrete type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TypeData {
    // TODO
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TypeCtorData {
    /// The signature.
    sig: CtorSig,

    /// Data for the type when we instantiate it.
    type_data: TypeData,
}

impl TypeCtorData {
    pub(crate) fn new(sig: CtorSig, type_data: TypeData) -> Self {
        Self { sig, type_data }
    }

    pub(crate) fn _sig(&self) -> &CtorSig {
        &self.sig
    }

    pub(crate) fn _type_data(&self) -> &TypeData {
        &self.type_data
    }
}

/// Describes the structure of arguments a type constructor accepts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CtorSig {
    /// Fixed arguments.  It could be no arguments and be like a thunk.
    ///
    /// Ex: `StableContainer[N]`, `List[T, N]`
    Fixed(Vec<CtorArg>),

    /// Takes a variable number of type arguments.
    ///
    /// Ex: `Union[T1, ..., Tn]`
    VariableTy,
}

/// Constructor arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CtorArg {
    /// Argument position is another type.
    Ty,

    /// Argument position is an resolvable int const.
    Int,
}

/// Describes something an identifier can point to.
#[derive(Clone, Debug)]
pub(crate) enum IdentTarget {
    Const(ConstValue),
    Ty(TypeData),
    TyCtor(TypeCtorData),
}

/// Describes something an alias can point to.  Right now this is always just a
/// direct reference to a concrete type.
#[derive(Clone, Debug)]
pub(crate) enum AliasRef {
    /// Direct alias of another type.
    Direct(Ty),
}

#[derive(Clone)]
pub(crate) struct TypeResolver {
    // TODO some way to express types that can be inherited from and types that can only be used as a member
    /// Constants in the module scope.
    idents: HashMap<Identifier, IdentTarget>,

    /// Maps aliases to other things.
    aliases: HashMap<Identifier, AliasRef>,
}

impl TypeResolver {
    pub(crate) fn new() -> Self {
        Self {
            idents: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    fn check_name_unused(&self, ident: &Identifier) -> Result<(), ResolverError> {
        if self.idents.contains_key(ident) || self.aliases.contains_key(ident) {
            return Err(ResolverError::RedeclareIdentifier(ident.clone()));
        }

        Ok(())
    }

    /// Inserts a type with a specific signature.
    pub(crate) fn insert_type(
        &mut self,
        ident: Identifier,
        type_data: TypeData,
    ) -> Result<(), ResolverError> {
        self.check_name_unused(&ident)?;
        self.idents.insert(ident, IdentTarget::Ty(type_data));
        Ok(())
    }

    pub(crate) fn insert_type_ctor(
        &mut self,
        ident: Identifier,
        ctor: CtorSig,
    ) -> Result<(), ResolverError> {
        self.check_name_unused(&ident)?;
        self.idents.insert(
            ident,
            IdentTarget::TyCtor(TypeCtorData::new(ctor, TypeData {})),
        );
        Ok(())
    }

    /// Declares a user type (which is a `Unit` type which does not accept
    /// arguments).
    pub(crate) fn decl_user_type(&mut self, ident: Identifier) -> Result<(), ResolverError> {
        self.check_name_unused(&ident)?;
        self.insert_type(ident, TypeData {})
    }

    /// Inserts an alias.
    fn insert_alias(&mut self, ident: Identifier, ar: AliasRef) -> Result<(), ResolverError> {
        self.check_name_unused(&ident)?;
        self.aliases.insert(ident, ar);
        Ok(())
    }

    /// Declares a type alias.  The target type MUST exist here, will cause errors otherwise.
    pub(crate) fn decl_type_alias(
        &mut self,
        ident: Identifier,
        ty: Ty,
    ) -> Result<(), ResolverError> {
        self.insert_alias(ident, AliasRef::Direct(ty))
    }

    /// Declares a const with an unspecified value.
    pub(crate) fn decl_const(
        &mut self,
        ident: Identifier,
        value: ConstValue,
    ) -> Result<(), ResolverError> {
        self.check_name_unused(&ident)?;
        self.idents.insert(ident, IdentTarget::Const(value));
        Ok(())
    }

    /// Gets an identifier if it's a type.
    pub(crate) fn _get_ident_as_ty(&self, ident: &Identifier) -> Result<&TypeData, ResolverError> {
        match self
            .idents
            .get(ident)
            .ok_or_else(|| ResolverError::UnknownIdent(ident.clone()))?
        {
            IdentTarget::Ty(td) => Ok(td),
            _ => Err(ResolverError::MismatchedArgKind(ident.clone())),
        }
    }

    /// Queries an identifier to the underlying `TypeData`, following alias references.
    pub(crate) fn _query_ident_typedata<'t>(
        &'t self,
        ident: &Identifier,
    ) -> Result<&'t TypeData, ResolverError> {
        if let Some(s) = self.idents.get(ident) {
            return Ok(match s {
                IdentTarget::Ty(td) => td,
                IdentTarget::TyCtor(ctor) => ctor._type_data(),
                _ => return Err(ResolverError::MismatchedArgKind(ident.clone())),
            });
        }

        match self.aliases.get(ident) {
            Some(ar) => Ok(match ar {
                AliasRef::Direct(conc_ty) => self._query_ident_typedata(conc_ty.base_name())?,
            }),
            None => Err(ResolverError::UnknownIdent(ident.clone())),
        }
    }

    /// Gets an identifier if it's simply a const.
    pub(crate) fn _get_ident_as_const(
        &self,
        ident: &Identifier,
    ) -> Result<&ConstValue, ResolverError> {
        match self
            .idents
            .get(ident)
            .ok_or_else(|| ResolverError::UnknownIdent(ident.clone()))?
        {
            IdentTarget::Const(v) => Ok(v),
            _ => Err(ResolverError::MismatchedArgKind(ident.clone())),
        }
    }

    /// Gets the referent of an identifier.
    pub(crate) fn get_ident_referent(&self, ident: &Identifier) -> Option<&IdentTarget> {
        self.idents.get(ident)
    }

    /// Resolves an identifier as an argument, possibly with its own arguments.
    pub(crate) fn resolve_ident_with_args(
        &self,
        ident: &Identifier,
        args: Option<&[TyArgSpec]>,
    ) -> Result<TyExpr, ResolverError> {
        // Check to see if this is an alias already.
        if let Some(alias) = self.aliases.get(ident) {
            match alias {
                AliasRef::Direct(ty) => {
                    // Just make sure we have no args.
                    if args.is_some() {
                        return Err(ResolverError::MismatchTypeArity(ident.clone()));
                    }

                    // re-construct the type
                    return Ok(TyExpr::Ty(ty.clone()));
                }
            }
        };

        // Otherwise, we do normal resolution.
        let Some(target) = self.get_ident_referent(ident) else {
            return Err(ResolverError::UnknownIdent(ident.clone()));
        };

        self.resolve_target(ident, args, target)
    }

    /// Resolves an identifier target against a potential instantiation.
    fn resolve_target(
        &self,
        ident: &Identifier,
        args: Option<&[TyArgSpec]>,
        target: &IdentTarget,
    ) -> Result<TyExpr, ResolverError> {
        match target {
            IdentTarget::Const(v) => {
                if args.is_none() {
                    Ok(TyExpr::Int(v.clone()))
                } else {
                    Err(ResolverError::ArgsOnConst(ident.clone()))
                }
            }

            IdentTarget::Ty(_td) => {
                if args.is_none() {
                    Ok(TyExpr::Ty(Ty::Simple(ident.clone())))
                } else {
                    Err(ResolverError::MismatchTypeArity(ident.clone()))
                }
            }

            IdentTarget::TyCtor(cd) => match (&cd.sig, args) {
                // This is types that take a specific number of arguments of varying kinds.
                (CtorSig::Fixed(sig_args), Some(spec_args)) => {
                    let mut args: Vec<TyExpr> = Vec::new();

                    // Go through each arg and make sure it matches the description.
                    for (sig_arg, spec_arg) in sig_args.iter().zip(spec_args.iter()) {
                        let arg: TyExpr = match (sig_arg, spec_arg) {
                            (CtorArg::Ty, TyArgSpec::None) | (CtorArg::Int, TyArgSpec::None) => {
                                return Err(ResolverError::MismatchedArg(
                                    ident.clone(),
                                    sig_arg.clone(),
                                    spec_arg.clone(),
                                ));
                            }
                            (CtorArg::Ty, TyArgSpec::Ident(arg_ident)) => {
                                self.resolve_ident_with_args(arg_ident, None)?
                            }
                            (CtorArg::Ty, TyArgSpec::Complex(complex)) => {
                                match self.resolve_ident_with_args(
                                    complex.base_name(),
                                    Some(complex.args()),
                                )? {
                                    a @ TyExpr::Ty(_) => a,
                                    TyExpr::Int(_) => {
                                        panic!("tyresolv: complex type resolved as const")
                                    }
                                    TyExpr::None => {
                                        panic!("tyresolv: complex type resolved as none")
                                    }
                                }
                            }
                            (CtorArg::Ty, TyArgSpec::IntLiteral(_)) => {
                                return Err(ResolverError::MismatchedArg(
                                    ident.clone(),
                                    sig_arg.clone(),
                                    spec_arg.clone(),
                                ));
                            }
                            (CtorArg::Int, TyArgSpec::Ident(arg_ident)) => {
                                self.resolve_ident_with_args(arg_ident, None)?
                            }
                            (CtorArg::Int, TyArgSpec::Complex(_)) => {
                                return Err(ResolverError::MismatchedArgKind(ident.clone()));
                            }
                            (CtorArg::Int, TyArgSpec::IntLiteral(v)) => {
                                TyExpr::Int(ConstValue::Int(*v))
                            }
                        };

                        args.push(arg);
                    }

                    Ok(TyExpr::Ty(Ty::Complex(ident.clone(), args)))
                }

                // This is types that take a varying number of arguments that are all types.
                (CtorSig::VariableTy, Some(spec_args)) => {
                    let mut args = Vec::new();

                    for spec_arg in spec_args {
                        let arg: TyExpr = match spec_arg {
                            TyArgSpec::None => TyExpr::None,
                            TyArgSpec::Ident(arg_ident) => {
                                self.resolve_ident_with_args(arg_ident, None)?
                            }
                            TyArgSpec::Complex(complex) => match self.resolve_ident_with_args(
                                complex.base_name(),
                                Some(complex.args()),
                            )? {
                                TyExpr::None => TyExpr::None,
                                a @ TyExpr::Ty(_) => a,
                                TyExpr::Int(_) => panic!("tyresolv: complex type resolved to int"),
                            },
                            TyArgSpec::IntLiteral(_) => {
                                return Err(ResolverError::MismatchedArgKind(ident.clone()));
                            }
                        };

                        args.push(arg);
                    }

                    Ok(TyExpr::Ty(Ty::Complex(ident.clone(), args)))
                }

                // All other cases are mismatched arity in some way or another.
                _ => Err(ResolverError::MismatchTypeArity(ident.clone())),
            },
        }
    }

    /// Resolves a type spec to a type expr.
    ///
    /// This is so huge so that we can preserve context for error reporting.
    pub(crate) fn resolve_spec_as_ty(&self, spec: &TyExprSpec) -> Result<Ty, ResolverError> {
        // Just match on the expression simply, then go from there.
        let expr = match spec {
            TyExprSpec::Simple(name) => self.resolve_ident_with_args(name, None)?,
            TyExprSpec::Complex(complex) => {
                self.resolve_ident_with_args(complex.base_name(), Some(complex.args()))?
            }
        };

        // And then just make sure it's a const.
        match expr {
            TyExpr::Ty(ty) => Ok(ty),
            TyExpr::Int(_) | TyExpr::None => {
                Err(ResolverError::MismatchedArgKind(spec.base_name().clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Identifier,
        ast::{ComplexTySpec, TyArgSpec, TyExprSpec},
        builtins,
        tysys::ConstValue,
    };

    use super::TypeResolver;

    fn make_ident(s: &str) -> Identifier {
        Identifier::try_from(s.to_owned()).expect("test: make ident")
    }

    fn make_resolver() -> TypeResolver {
        let mut resolv = TypeResolver::new();
        builtins::populate_builtin_types(&mut resolv);
        resolv
    }

    #[test]
    fn test_resolver_simple() {
        let resolv = make_resolver();

        let spec = TyExprSpec::Simple(make_ident("Container"));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make Container");

        eprintln!("{ty:?}");
    }

    #[test]
    fn test_resolver_list_simple() {
        let resolv = make_resolver();

        let arg1 = TyArgSpec::Ident(make_ident("byte"));
        let arg2 = TyArgSpec::IntLiteral(32);
        let spec = TyExprSpec::Complex(ComplexTySpec::new(make_ident("List"), vec![arg1, arg2]));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make List[byte, 32]");

        eprintln!("{ty:?}");
    }

    #[test]
    fn test_resolver_list_const() {
        let mut resolv = make_resolver();

        let const_name = make_ident("FOOBAR");
        resolv
            .decl_const(const_name.clone(), ConstValue::Int(1337))
            .expect("test: decl const");

        let arg1 = TyArgSpec::Ident(make_ident("byte"));
        let arg2 = TyArgSpec::Ident(const_name.clone());
        let spec = TyExprSpec::Complex(ComplexTySpec::new(make_ident("List"), vec![arg1, arg2]));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make List[byte, FOOBAR]");

        eprintln!("{ty:?}");
    }

    #[test]
    fn test_resolver_stablecontainer() {
        let mut resolv = make_resolver();

        let const_name = make_ident("FOOBAR");
        resolv
            .decl_const(const_name.clone(), ConstValue::Int(1337))
            .expect("test: decl const");

        let arg1 = TyArgSpec::Ident(const_name.clone());
        let spec = TyExprSpec::Complex(ComplexTySpec::new(
            make_ident("StableContainer"),
            vec![arg1],
        ));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make StableContainer[FOOBAR]");

        eprintln!("{ty:?}");
    }

    #[test]
    fn test_resolver_list_user() {
        let mut resolv = make_resolver();

        let const_name = make_ident("FOOBAR");
        resolv
            .decl_const(const_name.clone(), ConstValue::Int(1337))
            .expect("test: decl const");

        let ut_name = make_ident("UserType");
        resolv
            .decl_user_type(ut_name.clone())
            .expect("test: decl user type");

        let arg1 = TyArgSpec::Ident(ut_name.clone());
        let arg2 = TyArgSpec::IntLiteral(32);
        let spec = TyExprSpec::Complex(ComplexTySpec::new(make_ident("List"), vec![arg1, arg2]));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make List[UserType, 32]");

        eprintln!("{ty:?}");
    }
}
