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

/// Class definition.
#[derive(Clone, Debug)]
pub struct ClassDef {
    name: Identifier,
    parent_ty: Ty,
    fields: Vec<ClassFieldDef>,
}

impl ClassDef {
    /// Name of the class.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Parent type of the class.
    pub fn parent_ty(&self) -> &Ty {
        &self.parent_ty
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
                        // Treat all external types as non-const (we have no way of getting the value)
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
                resolver.decl_user_type(name.clone())?;
                idents.insert(name.clone(), IdentTarget::Ty(TypeData {}));
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

fn conv_classdef<'a>(
    def: &ClassDefEntry,
    resolv: &'a TypeResolver<'a>,
) -> Result<ClassDef, SchemaError> {
    let mut field_names = HashSet::new();
    let mut fields = Vec::new();

    for d in def.fields() {
        let name = d.name().clone();
        if field_names.contains(&name) {
            return Err(SchemaError::DuplicateFieldName(name));
        }

        field_names.insert(name.clone());

        let ty = resolv.resolve_spec_as_ty(d.ty())?;
        fields.push(ClassFieldDef { name, ty })
    }

    Ok(ClassDef {
        name: def.name().clone(),
        parent_ty: resolv.resolve_spec_as_ty(def.parent_ty())?,
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
