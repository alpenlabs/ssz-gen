//! Type resolver for the SSZ type system. It handles the resolution
//! of types, classes, and base classes in the SSZ schema, tracking custom type definitions
//! and unions. It provides functionality to:
//!
//! - Resolve types to their concrete Rust representations
//! - Resolve class definitions and inheritance
//! - Track and generate union types
//! - Manage custom type definitions

use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use sizzle_parser::{
    Identifier,
    tysys::{Ty, TyExpr},
};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, PathArguments, parse_quote};

use super::{BaseClass, ClassDef, ClassDefinition, SizeExpr, TypeDefinition, TypeResolution};
use crate::types::TypeResolutionKind;

/// Extract a simple type name from a TypeResolution for use as a variant name.
/// Returns None if the type doesn't have a simple extractable name (e.g., for None or complex
/// types).
fn extract_variant_name(ty_resolution: &TypeResolution) -> Option<String> {
    match &ty_resolution.ty {
        Some(syn::Type::Path(type_path)) if type_path.qself.is_none() => {
            // Get the last segment of the path (e.g., "Foo" from "crate::module::Foo")
            type_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
        }
        _ => None,
    }
}

/// Extract a variant name from a TyExpr (original type expression from schema).
/// This preserves type alias names instead of resolving to underlying types.
fn extract_variant_name_from_ty_expr(ty_expr: &TyExpr) -> Option<String> {
    match ty_expr {
        TyExpr::Ty(ty) => {
            // Extract identifier from Ty - get the base name
            Some(ty.base_name().0.clone())
        }
        _ => None,
    }
}

/// Converts a TyExpr to a SizeExpr, extracting size information for type parameters.
/// For imported constants or other complex expressions, falls back to the resolved value.
fn ty_expr_to_size_expr(expr: &TyExpr, resolved: &TypeResolution) -> SizeExpr {
    match expr {
        TyExpr::Int(val) => SizeExpr::Literal(val.eval()),
        TyExpr::ConstRef(ident, value) => SizeExpr::ConstRef(ident.0.clone(), *value),
        _ => {
            // For imported constants or other expressions, extract the resolved value
            match &resolved.resolution {
                TypeResolutionKind::Constant(value) => SizeExpr::Literal(*value),
                _ => panic!(
                    "Expected size parameter to resolve to a constant, got: {:?}",
                    resolved.resolution
                ),
            }
        }
    }
}

/// Converts a primitive type name into a Rust syn::Type
///
/// # Arguments
///
/// * `base_name` - The name of the primitive type (e.g., "bool", "u32")
///
/// # Returns
///
/// A syn::Type representing the Rust primitive type
pub fn primitive_rust_type(base_name: &str) -> Box<syn::Type> {
    Box::new(syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path::from(syn::Ident::new(base_name, proc_macro2::Span::call_site())),
    }))
}

/// Type resolver for SSZ type system
///
/// Manages the resolution of types, classes, and base classes in the SSZ type system.
/// Tracks custom type definitions and unions.
#[derive(Debug)]
pub struct TypeResolver<'a> {
    /// Map of module paths to their schemas
    pub resolvers: &'a RefCell<HashMap<PathBuf, Self>>,

    /// Map of type names to their definitions
    pub types: HashMap<String, TypeDefinition>,
    /// Map of class names to their definitions
    pub classes: HashMap<String, ClassDefinition>,
    /// Map of base class names to their implementations
    pub base_classes: HashMap<String, BaseClass>,
    /// Tracker for generated union type definitions
    pub union_tracker: Rc<RefCell<HashMap<String, TokenStream>>>,
}

impl<'a> TypeResolver<'a> {
    /// Creates a new TypeResolver with empty maps
    ///
    /// # Returns
    ///
    /// A new TypeResolver instance with empty maps
    pub fn new(resolvers: &'a RefCell<HashMap<PathBuf, Self>>) -> Self {
        Self {
            resolvers,
            types: HashMap::new(),
            classes: HashMap::new(),
            base_classes: HashMap::new(),
            union_tracker: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Creates a TypeResolution for a unit variant (no associated data)
    ///
    /// # Returns
    ///
    /// A TypeResolution representing a unit variant in a Union
    pub fn make_unit_variant_type(&self) -> TypeResolution {
        TypeResolution {
            ty: None,
            resolution: TypeResolutionKind::None,
        }
    }

    /// Creates a new TypeResolver with all built-in types and classes pre-registered
    ///
    /// # Returns
    ///
    /// A new TypeResolver instance with all built-in types and classes registered
    pub fn new_with_builtins(resolvers: &'a RefCell<HashMap<PathBuf, Self>>) -> Self {
        let mut resolver = Self {
            resolvers,
            types: HashMap::new(),
            classes: HashMap::new(),
            base_classes: HashMap::new(),
            union_tracker: Rc::new(RefCell::new(HashMap::new())),
        };

        // Built-in types
        resolver
            .types
            .insert("boolean".to_string(), TypeDefinition::Boolean);
        for i in [8, 16, 32, 64, 128, 256] {
            let uint_name = format!("uint{i}");
            resolver.types.insert(uint_name, TypeDefinition::UInt(i));
        }
        resolver
            .types
            .insert("bit".to_string(), TypeDefinition::Boolean);
        resolver
            .types
            .insert("byte".to_string(), TypeDefinition::UInt(8));
        resolver
            .types
            .insert("Vector".to_string(), TypeDefinition::Vector);
        resolver
            .types
            .insert("List".to_string(), TypeDefinition::List);
        resolver
            .types
            .insert("Bitvector".to_string(), TypeDefinition::Bitvector);
        resolver
            .types
            .insert("Bitlist".to_string(), TypeDefinition::Bitlist);
        resolver
            .types
            .insert("Optional".to_string(), TypeDefinition::Optional);
        resolver
            .types
            .insert("Union".to_string(), TypeDefinition::Union);
        for i in 1..=64 {
            let bytes_name = format!("Bytes{i}");
            resolver.types.insert(bytes_name, TypeDefinition::Bytes(i));
        }

        // Built-in classes
        resolver
            .base_classes
            .insert("Container".to_string(), BaseClass::Container);
        resolver.base_classes.insert(
            "StableContainer".to_string(),
            BaseClass::StableContainer(None),
        );
        resolver
            .base_classes
            .insert("Profile".to_string(), BaseClass::Profile(None));
        resolver
            .base_classes
            .insert("Union".to_string(), BaseClass::Union);
        resolver
            .classes
            .insert("Container".to_string(), ClassDefinition::Container);
        resolver.classes.insert(
            "StableContainer".to_string(),
            ClassDefinition::StableContainer,
        );
        resolver
            .classes
            .insert("Profile".to_string(), ClassDefinition::Profile);
        resolver
            .classes
            .insert("Union".to_string(), ClassDefinition::Union);

        resolver
    }

    /// Resolves a type to its concrete TypeResolution
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to resolve
    /// * `is_assignment` - Whether the type is being assigned an alias or not
    ///
    /// # Returns
    ///
    /// A TypeResolution representing the resolved type, or TypeResolution::None if unresolved
    pub fn resolve_type(&self, ty: &Ty, alias_ident: Option<&syn::Ident>) -> TypeResolution {
        // Check if the type is imported
        if let Ty::Imported(path, name, _) = ty {
            return self.resolve_imported_type(path, name);
        }
        if let Ty::ImportedComplex(path, name, _, args) = ty {
            return self.resolve_imported_complex_type(path, name, args);
        }

        // Check if the type is a base class (Container, StableContainer, Profile or aliases to
        // them)
        let base_class = self.resolve_base_class(ty);
        if let Some(base_class) = base_class {
            return base_class;
        }

        // Extract the type arguments
        let args = match ty {
            Ty::Imported(_, _, _) | Ty::ImportedComplex(_, _, _, _) | Ty::Simple(_) => vec![],
            Ty::Complex(_, args) => {
                let mut resolved_args = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    let ty_resolved = self.resolve_type_expr(arg);
                    match ty_resolved.resolution {
                        TypeResolutionKind::Unresolved => {
                            return TypeResolution {
                                ty: None,
                                resolution: TypeResolutionKind::Unresolved,
                            };
                        }
                        TypeResolutionKind::BaseClass(_) => {
                            panic!("BaseClass in type arguments are not allowed")
                        }
                        _ => resolved_args.push(ty_resolved),
                    }
                }
                resolved_args
            }
        };

        // We disallow Unions if they're being used "anonymously" (i.e. not assigned to an alias)
        // Unless it's a Union[None, T]
        if alias_ident.is_none()
            && ty.base_name().0 == "Union"
            && !(args.len() == 2 && args[0].resolution == TypeResolutionKind::None)
        {
            panic!("Unions cannot be used anonymously unless they are Union[None, T]");
        }

        // Resolve the type definition using the type arguments
        let type_def = self.types.get(ty.base_name().0.as_str());
        let original_args = match ty {
            Ty::Complex(_, args) => args.as_slice(),
            _ => &[],
        };
        match type_def {
            Some(def) => self.resolve_type_definition(def, args, original_args, alias_ident),
            None => TypeResolution {
                ty: None,
                resolution: TypeResolutionKind::Unresolved,
            },
        }
    }

    /// Resolves a type expression to its concrete TypeResolution
    ///
    /// # Arguments
    ///
    /// * `ty_expr` - The type expression to resolve
    ///
    /// # Returns
    ///
    /// A TypeResolution representing the resolved type expression
    fn resolve_type_expr(&self, ty_expr: &TyExpr) -> TypeResolution {
        match ty_expr {
            TyExpr::Ty(ty) => self.resolve_type(ty, None),
            TyExpr::Int(int) => TypeResolution {
                ty: None,
                resolution: TypeResolutionKind::Constant(int.eval()),
            },
            TyExpr::ConstRef(_ident, value) => {
                // Resolve to constant value; name is preserved in TyExpr for codegen
                TypeResolution {
                    ty: None,
                    resolution: TypeResolutionKind::Constant(*value),
                }
            }
            TyExpr::None => TypeResolution {
                ty: None,
                resolution: TypeResolutionKind::None,
            },
        }
    }

    /// Resolves a type to a base class if applicable
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to resolve
    ///
    /// # Returns
    ///
    /// Some(TypeResolution::BaseClass) if the type resolves to a base class, None otherwise
    fn resolve_base_class(&self, ty: &Ty) -> Option<TypeResolution> {
        // Base classes are only valid when used as simple types (no arguments)
        // e.g., `class Name(Union):` not `Union[Type1, Type2]`
        // For Union specifically, if it has arguments, it's a type constructor, not a base class
        if let Ty::Complex(_, _) = ty
            && ty.base_name().0 == "Union"
        {
            // Union[Type1, Type2] is a type constructor, not a base class
            return None;
        }

        let base_class = self.base_classes.get(ty.base_name().0.as_str());
        base_class.map(|base_class| match base_class {
            BaseClass::Container => TypeResolution {
                ty: None,
                resolution: TypeResolutionKind::BaseClass(BaseClass::Container),
            },
            BaseClass::StableContainer(max) => {
                let max = max.unwrap_or_else(|| match ty {
                    Ty::Imported(_, _, _) | Ty::Simple(_) => {
                        panic!("Stable container must have a max field count as first argument")
                    }
                    Ty::Complex(_, args) | Ty::ImportedComplex(_, _, _, args) => match args.first()
                    {
                        Some(TyExpr::Int(int)) => int.eval(),
                        Some(TyExpr::ConstRef(_, value)) => *value,
                        _ => {
                            panic!("Stable container must have a max field count as first argument")
                        }
                    },
                });
                assert!(max > 0, "Stable container must have a max field count > 0");
                TypeResolution {
                    ty: None,
                    resolution: TypeResolutionKind::BaseClass(BaseClass::StableContainer(Some(
                        max,
                    ))),
                }
            }
            BaseClass::Profile(tuple) => {
                let (name, max) = match tuple.clone() {
                    Some(tuple) => tuple,
                    None => match ty {
                        Ty::Imported(_, _, _) | Ty::Simple(_) => {
                            panic!("Profile must inherit from a stable container")
                        }
                        Ty::Complex(_, args) | Ty::ImportedComplex(_, _, _, args) => {
                            match args.first() {
                                Some(TyExpr::Ty(ty)) => {
                                    let name = ty.base_name().0.clone();
                                    let class_def = self.resolve_class(ty);
                                    if class_def.is_none() {
                                        return TypeResolution {
                                            ty: None,
                                            resolution: TypeResolutionKind::Unresolved,
                                        };
                                    }
                                    let class_def = class_def.unwrap();
                                    if let BaseClass::StableContainer(max) = class_def.base {
                                        (name, max.unwrap())
                                    } else {
                                        panic!(
                                            "Expected profile to inherit from a stable container"
                                        );
                                    }
                                }
                                _ => panic!("Profile must inherit from a class"),
                            }
                        }
                    },
                };
                TypeResolution {
                    ty: None,
                    resolution: TypeResolutionKind::BaseClass(BaseClass::Profile(Some((
                        name, max,
                    )))),
                }
            }
            BaseClass::Union => TypeResolution {
                ty: None,
                resolution: TypeResolutionKind::BaseClass(BaseClass::Union),
            },
        })
    }

    /// Resolves a type definition with its arguments to a concrete TypeResolution
    ///
    /// # Arguments
    ///
    /// * `def` - The type definition to resolve
    /// * `args` - The resolved type arguments
    /// * `original_args` - The original TyExpr arguments (for preserving constant names)
    ///
    /// # Returns
    ///
    /// A TypeResolution representing the resolved type definition
    fn resolve_type_definition(
        &self,
        def: &TypeDefinition,
        args: Vec<TypeResolution>,
        original_args: &[TyExpr],
        alias_ident: Option<&syn::Ident>,
    ) -> TypeResolution {
        let mut resolved_ty = None;
        let resolution = match def {
            TypeDefinition::Boolean => TypeResolutionKind::Boolean,
            TypeDefinition::UInt(size) => TypeResolutionKind::UInt(*size),
            TypeDefinition::Vector => {
                let size_expr = ty_expr_to_size_expr(&original_args[1], &args[1]);
                TypeResolutionKind::Vector(Box::new(args[0].clone()), size_expr)
            }
            TypeDefinition::List => {
                let size_expr = ty_expr_to_size_expr(&original_args[1], &args[1]);
                TypeResolutionKind::List(Box::new(args[0].clone()), size_expr)
            }
            TypeDefinition::Bitvector => {
                let size_expr = ty_expr_to_size_expr(&original_args[0], &args[0]);
                TypeResolutionKind::Bitvector(size_expr)
            }
            TypeDefinition::Bitlist => {
                let size_expr = ty_expr_to_size_expr(&original_args[0], &args[0]);
                TypeResolutionKind::Bitlist(size_expr)
            }
            TypeDefinition::Optional => TypeResolutionKind::Optional(Box::new(args[0].clone())),
            TypeDefinition::Union => {
                // Special case for Union[None, T]
                if args.len() == 2 && args[0].resolution == TypeResolutionKind::None {
                    TypeResolutionKind::Option(Box::new(args[1].clone()))
                } else {
                    let ident = alias_ident.unwrap().clone();
                    let ident_str = ident.to_string();

                    // Generate the enum variants using type names when available, falling back to
                    // Selector{i}
                    let variants: Vec<syn::Variant> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            // Try to extract a meaningful name from the type, otherwise use
                            // Selector{i}
                            let variant_name =
                                extract_variant_name(ty).unwrap_or_else(|| format!("Selector{i}"));
                            let ident =
                                syn::Ident::new(&variant_name, proc_macro2::Span::call_site());
                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    if i == 0 {
                                        parse_quote!(#ident)
                                    } else {
                                        panic!(
                                            "None is only allowed as the first variant in a Union"
                                        )
                                    }
                                }
                                _ => {
                                    let ty = ty.unwrap_type();
                                    parse_quote!(#ident(#ty))
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    // Generate TreeHash match arms for owned union enum
                    let owned_tree_hash_arms: Vec<TokenStream> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let selector_value = i as u8;
                            let variant_name = extract_variant_name(ty)
                                .unwrap_or_else(|| format!("Selector{i}"));
                            let variant_ident = Ident::new(&variant_name, Span::call_site());

                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    quote! {
                                        #ident::#variant_ident => {
                                            // For empty variants, use precomputed zero hash
                                            let zero_root = H::get_zero_hash(0);
                                            tree_hash::mix_in_selector_with_hasher::<H>(
                                                &zero_root,
                                                #selector_value
                                            ).expect("valid selector")
                                        }
                                    }
                                }
                                _ => {
                                    quote! {
                                        #ident::#variant_ident(inner) => {
                                            let root = <_ as tree_hash::TreeHash<H>>::tree_hash_root(inner);
                                            tree_hash::mix_in_selector_with_hasher::<H>(
                                                &root,
                                                #selector_value
                                            ).expect("valid selector")
                                        }
                                    }
                                }
                            }
                        })
                        .collect();

                    // Generate owned union enum with manual generic TreeHash impl
                    self.union_tracker.borrow_mut().insert(
                        ident_str.clone(),
                        quote! {
                            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
                            #[ssz(enum_behaviour="union")]
                            pub enum #ident {
                            #(#variants),*
                            }

                            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ident {
                                fn tree_hash_type() -> tree_hash::TreeHashType {
                                    tree_hash::TreeHashType::Container
                                }

                                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                                    unreachable!("Union should never be packed")
                                }

                                fn tree_hash_packing_factor() -> usize {
                                    unreachable!("Union should never be packed")
                                }

                                fn tree_hash_root(&self) -> H::Output {
                                    match self {
                                        #(#owned_tree_hash_arms,)*
                                    }
                                }
                            }
                        },
                    );

                    let ref_ident = Ident::new(&format!("{}Ref", ident_str), Span::call_site());

                    let mut view_type_aliases: Vec<TokenStream> = Vec::new();
                    let mut variant_view_types: Vec<(String, TokenStream)> = Vec::new();

                    for (i, ty) in args.iter().enumerate() {
                        if self.handle_none_variant(ty, &mut variant_view_types) {
                            continue;
                        }

                        let variant_name_from_expr = original_args
                            .get(i)
                            .and_then(extract_variant_name_from_ty_expr);

                        let underlying_type_name = self.extract_underlying_type_name(ty, false);
                        let underlying_view_ty = ty.to_view_type();
                        let underlying_name_opt = underlying_type_name.as_ref();
                        let underlying_type_resolution =
                            self.get_custom_type_resolution(underlying_name_opt);

                        let needs_alias = variant_name_from_expr.is_some()
                            && underlying_name_opt.is_some()
                            && variant_name_from_expr.as_ref() != underlying_name_opt
                            && matches!(ty.resolution, TypeResolutionKind::Class(_))
                            && underlying_type_resolution.is_some();

                        let underlying_view_ty = if needs_alias {
                            underlying_type_resolution.unwrap().to_view_type()
                        } else {
                            underlying_view_ty
                        };

                        let variant_name_from_resolved = extract_variant_name(ty);
                        let variant_name = variant_name_from_expr
                            .as_ref()
                            .or(variant_name_from_resolved.as_ref())
                            .cloned()
                            .unwrap_or_else(|| format!("Selector{i}"));

                        self.add_variant_view_type(
                            &mut view_type_aliases,
                            &mut variant_view_types,
                            &variant_name,
                            needs_alias,
                            quote! { #underlying_view_ty },
                        );
                    }

                    let selector_methods = self.generate_union_selector_methods(
                        &ident_str,
                        &args,
                        &variant_view_types,
                    );

                    let variant_names: Vec<String> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            extract_variant_name(ty).unwrap_or_else(|| format!("Selector{i}"))
                        })
                        .collect();
                    let to_owned_arms =
                        self.generate_union_to_owned_arms(&ident, &args, &variant_names);

                    let tree_hash_arms = self.generate_union_tree_hash_arms(&args);

                    let view_union_code = self.generate_union_view_struct_impl(
                        ref_ident,
                        &ident,
                        view_type_aliases,
                        selector_methods,
                        to_owned_arms,
                        tree_hash_arms,
                    );

                    self.union_tracker
                        .borrow_mut()
                        .insert(format!("{}Ref", ident_str), view_union_code);

                    TypeResolutionKind::Union(ident_str, args)
                }
            }
            TypeDefinition::Bytes(size) => TypeResolutionKind::Bytes(*size),
            TypeDefinition::CustomType(res) => {
                resolved_ty = res.ty.clone();
                res.resolution.clone()
            }
        };

        TypeResolution {
            ty: resolved_ty,
            resolution,
        }
    }

    /// Resolves a class definition for a given type
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to resolve as a class
    ///
    /// # Returns
    ///
    /// Some(ClassDef) if the type resolves to a class, None otherwise
    pub fn resolve_class(&self, ty: &Ty) -> Option<ClassDef> {
        if let Ty::Imported(path, name, _) = ty {
            let resolvers = self.resolvers.borrow();
            let resolver = resolvers
                .get(path)
                .expect("Cannot get class definitions from external crates");
            return resolver.resolve_class(&Ty::Simple(name.clone()));
        }
        if let Ty::ImportedComplex(path, name, _, args) = ty {
            let resolvers = self.resolvers.borrow();
            let resolver = resolvers
                .get(path)
                .expect("Cannot get class definitions from external crates");
            return resolver.resolve_class(&Ty::Complex(name.clone(), args.clone()));
        }

        let class_def = self.classes.get(ty.base_name().0.as_str())?;

        let args = match ty {
            Ty::Simple(_) | Ty::Imported(_, _, _) => vec![],
            Ty::Complex(_, args) | Ty::ImportedComplex(_, _, _, args) => args.clone(),
        };

        Some(self.resolve_class_definition(class_def, &args))
    }

    /// Resolves a class definition with its arguments to a concrete ClassDef
    ///
    /// # Arguments
    ///
    /// * `def` - The class definition to resolve
    /// * `args` - The type arguments for the definition
    ///
    /// # Returns
    ///
    /// A ClassDef representing the resolved class definition
    fn resolve_class_definition(&self, def: &ClassDefinition, args: &[TyExpr]) -> ClassDef {
        match def {
            ClassDefinition::Container => ClassDef {
                base: BaseClass::Container,
                fields: vec![],
                field_tokens: vec![],
                field_index: HashMap::new(),
                pragmas: vec![],
                doc_comment: None,
                doc: None,
            },
            ClassDefinition::StableContainer => {
                let max = match args.first() {
                    Some(TyExpr::Int(int)) => int.eval(),
                    Some(TyExpr::ConstRef(_, value)) => *value,
                    _ => panic!(
                        "Expected stable container to have a max field count as first argument"
                    ),
                };

                ClassDef {
                    base: BaseClass::StableContainer(Some(max)),
                    fields: vec![],
                    field_tokens: vec![],
                    field_index: HashMap::new(),
                    pragmas: vec![],
                    doc_comment: None,
                    doc: None,
                }
            }
            ClassDefinition::Profile => {
                let name;
                let class_def = match args.first() {
                    Some(TyExpr::Ty(inner)) => {
                        name = inner.base_name().0.clone();
                        let class_def = self.resolve_class(inner).unwrap();
                        if !class_def.is_stable_container() {
                            panic!("Expected profile to inherit from a stable container");
                        }
                        class_def
                    }
                    _ => panic!("Expected profile to inherit from a class"),
                };

                if let BaseClass::StableContainer(max) = class_def.base {
                    let max = max.unwrap();
                    ClassDef {
                        base: BaseClass::Profile(Some((name, max))),
                        fields: class_def.fields,
                        field_tokens: class_def.field_tokens,
                        field_index: class_def.field_index,
                        pragmas: class_def.pragmas,
                        doc_comment: class_def.doc_comment,
                        doc: class_def.doc,
                    }
                } else {
                    panic!("Expected profile to inherit from a stable container");
                }
            }
            ClassDefinition::Union => ClassDef {
                base: BaseClass::Union,
                fields: vec![],
                field_tokens: vec![],
                field_index: HashMap::new(),
                pragmas: vec![],
                doc_comment: None,
                doc: None,
            },
            ClassDefinition::Custom(class_def) => class_def.clone(),
        }
    }

    /// Resolves a type and adds it to the resolver's type registry with an alias
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to resolve
    /// * `alias_ident` - The identifier to use as an alias for the resolved type
    ///
    /// # Returns
    ///
    /// A TypeResolution representing the resolved type
    pub fn resolve_type_and_add(&mut self, ty: &Ty, alias_ident: &syn::Ident) -> TypeResolution {
        let resolved = self.resolve_type(ty, Some(alias_ident));

        let alias_str = alias_ident.to_string();
        if resolved.is_type() && !self.types.contains_key(&alias_str) {
            // Add the new type to the types map so it can be referenced by other types
            self.types.insert(
                alias_str.clone(),
                TypeDefinition::CustomType(Box::new(TypeResolution {
                    ty: Some(syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(syn::Ident::new(
                            &alias_str,
                            proc_macro2::Span::call_site(),
                        )),
                    })),
                    resolution: resolved.resolution.clone(),
                })),
            );
        };

        if resolved.is_base_class() && !self.base_classes.contains_key(&alias_str) {
            // Add the base class to the base classes map so it can be aliased if needed
            let base_class = resolved.clone().unwrap_base_class();
            self.base_classes
                .insert(alias_str.clone(), base_class.clone());

            // Add the base class as an empty version of the base class itself for inheritance
            // purposes
            let class_def = match &base_class {
                BaseClass::Container => ClassDefinition::Custom(ClassDef {
                    base: BaseClass::Container,
                    fields: vec![],
                    field_tokens: vec![],
                    field_index: HashMap::new(),
                    pragmas: vec![],
                    doc_comment: None,
                    doc: None,
                }),
                BaseClass::StableContainer(Some(max)) => ClassDefinition::Custom(ClassDef {
                    base: BaseClass::StableContainer(Some(*max)),
                    fields: vec![],
                    field_tokens: vec![],
                    field_index: HashMap::new(),
                    pragmas: vec![],
                    doc_comment: None,
                    doc: None,
                }),
                BaseClass::Profile(Some((name, max))) => {
                    let resolvers = self.resolvers.borrow();
                    let class_def = match ty {
                        Ty::Imported(path, _, _) | Ty::ImportedComplex(path, _, _, _) => {
                            let resolver = resolvers.get(path).unwrap();
                            resolver.classes.get(name).unwrap()
                        }
                        _ => self.classes.get(name).unwrap(),
                    };
                    let resolved_def = self.resolve_class_definition(class_def, &[]);
                    ClassDefinition::Custom(ClassDef {
                        base: BaseClass::Profile(Some((name.clone(), *max))),
                        fields: resolved_def.fields,
                        field_tokens: resolved_def.field_tokens,
                        field_index: resolved_def.field_index,
                        pragmas: resolved_def.pragmas,
                        doc_comment: resolved_def.doc_comment,
                        doc: resolved_def.doc,
                    })
                }
                _ => panic!(
                    "Expected base class alias to have the necessary fields for the base class it aliases"
                ),
            };

            self.classes.insert(alias_str, class_def);
        }

        resolved
    }

    /// Adds a class definition to the resolver's registry
    ///
    /// # Arguments
    ///
    /// * `class_ident` - The identifier for the class
    /// * `class_def` - The class definition to add
    pub fn add_class(&mut self, class_ident: &syn::Ident, class_def: ClassDef) {
        let class_str = class_ident.to_string();

        // Add the class to the classes map so it can be inherited from
        if !self.classes.contains_key(&class_str) {
            self.classes
                .insert(class_str.clone(), ClassDefinition::Custom(class_def));
        }

        // Add the class to the types map so it can be referenced by other types
        self.types.entry(class_str.clone()).or_insert_with(|| {
            TypeDefinition::CustomType(Box::new(TypeResolution {
                ty: Some(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path::from(syn::Ident::new(
                        &class_str,
                        proc_macro2::Span::call_site(),
                    )),
                })),
                resolution: TypeResolutionKind::Class(class_str),
            }))
        });
    }

    /// Adds a constant to the resolver's registry
    ///
    /// # Arguments
    ///
    /// * `constant` - The constant to add
    /// * `value` - The value of the constant
    pub fn add_constant(&mut self, constant: &syn::Ident, value: u64) {
        let constant_str = constant.to_string();
        self.types.insert(
            constant_str,
            TypeDefinition::CustomType(Box::new(TypeResolution {
                ty: Some(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path::from(constant.clone()),
                })),
                resolution: TypeResolutionKind::Constant(value),
            })),
        );
    }

    /// Resolves an imported type to its concrete TypeResolution
    fn resolve_imported_type(&self, path: &PathBuf, name: &Identifier) -> TypeResolution {
        let resolvers = self.resolvers.borrow();
        // If the key doesn't exist it means it's imported from an external crate.
        if !resolvers.contains_key(path) {
            // Convert path to rust module path
            let mut path_segments = syn::punctuated::Punctuated::new();
            path_segments.extend(path.to_str().unwrap().split(std::path::MAIN_SEPARATOR).map(
                |s| syn::PathSegment {
                    ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                },
            ));
            // Add the name of the imported type
            path_segments.push(syn::PathSegment {
                ident: syn::Ident::new(&name.0, proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::None,
            });

            let ty = Some(syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: path_segments,
                },
            }));

            return TypeResolution {
                ty,
                resolution: TypeResolutionKind::External,
            };
        }
        let resolver = resolvers.get(path).unwrap();

        // Check if it's a base_class
        let base_class = resolver.resolve_base_class(&Ty::Simple(name.clone()));
        if let Some(base_class) = base_class {
            return base_class;
        }

        let mut type_resolution = resolver.resolve_type(&Ty::Simple(name.clone()), None);

        // Create a path with the crate prefix for internal imports
        // crate::folder1::folder2::name
        let path = path.clone();
        let path_str = path.to_str().unwrap();
        let mut path_segments = syn::punctuated::Punctuated::new();
        path_segments.push(syn::PathSegment {
            ident: syn::Ident::new("crate", proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        path_segments.extend(
            path_str
                .split(std::path::MAIN_SEPARATOR)
                .map(|s| syn::PathSegment {
                    ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                }),
        );
        path_segments.push(syn::PathSegment {
            ident: syn::Ident::new(&name.0, proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });

        type_resolution.ty = Some(syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: path_segments,
            },
        }));

        // If the type was not found (empty internal module), treat as External
        if type_resolution.resolution == TypeResolutionKind::Unresolved {
            type_resolution.resolution = TypeResolutionKind::External;
        }

        type_resolution
    }

    fn resolve_imported_complex_type(
        &self,
        path: &PathBuf,
        name: &Identifier,
        args: &[TyExpr],
    ) -> TypeResolution {
        let generic_args: Vec<GenericArgument> = args
            .iter()
            .map(|arg| self.ty_expr_to_generic_arg(arg))
            .collect();
        let resolvers = self.resolvers.borrow();
        let is_external = !resolvers.contains_key(path);
        let ty = self.build_imported_type_path(path, name, Some(generic_args), !is_external);

        if is_external {
            return TypeResolution {
                ty: Some(ty),
                resolution: TypeResolutionKind::External,
            };
        }

        let resolver = resolvers.get(path).unwrap();
        let mut type_resolution =
            resolver.resolve_type(&Ty::Complex(name.clone(), args.to_vec()), None);
        type_resolution.ty = Some(ty);

        if type_resolution.resolution == TypeResolutionKind::Unresolved {
            type_resolution.resolution = TypeResolutionKind::External;
        }

        type_resolution
    }

    fn ty_expr_to_generic_arg(&self, ty_expr: &TyExpr) -> GenericArgument {
        match ty_expr {
            TyExpr::Ty(ty) => {
                let resolved = self.resolve_type(ty, None);
                let ty = if resolved.resolution == TypeResolutionKind::Unresolved {
                    self.ty_to_type_fallback(ty)
                } else {
                    resolved.unwrap_type_preserving_const_names()
                };
                GenericArgument::Type(ty)
            }
            TyExpr::Int(value) => {
                let lit = syn::LitInt::new(&value.eval().to_string(), Span::call_site());
                GenericArgument::Const(parse_quote!(#lit))
            }
            TyExpr::ConstRef(ident, _) => {
                let ident = Ident::new(&ident.0, Span::call_site());
                GenericArgument::Const(parse_quote!(#ident))
            }
            TyExpr::None => GenericArgument::Type(parse_quote!(())),
        }
    }

    fn ty_to_type_fallback(&self, ty: &Ty) -> syn::Type {
        match ty {
            Ty::Simple(ident) => {
                let ident = Ident::new(&ident.0, Span::call_site());
                parse_quote!(#ident)
            }
            Ty::Imported(path, name, _) => {
                let resolvers = self.resolvers.borrow();
                let include_crate = resolvers.contains_key(path);
                self.build_imported_type_path(path, name, None, include_crate)
            }
            Ty::ImportedComplex(path, name, _, args) => {
                let resolvers = self.resolvers.borrow();
                let include_crate = resolvers.contains_key(path);
                let generic_args: Vec<GenericArgument> = args
                    .iter()
                    .map(|arg| self.ty_expr_to_generic_arg(arg))
                    .collect();
                self.build_imported_type_path(path, name, Some(generic_args), include_crate)
            }
            Ty::Complex(name, args) => {
                let ident = Ident::new(&name.0, Span::call_site());
                let generic_args: Vec<GenericArgument> = args
                    .iter()
                    .map(|arg| self.ty_expr_to_generic_arg(arg))
                    .collect();
                let mut path_segments = syn::punctuated::Punctuated::new();
                let args = self.generic_args_to_path_args(generic_args);
                path_segments.push(syn::PathSegment {
                    ident,
                    arguments: args,
                });
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: path_segments,
                    },
                })
            }
        }
    }

    fn build_imported_type_path(
        &self,
        path: &PathBuf,
        name: &Identifier,
        generic_args: Option<Vec<GenericArgument>>,
        include_crate: bool,
    ) -> syn::Type {
        let mut path_segments = syn::punctuated::Punctuated::new();
        if include_crate {
            path_segments.push(syn::PathSegment {
                ident: syn::Ident::new("crate", proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::None,
            });
        }
        path_segments.extend(
            path.to_str()
                .unwrap()
                .split(std::path::MAIN_SEPARATOR)
                .map(|s| syn::PathSegment {
                    ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                }),
        );
        let arguments = match generic_args {
            Some(args) => self.generic_args_to_path_args(args),
            None => syn::PathArguments::None,
        };
        path_segments.push(syn::PathSegment {
            ident: syn::Ident::new(&name.0, proc_macro2::Span::call_site()),
            arguments,
        });

        syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: path_segments,
            },
        })
    }

    fn generic_args_to_path_args(&self, args: Vec<GenericArgument>) -> syn::PathArguments {
        if args.is_empty() {
            return PathArguments::None;
        }
        let mut punctuated = syn::punctuated::Punctuated::new();
        punctuated.extend(args);
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: punctuated,
            gt_token: Default::default(),
        })
    }

    /// Generates view type code for union types
    ///
    /// This generates the `UnionRef<'a>` struct and its associated implementations
    /// including selector methods, TreeHash, and DecodeView. It also generates
    /// view type aliases for variants that use type aliases (e.g., `DepositRef<'a>`).
    ///
    /// # Arguments
    ///
    /// * `union_name` - The name of the union type (e.g., "PendingInputEntry")
    /// * `union_ident` - The identifier for the union type
    /// * `args` - The resolved type arguments for each union variant
    /// * `variant_names` - The names of each variant (from field names in new syntax)
    /// * `variant_pragmas` - The pragmas for each variant
    ///
    /// # Returns
    ///
    /// A TokenStream containing the complete view type implementation for the union
    pub fn generate_union_view_code(
        &self,
        union_name: &str,
        union_ident: &Ident,
        args: &[TypeResolution],
        variant_names: &[String],
        variant_pragmas: &[Vec<String>],
    ) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", union_name), Span::call_site());

        let mut view_type_aliases: Vec<TokenStream> = Vec::new();
        let mut variant_view_types: Vec<(String, TokenStream)> = Vec::new();

        for (i, ty) in args.iter().enumerate() {
            if self.handle_none_variant(ty, &mut variant_view_types) {
                continue;
            }

            let variant_name = variant_names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("Selector{i}"));
            let pragmas = variant_pragmas.get(i).cloned().unwrap_or_default();

            let underlying_type_name = self.extract_underlying_type_name(ty, true);
            let underlying_view_ty = ty.to_view_type_with_pragmas(&pragmas);
            let underlying_name_opt = underlying_type_name.as_ref();
            let underlying_type_resolution = self.get_custom_type_resolution(underlying_name_opt);

            let default_name = String::new();
            let underlying_name_str = underlying_name_opt.unwrap_or(&default_name);

            let is_external_container_alias = matches!(ty.resolution, TypeResolutionKind::External)
                && variant_name != *underlying_name_str
                && pragmas
                    .iter()
                    .any(|p| p.trim() == "external_kind: container");

            let needs_alias = variant_name != *underlying_name_str
                && (matches!(ty.resolution, TypeResolutionKind::Class(_))
                    && underlying_type_resolution.is_some()
                    || is_external_container_alias);

            let final_view_ty = if needs_alias && is_external_container_alias {
                underlying_view_ty
            } else if needs_alias {
                underlying_type_resolution
                    .unwrap()
                    .to_view_type_with_pragmas(&pragmas)
            } else {
                underlying_view_ty
            };

            self.add_variant_view_type(
                &mut view_type_aliases,
                &mut variant_view_types,
                &variant_name,
                needs_alias,
                quote! { #final_view_ty },
            );
        }

        let selector_methods =
            self.generate_union_selector_methods(union_name, args, &variant_view_types);

        let to_owned_arms = self.generate_union_to_owned_arms(union_ident, args, variant_names);

        let tree_hash_arms = self.generate_union_tree_hash_arms(args);

        self.generate_union_view_struct_impl(
            ref_ident,
            union_ident,
            view_type_aliases,
            selector_methods,
            to_owned_arms,
            tree_hash_arms,
        )
    }

    fn handle_none_variant(
        &self,
        ty: &TypeResolution,
        variant_view_types: &mut Vec<(String, TokenStream)>,
    ) -> bool {
        if ty.resolution == TypeResolutionKind::None {
            variant_view_types.push(("None".to_string(), quote! { () }));
            return true;
        }
        false
    }

    fn extract_underlying_type_name(
        &self,
        ty: &TypeResolution,
        include_external: bool,
    ) -> Option<String> {
        match &ty.resolution {
            TypeResolutionKind::Class(class_name) => Some(class_name.clone()),
            TypeResolutionKind::External if include_external => {
                if let Some(syn::Type::Path(syn::TypePath { path, .. })) = &ty.ty {
                    path.segments.last().map(|seg| seg.ident.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_custom_type_resolution(&self, type_name: Option<&String>) -> Option<&TypeResolution> {
        type_name
            .and_then(|name| self.types.get(name))
            .and_then(|def| match def {
                TypeDefinition::CustomType(res) => Some(res.as_ref()),
                _ => None,
            })
    }

    fn add_variant_view_type(
        &self,
        view_type_aliases: &mut Vec<TokenStream>,
        variant_view_types: &mut Vec<(String, TokenStream)>,
        variant_name: &str,
        needs_alias: bool,
        view_ty: TokenStream,
    ) {
        if needs_alias {
            let alias_name = Ident::new(&format!("{}Ref", variant_name), Span::call_site());
            view_type_aliases.push(quote! {
                pub type #alias_name<'a> = #view_ty;
            });
            variant_view_types.push((variant_name.to_string(), quote! { #alias_name<'_> }));
        } else {
            variant_view_types.push((variant_name.to_string(), quote! { #view_ty }));
        }
    }

    fn generate_union_view_struct_impl(
        &self,
        ref_ident: Ident,
        union_ident: &Ident,
        view_type_aliases: Vec<TokenStream>,
        selector_methods: Vec<TokenStream>,
        to_owned_arms: Vec<TokenStream>,
        tree_hash_arms: Vec<TokenStream>,
    ) -> TokenStream {
        quote! {
            #(#view_type_aliases)*

            #[derive(Debug, Copy, Clone)]
            pub struct #ref_ident<'a> {
                bytes: &'a [u8],
            }

            impl<'a> #ref_ident<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }

                #(#selector_methods)*

                pub fn to_owned(&self) -> #union_ident {
                    match self.selector() {
                        #(#to_owned_arms,)*
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }

            impl<'a> ssz::view::DecodeView<'a> for #ref_ident<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }

            impl<'a> ssz::view::SszTypeInfo for #ref_ident<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }

                fn ssz_fixed_len() -> usize {
                    0
                }
            }

            impl<'a> ssz_types::view::ToOwnedSsz<#union_ident> for #ref_ident<'a> {
                fn to_owned(&self) -> #union_ident {
                    <#ref_ident<'a>>::to_owned(self)
                }
            }

            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ref_ident<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }

                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }

                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }

                fn tree_hash_root(&self) -> H::Output {
                    match self.selector() {
                        #(#tree_hash_arms,)*
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
        }
    }

    /// Generates selector methods for union view types
    ///
    /// # Arguments
    ///
    /// * `union_name` - The name of the union type
    /// * `args` - The resolved type arguments for the union variants
    /// * `variant_view_types` - The view types for each variant (name, type)
    ///
    /// # Returns
    ///
    /// A vector of TokenStreams containing the selector method implementations
    pub fn generate_union_selector_methods(
        &self,
        union_name: &str,
        args: &[TypeResolution],
        variant_view_types: &[(String, TokenStream)],
    ) -> Vec<TokenStream> {
        args.iter()
            .enumerate()
            .map(|(i, ty)| {
                let method_name = Ident::new(&format!("as_selector{i}"), Span::call_site());
                let selector_value = i as u8;
                let error_msg = format!("Wrong selector for {}: expected {}", union_name, i);

                match ty.resolution {
                    TypeResolutionKind::None => {
                        quote! {
                            pub fn #method_name(&self) -> Result<(), ssz::DecodeError> {
                                if self.selector() != #selector_value {
                                    return Err(ssz::DecodeError::BytesInvalid(
                                        #error_msg.to_string()
                                    ));
                                }
                                Ok(())
                            }
                        }
                    }
                    _ => {
                        let (_, view_ty) = variant_view_types.get(i).unwrap();
                        quote! {
                            pub fn #method_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                if self.selector() != #selector_value {
                                    return Err(ssz::DecodeError::BytesInvalid(
                                        #error_msg.to_string()
                                    ));
                                }
                                ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                            }
                        }
                    }
                }
            })
            .collect()
    }

    /// Generates TreeHash match arms for union view types
    ///
    /// # Arguments
    ///
    /// * `args` - The resolved type arguments for the union variants
    ///
    /// # Returns
    ///
    /// A vector of TokenStreams containing the TreeHash match arms
    pub fn generate_union_tree_hash_arms(&self, args: &[TypeResolution]) -> Vec<TokenStream> {
        args.iter()
            .enumerate()
            .map(|(i, ty)| {
                let selector_value = i as u8;
                let method_name = Ident::new(&format!("as_selector{i}"), Span::call_site());

                match ty.resolution {
                    TypeResolutionKind::None => {
                        quote! {
                            #selector_value => {
                                // For empty variants, use precomputed zero hash
                                let zero_root = H::get_zero_hash(0);
                                tree_hash::mix_in_selector_with_hasher::<H>(
                                    &zero_root,
                                    #selector_value
                                ).expect("valid selector")
                            }
                        }
                    }
                    _ => {
                        quote! {
                            #selector_value => {
                                let value = self.#method_name().expect("valid selector");
                                tree_hash::mix_in_selector_with_hasher::<H>(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    #selector_value
                                ).expect("valid selector")
                            }
                        }
                    }
                }
            })
            .collect()
    }

    /// Generates to_owned match arms for union view types
    ///
    /// # Arguments
    ///
    /// * `union_ident` - The identifier for the union type
    /// * `args` - The resolved type arguments for the union variants
    /// * `variant_names` - The names of each variant
    ///
    /// # Returns
    ///
    /// A vector of TokenStreams containing the to_owned match arms
    pub fn generate_union_to_owned_arms(
        &self,
        union_ident: &Ident,
        args: &[TypeResolution],
        variant_names: &[String],
    ) -> Vec<TokenStream> {
        args.iter()
            .enumerate()
            .map(|(i, ty)| {
                let selector_value = i as u8;
                let variant_name = variant_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("Selector{i}"));
                let variant_ident = Ident::new(&variant_name, Span::call_site());
                let method_name = Ident::new(&format!("as_selector{i}"), Span::call_site());

                match ty.resolution {
                    TypeResolutionKind::None => {
                        quote! {
                            #selector_value => {
                                self.#method_name().expect("valid selector");
                                #union_ident::#variant_ident
                            }
                        }
                    }
                    TypeResolutionKind::Boolean | TypeResolutionKind::UInt(_) => {
                        quote! {
                            #selector_value => #union_ident::#variant_ident(
                                self.#method_name().expect("valid selector")
                            )
                        }
                    }
                    _ => {
                        // Use ToOwnedSsz trait method for proper type resolution with external
                        // types
                        quote! {
                            #selector_value => #union_ident::#variant_ident({
                                let view = self.#method_name().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                    }
                }
            })
            .collect()
    }
}
