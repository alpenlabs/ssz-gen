//! Type system resolver.
//!
//! Does the weird bookkeeping to figure out if schema types are well-formed.

use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;

use crate::{
    Identifier,
    ast::{ImportedComplexTySpec, ImportedTySpec, TyArgSpec, TyExprSpec},
    tysys::{ConstValue, Ty, TyExpr},
};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ResolverError {
    #[error("unknown import '{0:?}'")]
    UnknownImport(Identifier),

    #[error("unknown import item '{0:?}' in '{1:?}'")]
    UnknownImportItem(Identifier, Identifier),

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

/// Describes the type of a module.
pub(crate) enum ModuleTypeMap {
    External,
    Internal(HashMap<Identifier, IdentTarget>),
}

impl ModuleTypeMap {
    pub(crate) fn is_external(&self) -> bool {
        matches!(self, Self::External)
    }

    /// Returns true if this is an empty internal module (existing Rust module with no schema)
    pub(crate) fn is_empty_internal(&self) -> bool {
        match self {
            Self::External => false,
            Self::Internal(idents) => idents.is_empty(),
        }
    }

    pub(crate) fn get(&self, ident: &Identifier) -> Option<&IdentTarget> {
        match self {
            Self::External => None,
            Self::Internal(idents) => idents.get(ident),
        }
    }

    pub(crate) fn contains_key(&self, ident: &Identifier) -> bool {
        match self {
            Self::External => false,
            Self::Internal(idents) => idents.contains_key(ident),
        }
    }
}

pub(crate) type CrossModuleTypeMap<'a> = HashMap<PathBuf, ModuleTypeMap>;

#[derive(Clone)]
pub(crate) struct TypeResolver<'a> {
    /// Map of module paths to their respective resolvers.
    cross_module_types: &'a CrossModuleTypeMap<'a>,

    // TODO some way to express types that can be inherited from and types that can only be used as
    // a member
    /// Constants in the module scope.
    idents: HashMap<Identifier, IdentTarget>,

    /// Maps aliases to other things.
    aliases: HashMap<Identifier, AliasRef>,
}

impl<'a> TypeResolver<'a> {
    pub(crate) fn new(cross_module_types: &'a CrossModuleTypeMap<'a>) -> Self {
        Self {
            cross_module_types,
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
                    // Preserve constant name for codegen
                    Ok(TyExpr::ConstRef(ident.clone(), v.eval()))
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
                // Special case: Union can be used as a base class (no args) or type constructor
                // (with args)
                (CtorSig::VariableTy, None) if ident.0 == "Union" => {
                    Ok(TyExpr::Ty(Ty::Simple(ident.clone())))
                }
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
                                let expr = self.resolve_ident_with_args(arg_ident, None)?;
                                match expr {
                                    TyExpr::Ty(_) => expr,
                                    _ => {
                                        return Err(ResolverError::MismatchedArgKind(
                                            ident.clone(),
                                        ));
                                    }
                                }
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
                                    TyExpr::ConstRef(_, _) => {
                                        panic!("tyresolv: complex type resolved as const ref")
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
                                let expr = self.resolve_ident_with_args(arg_ident, None)?;
                                match expr {
                                    TyExpr::Int(v) => TyExpr::Int(v),
                                    TyExpr::ConstRef(id, value) => TyExpr::ConstRef(id, value),
                                    _ => {
                                        return Err(ResolverError::MismatchedArgKind(
                                            ident.clone(),
                                        ));
                                    }
                                }
                            }
                            (CtorArg::Int, TyArgSpec::Complex(_)) => {
                                return Err(ResolverError::MismatchedArgKind(ident.clone()));
                            }
                            (CtorArg::Int, TyArgSpec::IntLiteral(v)) => {
                                TyExpr::Int(ConstValue::Int(*v))
                            }
                            (CtorArg::Ty, TyArgSpec::Imported(imported)) => {
                                let Some(ident_targets) =
                                    self.cross_module_types.get(imported.module_path())
                                else {
                                    return Err(ResolverError::UnknownImport(
                                        imported.module_name().clone(),
                                    ));
                                };

                                // If external or empty internal (existing Rust module), skip
                                // validation
                                if ident_targets.is_external() || ident_targets.is_empty_internal()
                                {
                                    TyExpr::Ty(Ty::Imported(
                                        imported.module_path().clone(),
                                        imported.base_name().clone(),
                                        imported.full_name(),
                                    ))
                                } else {
                                    // Otherwise, we need to make sure it's a valid identifier.
                                    let Some(ident_target) =
                                        ident_targets.get(imported.base_name())
                                    else {
                                        return Err(ResolverError::UnknownImportItem(
                                            imported.module_name().clone(),
                                            imported.base_name().clone(),
                                        ));
                                    };

                                    match ident_target {
                                        IdentTarget::Ty(_) => TyExpr::Ty(Ty::Imported(
                                            imported.module_path().clone(),
                                            imported.base_name().clone(),
                                            imported.full_name(),
                                        )),
                                        _ => {
                                            return Err(ResolverError::MismatchedArgKind(
                                                ident.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                            (CtorArg::Ty, TyArgSpec::ImportedComplex(imported)) => {
                                let Some(ident_targets) =
                                    self.cross_module_types.get(imported.module_path())
                                else {
                                    return Err(ResolverError::UnknownImport(
                                        imported.module_name().clone(),
                                    ));
                                };

                                // If external or empty internal (existing Rust module), skip
                                // validation
                                if ident_targets.is_external() || ident_targets.is_empty_internal()
                                {
                                    let args = imported
                                        .args()
                                        .iter()
                                        .map(|arg| self.resolve_ty_arg_as_expr(arg))
                                        .collect::<Result<Vec<_>, _>>()?;
                                    TyExpr::Ty(Ty::ImportedComplex(
                                        imported.module_path().clone(),
                                        imported.base_name().clone(),
                                        imported.full_name(),
                                        args,
                                    ))
                                } else {
                                    // Otherwise, we need to make sure it's a valid identifier.
                                    let Some(ident_target) =
                                        ident_targets.get(imported.base_name())
                                    else {
                                        return Err(ResolverError::UnknownImportItem(
                                            imported.module_name().clone(),
                                            imported.base_name().clone(),
                                        ));
                                    };

                                    match ident_target {
                                        IdentTarget::Ty(_) => {
                                            let args = imported
                                                .args()
                                                .iter()
                                                .map(|arg| self.resolve_ty_arg_as_expr(arg))
                                                .collect::<Result<Vec<_>, _>>()?;
                                            TyExpr::Ty(Ty::ImportedComplex(
                                                imported.module_path().clone(),
                                                imported.base_name().clone(),
                                                imported.full_name(),
                                                args,
                                            ))
                                        }
                                        _ => {
                                            return Err(ResolverError::MismatchedArgKind(
                                                ident.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                            (CtorArg::Int, TyArgSpec::Imported(imported)) => {
                                let Some(ident_targets) =
                                    self.cross_module_types.get(imported.module_path())
                                else {
                                    return Err(ResolverError::UnknownImport(
                                        imported.module_name().clone(),
                                    ));
                                };

                                // If external or empty internal (existing Rust module), skip
                                // validation
                                if ident_targets.is_external() || ident_targets.is_empty_internal()
                                {
                                    TyExpr::Ty(Ty::Imported(
                                        imported.module_path().clone(),
                                        imported.base_name().clone(),
                                        imported.full_name(),
                                    ))
                                } else {
                                    // Otherwise, we need to make sure it's a valid identifier.
                                    let Some(ident_target) =
                                        ident_targets.get(imported.base_name())
                                    else {
                                        return Err(ResolverError::UnknownImportItem(
                                            imported.module_name().clone(),
                                            imported.base_name().clone(),
                                        ));
                                    };

                                    match ident_target {
                                        IdentTarget::Const(_) => TyExpr::Ty(Ty::Imported(
                                            imported.module_path().clone(),
                                            imported.base_name().clone(),
                                            imported.full_name(),
                                        )),
                                        _ => {
                                            return Err(ResolverError::MismatchedArgKind(
                                                ident.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                            (CtorArg::Int, TyArgSpec::ImportedComplex(_)) => {
                                return Err(ResolverError::MismatchedArgKind(ident.clone()));
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
                                TyExpr::ConstRef(_, _) => {
                                    panic!("tyresolv: complex type resolved to const ref")
                                }
                            },
                            TyArgSpec::IntLiteral(_) => {
                                return Err(ResolverError::MismatchedArgKind(ident.clone()));
                            }
                            TyArgSpec::Imported(imported) => {
                                let Some(ident_targets) =
                                    self.cross_module_types.get(imported.module_path())
                                else {
                                    return Err(ResolverError::UnknownImport(
                                        imported.module_name().clone(),
                                    ));
                                };

                                // If external or empty internal (existing Rust module), skip
                                // validation
                                if ident_targets.is_external() || ident_targets.is_empty_internal()
                                {
                                    TyExpr::Ty(Ty::Imported(
                                        imported.module_path().clone(),
                                        imported.base_name().clone(),
                                        imported.full_name(),
                                    ))
                                } else {
                                    // Otherwise, we need to make sure it's a valid identifier.
                                    let Some(ident_target) =
                                        ident_targets.get(imported.base_name())
                                    else {
                                        return Err(ResolverError::UnknownImportItem(
                                            imported.module_name().clone(),
                                            imported.base_name().clone(),
                                        ));
                                    };

                                    match ident_target {
                                        IdentTarget::Ty(_) => TyExpr::Ty(Ty::Imported(
                                            imported.module_path().clone(),
                                            imported.base_name().clone(),
                                            imported.full_name(),
                                        )),
                                        _ => {
                                            return Err(ResolverError::MismatchedArgKind(
                                                ident.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                            TyArgSpec::ImportedComplex(imported) => {
                                let Some(ident_targets) =
                                    self.cross_module_types.get(imported.module_path())
                                else {
                                    return Err(ResolverError::UnknownImport(
                                        imported.module_name().clone(),
                                    ));
                                };

                                // If external or empty internal (existing Rust module), skip
                                // validation
                                if ident_targets.is_external() || ident_targets.is_empty_internal()
                                {
                                    let args = imported
                                        .args()
                                        .iter()
                                        .map(|arg| self.resolve_ty_arg_as_expr(arg))
                                        .collect::<Result<Vec<_>, _>>()?;
                                    TyExpr::Ty(Ty::ImportedComplex(
                                        imported.module_path().clone(),
                                        imported.base_name().clone(),
                                        imported.full_name(),
                                        args,
                                    ))
                                } else {
                                    // Otherwise, we need to make sure it's a valid identifier.
                                    let Some(ident_target) =
                                        ident_targets.get(imported.base_name())
                                    else {
                                        return Err(ResolverError::UnknownImportItem(
                                            imported.module_name().clone(),
                                            imported.base_name().clone(),
                                        ));
                                    };

                                    match ident_target {
                                        IdentTarget::Ty(_) => {
                                            let args = imported
                                                .args()
                                                .iter()
                                                .map(|arg| self.resolve_ty_arg_as_expr(arg))
                                                .collect::<Result<Vec<_>, _>>()?;
                                            TyExpr::Ty(Ty::ImportedComplex(
                                                imported.module_path().clone(),
                                                imported.base_name().clone(),
                                                imported.full_name(),
                                                args,
                                            ))
                                        }
                                        _ => {
                                            return Err(ResolverError::MismatchedArgKind(
                                                ident.clone(),
                                            ));
                                        }
                                    }
                                }
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

    fn resolve_imported_ty_spec(&self, imported: &ImportedTySpec) -> Result<Ty, ResolverError> {
        let Some(ident_targets) = self.cross_module_types.get(imported.module_path()) else {
            return Err(ResolverError::UnknownImport(imported.module_name().clone()));
        };

        // If external or empty internal (existing Rust module), skip validation
        if ident_targets.is_external() || ident_targets.is_empty_internal() {
            return Ok(Ty::Imported(
                imported.module_path().clone(),
                imported.base_name().clone(),
                imported.full_name(),
            ));
        }

        // Otherwise, we need to make sure it's a valid identifier.
        if !ident_targets.contains_key(imported.base_name()) {
            return Err(ResolverError::UnknownImportItem(
                imported.module_name().clone(),
                imported.base_name().clone(),
            ));
        }

        Ok(Ty::Imported(
            imported.module_path().clone(),
            imported.base_name().clone(),
            imported.full_name(),
        ))
    }

    fn resolve_imported_complex_ty_spec(
        &self,
        imported: &ImportedComplexTySpec,
    ) -> Result<Ty, ResolverError> {
        let Some(ident_targets) = self.cross_module_types.get(imported.module_path()) else {
            return Err(ResolverError::UnknownImport(imported.module_name().clone()));
        };

        // If external or empty internal (existing Rust module), skip validation
        if ident_targets.is_external() || ident_targets.is_empty_internal() {
            let args = imported
                .args()
                .iter()
                .map(|arg| self.resolve_ty_arg_as_expr(arg))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(Ty::ImportedComplex(
                imported.module_path().clone(),
                imported.base_name().clone(),
                imported.full_name(),
                args,
            ));
        }

        // Otherwise, we need to make sure it's a valid identifier.
        if !ident_targets.contains_key(imported.base_name()) {
            return Err(ResolverError::UnknownImportItem(
                imported.module_name().clone(),
                imported.base_name().clone(),
            ));
        }

        let args = imported
            .args()
            .iter()
            .map(|arg| self.resolve_ty_arg_as_expr(arg))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Ty::ImportedComplex(
            imported.module_path().clone(),
            imported.base_name().clone(),
            imported.full_name(),
            args,
        ))
    }

    fn resolve_ty_arg_as_expr(&self, arg: &TyArgSpec) -> Result<TyExpr, ResolverError> {
        match arg {
            TyArgSpec::Ident(ident) => self.resolve_ident_with_args(ident, None),
            TyArgSpec::Complex(complex) => {
                self.resolve_ident_with_args(complex.base_name(), Some(complex.args()))
            }
            TyArgSpec::Imported(imported) => {
                let ty = self.resolve_imported_ty_spec(imported)?;
                Ok(TyExpr::Ty(ty))
            }
            TyArgSpec::ImportedComplex(imported) => {
                let ty = self.resolve_imported_complex_ty_spec(imported)?;
                Ok(TyExpr::Ty(ty))
            }
            TyArgSpec::IntLiteral(v) => Ok(TyExpr::Int(ConstValue::Int(*v))),
            TyArgSpec::None => Ok(TyExpr::None),
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
            TyExprSpec::None => TyExpr::None,
            TyExprSpec::Imported(imported) => {
                let ty = self.resolve_imported_ty_spec(imported)?;
                TyExpr::Ty(ty)
            }
            TyExprSpec::ImportedComplex(imported) => {
                let ty = self.resolve_imported_complex_ty_spec(imported)?;
                TyExpr::Ty(ty)
            }
        };

        // And then just make sure it's a type (not a const or None).
        match expr {
            TyExpr::Ty(ty) => Ok(ty),
            TyExpr::None => {
                // None is valid for Union unit variants, but not as a type argument
                // Return a placeholder error since we shouldn't reach here for valid union variants
                Err(ResolverError::MismatchedArgKind(
                    Identifier::try_from("null").expect("valid identifier"),
                ))
            }
            TyExpr::Int(_) | TyExpr::ConstRef(_, _) => {
                Err(ResolverError::MismatchedArgKind(spec.base_name().clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{CrossModuleTypeMap, TypeResolver};
    use crate::{
        Identifier,
        ast::{ComplexTySpec, TyArgSpec, TyExprSpec},
        builtins,
        tysys::ConstValue,
    };

    fn make_ident(s: &str) -> Identifier {
        Identifier::try_from(s.to_owned()).expect("test: make ident")
    }

    fn make_resolver<'a>(cross_module_types: &'a CrossModuleTypeMap<'a>) -> TypeResolver<'a> {
        let mut resolv = TypeResolver::new(cross_module_types);
        builtins::populate_builtin_types(&mut resolv);
        resolv
    }

    #[test]
    fn test_resolver_simple() {
        let cross_module_types = HashMap::new();
        let resolv = make_resolver(&cross_module_types);

        let spec = TyExprSpec::Simple(make_ident("Container"));

        let ty = resolv
            .resolve_spec_as_ty(&spec)
            .expect("test: make Container");

        eprintln!("{ty:?}");
    }

    #[test]
    fn test_resolver_list_simple() {
        let cross_module_types = HashMap::new();
        let resolv = make_resolver(&cross_module_types);

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
        let cross_module_types = HashMap::new();
        let mut resolv = make_resolver(&cross_module_types);

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
        let cross_module_types = HashMap::new();
        let mut resolv = make_resolver(&cross_module_types);

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
        let cross_module_types = HashMap::new();
        let mut resolv = make_resolver(&cross_module_types);

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
