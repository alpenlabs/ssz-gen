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
use syn::{Ident, parse_quote};

use super::{BaseClass, ClassDef, ClassDefinition, TypeDefinition, TypeResolution};
use crate::types::TypeResolutionKind;

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

        // Check if the type is a base class (Container, StableContainer, Profile or aliases to
        // them)
        let base_class = self.resolve_base_class(ty);
        if let Some(base_class) = base_class {
            return base_class;
        }

        // Extract the type arguments
        let args = match ty {
            Ty::Imported(_, _, _) | Ty::Simple(_) => vec![],
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
        match type_def {
            Some(def) => self.resolve_type_definition(def, args, alias_ident),
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
                    Ty::Complex(_, args) => match args.first() {
                        Some(TyExpr::Int(int)) => int.eval(),
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
                        Ty::Complex(_, args) => match args.first() {
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
                                    panic!("Expected profile to inherit from a stable container");
                                }
                            }
                            _ => panic!("Profile must inherit from a class"),
                        },
                    },
                };
                TypeResolution {
                    ty: None,
                    resolution: TypeResolutionKind::BaseClass(BaseClass::Profile(Some((
                        name, max,
                    )))),
                }
            }
        })
    }

    /// Resolves a type definition with its arguments to a concrete TypeResolution
    ///
    /// # Arguments
    ///
    /// * `def` - The type definition to resolve
    /// * `args` - The type arguments for the definition
    ///
    /// # Returns
    ///
    /// A TypeResolution representing the resolved type definition
    fn resolve_type_definition(
        &self,
        def: &TypeDefinition,
        args: Vec<TypeResolution>,
        alias_ident: Option<&syn::Ident>,
    ) -> TypeResolution {
        let mut resolved_ty = None;
        let resolution = match def {
            TypeDefinition::Boolean => TypeResolutionKind::Boolean,
            TypeDefinition::UInt(size) => TypeResolutionKind::UInt(*size),
            TypeDefinition::Vector => {
                let size = match args[1].resolution {
                    TypeResolutionKind::Constant(size) => size,
                    _ => panic!("Expected constant value for vector size"),
                };
                TypeResolutionKind::Vector(Box::new(args[0].clone()), size)
            }
            TypeDefinition::List => {
                let size = match args[1].resolution {
                    TypeResolutionKind::Constant(size) => size,
                    _ => panic!("Expected constant value for list size"),
                };
                TypeResolutionKind::List(Box::new(args[0].clone()), size)
            }
            TypeDefinition::Bitvector => {
                let size = match args[0].resolution {
                    TypeResolutionKind::Constant(size) => size,
                    _ => panic!("Expected constant value for bitvector size"),
                };
                TypeResolutionKind::Bitvector(size)
            }
            TypeDefinition::Bitlist => {
                let size = match args[0].resolution {
                    TypeResolutionKind::Constant(size) => size,
                    _ => panic!("Expected constant value for bitlist size"),
                };
                TypeResolutionKind::Bitlist(size)
            }
            TypeDefinition::Optional => TypeResolutionKind::Optional(Box::new(args[0].clone())),
            TypeDefinition::Union => {
                // Special case for Union[None, T]
                if args.len() == 2 && args[0].resolution == TypeResolutionKind::None {
                    TypeResolutionKind::Option(Box::new(args[1].clone()))
                } else {
                    let ident = alias_ident.unwrap().clone();
                    let ident_str = ident.to_string();

                    // Generate the enum variants Selector0, Selector1, etc. and insert the union
                    // into our union tracker
                    let variants: Vec<syn::Variant> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let ident = syn::Ident::new(
                                &format!("Selector{i}"),
                                proc_macro2::Span::call_site(),
                            );
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
                            let variant_ident = Ident::new(&format!("Selector{i}"), Span::call_site());

                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    quote! {
                                        #ident::#variant_ident => {
                                            tree_hash::mix_in_selector_with_hasher::<H>(
                                                &tree_hash::Hash256::ZERO,
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

                    // Generate selector methods for lazy variant access
                    let selector_methods: Vec<TokenStream> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let method_name = Ident::new(&format!("as_selector{i}"), Span::call_site());
                            let selector_value = i as u8;
                            let error_msg = format!("Wrong selector for {}: expected {}", ident_str, i);

                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    // Unit variant
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
                                    let view_ty = ty.to_view_type();
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
                        .collect();

                    // Generate to_owned dispatch based on selector
                    let to_owned_arms: Vec<TokenStream> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let selector_value = i as u8;
                            let variant_ident =
                                Ident::new(&format!("Selector{i}"), Span::call_site());
                            let method_name =
                                Ident::new(&format!("as_selector{i}"), Span::call_site());

                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    quote! {
                                        #selector_value => {
                                            self.#method_name().expect("valid selector");
                                            #ident::#variant_ident
                                        }
                                    }
                                }
                                TypeResolutionKind::Boolean | TypeResolutionKind::UInt(_) => {
                                    quote! {
                                        #selector_value => #ident::#variant_ident(
                                            self.#method_name().expect("valid selector")
                                        )
                                    }
                                }
                                _ => {
                                    quote! {
                                        #selector_value => #ident::#variant_ident(
                                            self.#method_name().expect("valid selector").to_owned()
                                        )
                                    }
                                }
                            }
                        })
                        .collect();

                    // Generate TreeHash implementation for lazy union
                    let tree_hash_arms: Vec<TokenStream> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let selector_value = i as u8;
                            let method_name = Ident::new(&format!("as_selector{i}"), Span::call_site());

                            match ty.resolution {
                                TypeResolutionKind::None => {
                                    quote! {
                                        #selector_value => {
                                            tree_hash::mix_in_selector_with_hasher::<H>(
                                                &tree_hash::Hash256::ZERO,
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
                        .collect();

                    // Store the view union as thin wrapper with lazy access
                    self.union_tracker.borrow_mut().insert(
                        format!("{}Ref", ident_str),
                        quote! {
                            #[derive(Debug, Copy, Clone)]
                            pub struct #ref_ident<'a> {
                                bytes: &'a [u8],
                            }

                            impl<'a> #ref_ident<'a> {
                                pub fn selector(&self) -> u8 {
                                    self.bytes[0]
                                }

                                #(#selector_methods)*

                                pub fn to_owned(&self) -> #ident {
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
                        },
                    );

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

        let class_def = self.classes.get(ty.base_name().0.as_str())?;

        let args = match ty {
            Ty::Simple(_) | Ty::Imported(_, _, _) => vec![],
            Ty::Complex(_, args) => args.clone(),
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
                    let class_def = if let Ty::Imported(path, _, _) = ty {
                        let resolver = resolvers.get(path).unwrap();
                        resolver.classes.get(name).unwrap()
                    } else {
                        self.classes.get(name).unwrap()
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
}
