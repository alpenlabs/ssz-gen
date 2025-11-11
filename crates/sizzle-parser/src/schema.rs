//! Schema definitions.

use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::{
    Identifier,
    ast::{AssignExpr, ClassDefEntry, Module, ModuleEntry},
    builtins,
    ty_resolver::{CrossModuleTypeMap, IdentTarget, ResolverError, TypeData, TypeResolver},
    tysys::{ConstValue, Ty, TyExpr},
};

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("unknown import '{0:?}'")]
    UnknownImport(Identifier),

    #[error("unknown import item '{0:?}' in '{1:?}'")]
    UnknownImportItem(Identifier, Identifier),

    #[error("unsupported import '{0:?}' in '{1:?}'")]
    UnsupportedImport(Identifier, Identifier),

    #[error("duplicate field name '{0:?}'")]
    DuplicateFieldName(Identifier),

    #[error("duplcate item name '{0:?}'")]
    DuplicateItemName(Identifier),

    #[error("found type cycle including type '{0:?}'")]
    CyclicTypedefs(Identifier),

    #[error("tyresolv: {0}")]
    Ty(#[from] ResolverError),
}

/// High level SSZ schema.
#[derive(Clone, Debug)]
pub struct SszSchema {
    constants: Vec<ConstDef>,
    classes: Vec<ClassDef>,
    aliases: Vec<AliasDef>,
}

impl SszSchema {
    /// All constants in the schema.
    pub fn constants(&self) -> &[ConstDef] {
        &self.constants
    }

    /// All classes in the schema.
    pub fn classes(&self) -> &[ClassDef] {
        &self.classes
    }

    /// All aliases in the schema.
    pub fn aliases(&self) -> &[AliasDef] {
        &self.aliases
    }
}

#[derive(Clone, Debug)]
pub struct ConstDef {
    name: Identifier,
    value: ConstValue,
}

impl ConstDef {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> &ConstValue {
        &self.value
    }
}

/// Type parameter in a generic class definition.
#[derive(Clone, Debug)]
pub struct TypeParam {
    name: Identifier,
    kind: TypeParamKind,
}

impl TypeParam {
    /// Name of the type parameter.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Kind of the type parameter (Type or Const).
    pub fn kind(&self) -> TypeParamKind {
        self.kind
    }
}

/// Kind of type parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeParamKind {
    /// Type variable (e.g., T, U, H)
    Type,
    /// Const variable (e.g., N for array sizes)
    Const,
}

/// Class definition.
#[derive(Clone, Debug)]
pub struct ClassDef {
    name: Identifier,
    type_params: Vec<TypeParam>,
    parent_ty: Ty,
    doc: Option<String>,
    doc_comment: Option<String>,
    pragmas: Vec<String>,
    fields: Vec<ClassFieldDef>,
}

impl ClassDef {
    /// Name of the class.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Type parameters of the class (empty for non-generic classes).
    pub fn type_params(&self) -> &[TypeParam] {
        &self.type_params
    }

    /// Parent type of the class.
    pub fn parent_ty(&self) -> &Ty {
        &self.parent_ty
    }

    /// Documentation string for the class.
    pub fn doc(&self) -> Option<&str> {
        self.doc.as_ref().map(|s| s.as_ref())
    }

    /// Doc comment for the class.
    pub fn doc_comment(&self) -> Option<&str> {
        self.doc_comment.as_ref().map(|s| s.as_ref())
    }

    /// Pragma comments for the class.
    pub fn pragmas(&self) -> &[String] {
        &self.pragmas
    }

    /// Fields of the class.
    pub fn fields(&self) -> &[ClassFieldDef] {
        &self.fields
    }
}

/// Class field definition.
#[derive(Clone, Debug)]
pub struct ClassFieldDef {
    name: Identifier,
    ty: Ty,
    doc_comment: Option<String>,
    pragmas: Vec<String>,
}

impl ClassFieldDef {
    /// Name of the field.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Type of the field.
    pub fn ty(&self) -> &Ty {
        &self.ty
    }

    /// Doc comment for the field.
    pub fn doc_comment(&self) -> Option<&str> {
        self.doc_comment.as_ref().map(|s| s.as_ref())
    }

    /// Pragma comments for the field.
    pub fn pragmas(&self) -> &[String] {
        &self.pragmas
    }
}

/// Type alias definition.
#[derive(Clone, Debug)]
pub struct AliasDef {
    name: Identifier,
    ty: Ty,
}

impl AliasDef {
    /// Name of the alias.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Concrete type that we are aliasing.
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
}

/// Converts a AST module to a full schema.
pub(crate) fn conv_module_to_schema<'a>(
    m: &Module,
    cross_module_types: &'a CrossModuleTypeMap<'a>,
) -> Result<(SszSchema, HashMap<Identifier, IdentTarget>), SchemaError> {
    let mut resolver = TypeResolver::new(cross_module_types);
    builtins::populate_builtin_types(&mut resolver);

    // Do a first pass to prepare the type resolver and abort if there's any obvious duplicates.
    let mut idents = HashMap::new();
    let mut constants = Vec::new();
    let mut class_defs = Vec::new();
    let mut aliases = Vec::new();

    for d in m.entries() {
        let name = d.name();
        if idents.contains_key(name) {
            return Err(SchemaError::DuplicateItemName(name.clone()));
        }

        match d {
            ModuleEntry::Assignment(def) => match def.value() {
                AssignExpr::Imported(imported) => {
                    let path = imported.module_path();
                    let Some(ident_targets) = cross_module_types.get(path) else {
                        return Err(SchemaError::UnknownImport(imported.module_name().clone()));
                    };

                    if ident_targets.is_external() {
                        // Treat all external types as non-const (we have no way of getting the
                        // value)
                        resolver.decl_user_type(name.clone())?;
                        idents.insert(name.clone(), IdentTarget::Ty(TypeData {}));
                        aliases.push(AliasDef {
                            name: name.clone(),
                            ty: Ty::Imported(
                                path.clone(),
                                imported.base_name().clone(),
                                imported.full_name(),
                            ),
                        });
                        continue;
                    }

                    let Some(ident_target) = ident_targets.get(imported.base_name()) else {
                        return Err(SchemaError::UnknownImportItem(
                            imported.module_name().clone(),
                            imported.base_name().clone(),
                        ));
                    };

                    match ident_target {
                        IdentTarget::Ty(_) => {
                            resolver.decl_user_type(name.clone())?;
                        }
                        IdentTarget::Const(const_value) => {
                            resolver.decl_const(name.clone(), const_value.clone())?;
                        }
                        _ => {
                            return Err(SchemaError::UnsupportedImport(
                                imported.module_name().clone(),
                                imported.base_name().clone(),
                            ));
                        }
                    }

                    idents.insert(name.clone(), ident_target.clone());
                    aliases.push(AliasDef {
                        name: name.clone(),
                        ty: Ty::Imported(
                            path.clone(),
                            imported.base_name().clone(),
                            imported.full_name(),
                        ),
                    });
                }

                // This is pretty straightforward, we just look up the identifier in-place.
                AssignExpr::Name(ident) => match resolver.resolve_ident_with_args(ident, None)? {
                    TyExpr::Ty(ty) => {
                        resolver.decl_user_type(name.clone())?;

                        idents.insert(name.clone(), IdentTarget::Ty(TypeData {}));
                        aliases.push(AliasDef {
                            name: name.clone(),
                            ty,
                        })
                    }
                    TyExpr::Int(v) => {
                        resolver.decl_const(name.clone(), v.clone())?;

                        idents.insert(name.clone(), IdentTarget::Const(v.clone()));
                        constants.push(ConstDef {
                            name: name.clone(),
                            value: v,
                        })
                    }
                    TyExpr::None => panic!("schema: assignment to None"),
                },

                // Complex types we can also handle.
                AssignExpr::Complex(complex) => {
                    // Handle as a regular type alias
                    let resolved_expr = resolver
                        .resolve_ident_with_args(complex.base_name(), Some(complex.args()))?;
                    let ty = match resolved_expr {
                        TyExpr::Ty(ty) => ty,
                        TyExpr::Int(_) => {
                            panic!("schema: resolver generated int for complex tyspec")
                        }
                        TyExpr::None => {
                            panic!("schema: resolver generated None for complex tyspec")
                        }
                    };

                    // We expose the type into the resolver as a unit type.
                    resolver.decl_user_type(name.clone())?;

                    idents.insert(name.clone(), IdentTarget::Ty(TypeData {}));
                    aliases.push(AliasDef {
                        name: name.clone(),
                        ty,
                    })
                }

                // Values are trivia.
                AssignExpr::Value(val) => {
                    resolver.decl_const(name.clone(), val.clone())?;
                    idents.insert(name.clone(), IdentTarget::Const(val.clone()));
                    constants.push(ConstDef {
                        name: name.clone(),
                        value: val.clone(),
                    })
                }
            },
            ModuleEntry::Class(def) => {
                // If this is a generic class, register as type constructor for resolution
                if !def.type_params().is_empty() {
                    use crate::ty_resolver::{CtorArg, CtorSig};
                    let sig = CtorSig::Fixed(
                        def.type_params()
                            .iter()
                            .map(|p| match p.kind {
                                crate::ast::TypeParamKind::Type => CtorArg::Ty,
                                crate::ast::TypeParamKind::Const => CtorArg::Int,
                            })
                            .collect(),
                    );
                    resolver.insert_type_ctor(name.clone(), sig.clone())?;
                    idents.insert(
                        name.clone(),
                        IdentTarget::TyCtor(crate::ty_resolver::TypeCtorData {
                            sig,
                            generic_class_def: None, // Don't store - we're not monomorphizing
                        }),
                    );
                } else {
                    // Non-generic class, register as regular type
                    resolver.decl_user_type(name.clone())?;
                    idents.insert(name.clone(), IdentTarget::Ty(TypeData {}));
                }

                // Add to class definitions for code generation
                class_defs.push(def);
            }
        }
    }

    // Now actually construct all the classes.
    let mut classes = Vec::new();
    for d in class_defs {
        classes.push(conv_classdef(d, &resolver)?);
    }

    // Check for cycles.
    let class_defs = classes
        .iter()
        .map(|d| (d.name(), d))
        .collect::<HashMap<_, _>>();
    for id in class_defs.keys() {
        trace_type_for_cycles(id, id, &class_defs)?;
    }

    // Create a the final schema.
    let schema = SszSchema {
        classes,
        constants,
        aliases,
    };

    Ok((schema, idents))
}

/// Convert AST ClassDefEntry to schema ClassDef.
fn conv_classdef<'a>(
    def: &ClassDefEntry,
    resolv: &'a TypeResolver<'a>,
) -> Result<ClassDef, SchemaError> {
    let mut field_names = HashSet::new();
    let mut fields = Vec::new();

    // Create a temporary resolver that includes type parameters in scope
    let mut temp_resolv = resolv.clone();

    // Add type parameters to the temporary resolver so they can be used in field types
    for tp in def.type_params() {
        match tp.kind {
            crate::ast::TypeParamKind::Type => {
                // Register type parameter as a simple type
                temp_resolv.decl_user_type(tp.name.clone())?;
            }
            crate::ast::TypeParamKind::Const => {
                // Register const parameter as a const with placeholder value
                // The actual value will be provided when the generic is instantiated
                temp_resolv.decl_const(tp.name.clone(), crate::tysys::ConstValue::Int(0))?;
            }
        }
    }

    for d in def.fields() {
        let name = d.name().clone();
        if field_names.contains(&name) {
            return Err(SchemaError::DuplicateFieldName(name));
        }

        field_names.insert(name.clone());

        // Use temp_resolv which has type parameters in scope
        let ty = temp_resolv.resolve_spec_as_ty(d.ty())?;
        fields.push(ClassFieldDef {
            name,
            ty,
            doc_comment: d.doc_comment().map(|s| s.to_owned()),
            pragmas: d.pragmas().to_vec(),
        })
    }

    // Convert AST type parameters to schema type parameters
    let type_params = def
        .type_params()
        .iter()
        .map(|tp| TypeParam {
            name: tp.name.clone(),
            kind: match tp.kind {
                crate::ast::TypeParamKind::Type => TypeParamKind::Type,
                crate::ast::TypeParamKind::Const => TypeParamKind::Const,
            },
        })
        .collect();

    Ok(ClassDef {
        name: def.name().clone(),
        type_params,
        parent_ty: resolv.resolve_spec_as_ty(def.parent_ty())?,
        doc: def.doc().map(|s| s.to_owned()),
        doc_comment: def.doc_comment().map(|s| s.to_owned()),
        pragmas: def.pragmas().to_vec(),
        fields,
    })
}

fn trace_type_for_cycles<'d>(
    ident: &Identifier,
    root: &Identifier,
    defs: &'d HashMap<&'d Identifier, &'d ClassDef>,
) -> Result<(), SchemaError> {
    let Some(def) = defs.get(ident) else {
        // We know that there's no undefined identifiers by this point, so it
        // not being here means it's a builtin type or a constant.
        return Ok(());
    };

    for f in def.fields() {
        for reffed_id in f.ty().iter_idents() {
            if reffed_id == root {
                return Err(SchemaError::CyclicTypedefs(root.clone()));
            }

            trace_type_for_cycles(reffed_id, root, defs)?;
        }
    }

    Ok(())
}
