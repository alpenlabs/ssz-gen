//! The types used in the SSZ codegen

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Path, Type, TypePath, parse_quote};

use crate::{derive_config::DeriveConfig, types::resolver::TypeResolver};
pub mod resolver;

/// Represents a size expression for type parameters
///
/// This captures size arguments in SSZ types like `List[T, N]` where N can be
/// either a literal value or a named constant reference.
#[derive(Clone, Debug, PartialEq)]
pub enum SizeExpr {
    /// Literal size value (e.g., `42`)
    Literal(u64),
    /// Named constant reference with its resolved value.
    ///
    /// Stores the constant name (e.g., `"VAL_X"`) and its resolved value (e.g., `42`).
    /// Used to preserve constant names in generated code like `BitList<{ VAL_X as usize }>`.
    ConstRef(String, u64),
}

impl SizeExpr {
    /// Get the resolved numeric value
    pub fn value(&self) -> u64 {
        match self {
            SizeExpr::Literal(v) | SizeExpr::ConstRef(_, v) => *v,
        }
    }
}

/// Converts a primitive type name into a Rust [`Type`].
///
/// # Arguments
///
/// * `base_name` - The name of the primitive type (e.g., "bool", "u32")
///
/// # Returns
///
/// A [`Type`] representing the Rust primitive type
pub fn primitive_rust_type(base_name: &str) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path::from(Ident::new(base_name, Span::call_site())),
    })
}

/// Converts `crate::ssz::` paths to `super::` paths for cross-entry type references.
///
/// When types are referenced across entry points in nested modules, they should use
/// `super::` paths instead of `crate::ssz::` paths.
///
/// # Arguments
///
/// * `path` - The path to potentially convert
///
/// # Returns
///
/// `Some(new_path)` if the path was converted, [`None`] if no conversion was needed
fn convert_crate_ssz_to_super(path: &Path) -> Option<Path> {
    let mut path_segments = path.segments.clone();
    if path_segments.len() >= 3
        && path_segments[0].ident == "crate"
        && path_segments[1].ident == "ssz"
    {
        path_segments = path_segments.into_iter().skip(2).collect();
        let mut new_segments = syn::punctuated::Punctuated::new();
        new_segments.push(syn::PathSegment {
            ident: Ident::new("super", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        new_segments.extend(path_segments.iter().cloned());
        Some(Path {
            leading_colon: path.leading_colon,
            segments: new_segments,
        })
    } else {
        None
    }
}

/// Represents the resolution of a type
#[derive(Clone, Debug)]
pub struct TypeResolution {
    /// The type we want to use in our generated rust code
    pub ty: Option<Type>,
    /// What SSZ primitive type this type resolves to
    pub resolution: TypeResolutionKind,
}

impl PartialEq for TypeResolution {
    fn eq(&self, other: &Self) -> bool {
        self.resolution == other.resolution
    }
}

/// Represents the resolution status of a type reference
#[derive(Clone, Debug, PartialEq)]
pub enum TypeResolutionKind {
    /// Type has not been resolved yet
    Unresolved,
    /// Type resolves to None
    None,
    /// Constant value
    Constant(u64),
    /// Type resolves to a base class
    BaseClass(BaseClass),
    /// Class value
    Class(String),
    /// Boolean type
    Boolean,
    /// Unsigned integer with specified bit width
    UInt(usize),
    /// Fixed-length vector (element type, size expression)
    Vector(Box<TypeResolution>, SizeExpr),
    /// Variable-length list (element type, size expression)
    List(Box<TypeResolution>, SizeExpr),
    /// Fixed-length vector of bits (size expression)
    Bitvector(SizeExpr),
    /// Variable-length list of bits (size expression)
    Bitlist(SizeExpr),
    /// Optional type (can be None) for use in stable containers only
    Optional(Box<TypeResolution>),
    /// Special for Union[None, T]
    Option(Box<TypeResolution>),
    /// Union type
    Union(String, Vec<TypeResolution>),
    /// Fixed-length byte array
    Bytes(usize),
    /// External type always assumed valid no matter the context
    External,
}

impl TypeResolution {
    /// Returns true if the resolution is `Unresolved`
    ///
    /// # Returns
    ///
    /// `true` if the resolution is `Unresolved`, `false` otherwise
    pub fn is_unresolved(&self) -> bool {
        matches!(self.resolution, TypeResolutionKind::Unresolved)
    }

    /// Returns true if the resolution is not `Unresolved`
    ///
    /// # Returns
    ///
    /// `true` if the resolution is not `Unresolved`, `false` otherwise
    pub fn is_resolved(&self) -> bool {
        !self.is_unresolved()
    }

    /// Returns true if the resolution is a BaseClass
    ///
    /// # Returns
    ///
    /// `true` if the resolution is a BaseClass, `false` otherwise
    pub fn is_base_class(&self) -> bool {
        matches!(self.resolution, TypeResolutionKind::BaseClass(_))
    }

    /// Returns true if the resolution is a Type
    ///
    /// # Returns
    ///
    /// `true` if the resolution is a Type, `false` otherwise
    pub fn is_type(&self) -> bool {
        !self.is_base_class() && !self.is_unresolved()
    }

    /// Returns true if the resolution is a constant
    ///
    /// # Returns
    ///
    /// `true` if the resolution is a constant, `false` otherwise
    pub fn is_constant(&self) -> bool {
        matches!(self.resolution, TypeResolutionKind::Constant(_))
    }

    /// Unwraps any of the type variants into a [`Type`], panics if not a type variant.
    ///
    /// # Arguments
    ///
    /// * `type_resolver` - The type resolver to use to resolve the type
    ///
    /// # Returns
    ///
    /// The unwrapped [`Type`] if this is one of the type variants
    ///
    /// # Panics
    ///
    /// Panics if the resolution is not a type variant.
    pub fn unwrap_type(&self) -> Type {
        if let Some(ty) = &self.ty {
            // For locally generated types, convert crate::ssz:: paths to super:: paths
            if matches!(self.resolution, TypeResolutionKind::Class(_))
                && let Type::Path(TypePath { path, .. }) = ty
                && let Some(converted_path) = convert_crate_ssz_to_super(path)
            {
                return parse_quote! { #converted_path };
            }
            return ty.clone();
        }

        match &self.resolution {
            TypeResolutionKind::Class(class) => {
                let class = Ident::new(class, Span::call_site());
                parse_quote!(#class)
            }
            TypeResolutionKind::Boolean => primitive_rust_type("bool"),
            TypeResolutionKind::UInt(size) => {
                // Use U128/U256 from ssz_primitives for SSZ-specific serialization
                match size {
                    128 => primitive_rust_type("U128"),
                    256 => primitive_rust_type("U256"),
                    _ => primitive_rust_type(&format!("u{size}")),
                }
            }
            TypeResolutionKind::Vector(ty, size_expr) => {
                let size = size_expr.value() as usize;
                // Special case: Vector[byte, N] -> FixedBytes<N>
                // This ensures proper trait implementations (SszTypeInfo, ToOwnedSsz) when used in
                // Lists
                if matches!(ty.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(FixedBytes<#size>)
                } else {
                    let ty = ty.unwrap_type();
                    parse_quote!(FixedVector<#ty, #size>)
                }
            }
            TypeResolutionKind::List(ty, size_expr) => {
                let ty = ty.unwrap_type();
                let size = size_expr.value() as usize;
                parse_quote!(VariableList<#ty, #size>)
            }
            TypeResolutionKind::Bitvector(size_expr) => {
                let size = size_expr.value() as usize;
                parse_quote!(BitVector<#size>)
            }
            TypeResolutionKind::Bitlist(size_expr) => {
                let size = size_expr.value() as usize;
                parse_quote!(BitList<#size>)
            }
            TypeResolutionKind::Optional(ty) => {
                let ty = ty.unwrap_type();
                parse_quote!(Optional<#ty>)
            }
            TypeResolutionKind::Option(ty) => {
                let ty = ty.unwrap_type();
                parse_quote!(Option<#ty>)
            }
            TypeResolutionKind::Union(ident, _) => {
                let ident = Ident::new(ident, Span::call_site());
                parse_quote!(#ident)
            }
            TypeResolutionKind::Bytes(size) => {
                // Generate FixedBytes for consistency with Vector[byte, N]
                parse_quote!(FixedBytes<#size>)
            }
            _ => panic!("Expected type resolution to be a type"),
        }
    }

    /// Check if this type contains any constant references in size parameters
    pub fn contains_const_ref(&self) -> bool {
        match &self.resolution {
            TypeResolutionKind::Vector(ty, size_expr) | TypeResolutionKind::List(ty, size_expr) => {
                matches!(size_expr, SizeExpr::ConstRef(_, _)) || ty.contains_const_ref()
            }
            TypeResolutionKind::Bitvector(size_expr) | TypeResolutionKind::Bitlist(size_expr) => {
                matches!(size_expr, SizeExpr::ConstRef(_, _))
            }
            TypeResolutionKind::Optional(ty) | TypeResolutionKind::Option(ty) => {
                ty.contains_const_ref()
            }
            _ => false,
        }
    }

    /// Unwrap with constant preservation if needed, otherwise regular unwrap
    fn unwrap_with_const_check(&self) -> Type {
        if self.contains_const_ref() {
            self.unwrap_type_preserving_const_names()
        } else {
            self.unwrap_type()
        }
    }

    /// Unwraps a type, preserving constant names in size parameters as `{ CONST as usize }`.
    /// Expands type aliases only when they contain constants.
    pub fn unwrap_type_preserving_const_names(&self) -> Type {
        match &self.resolution {
            TypeResolutionKind::Vector(ty, size_expr) => {
                let elem_ty = ty.unwrap_with_const_check();
                match size_expr {
                    SizeExpr::Literal(size)
                        if matches!(ty.resolution, TypeResolutionKind::UInt(8)) =>
                    {
                        let size = *size as usize;
                        parse_quote!(FixedBytes<#size>)
                    }
                    SizeExpr::ConstRef(name, _)
                        if matches!(ty.resolution, TypeResolutionKind::UInt(8)) =>
                    {
                        let const_ident = Ident::new(name, Span::call_site());
                        parse_quote!(FixedBytes<{ #const_ident as usize }>)
                    }
                    SizeExpr::Literal(size) => {
                        let size = *size as usize;
                        parse_quote!(FixedVector<#elem_ty, #size>)
                    }
                    SizeExpr::ConstRef(name, _) => {
                        let const_ident = Ident::new(name, Span::call_site());
                        parse_quote!(FixedVector<#elem_ty, { #const_ident as usize }>)
                    }
                }
            }
            TypeResolutionKind::List(ty, size_expr) => {
                let elem_ty = ty.unwrap_with_const_check();
                match size_expr {
                    SizeExpr::Literal(size) => {
                        let size = *size as usize;
                        parse_quote!(VariableList<#elem_ty, #size>)
                    }
                    SizeExpr::ConstRef(name, _) => {
                        let const_ident = Ident::new(name, Span::call_site());
                        parse_quote!(VariableList<#elem_ty, { #const_ident as usize }>)
                    }
                }
            }
            TypeResolutionKind::Bitvector(size_expr) | TypeResolutionKind::Bitlist(size_expr) => {
                let type_name = if matches!(self.resolution, TypeResolutionKind::Bitvector(_)) {
                    "BitVector"
                } else {
                    "BitList"
                };
                match size_expr {
                    SizeExpr::Literal(size) => {
                        let size = *size as usize;
                        let ident = Ident::new(type_name, Span::call_site());
                        parse_quote!(#ident<#size>)
                    }
                    SizeExpr::ConstRef(name, _) => {
                        let const_ident = Ident::new(name, Span::call_site());
                        let ident = Ident::new(type_name, Span::call_site());
                        parse_quote!(#ident<{ #const_ident as usize }>)
                    }
                }
            }
            TypeResolutionKind::Optional(ty) => {
                let ty = ty.unwrap_with_const_check();
                parse_quote!(Optional<#ty>)
            }
            TypeResolutionKind::Option(ty) => {
                let ty = ty.unwrap_with_const_check();
                parse_quote!(Option<#ty>)
            }
            TypeResolutionKind::Class(class) => {
                let class = Ident::new(class, Span::call_site());
                parse_quote!(#class)
            }
            TypeResolutionKind::Boolean => primitive_rust_type("bool"),
            TypeResolutionKind::UInt(size) => {
                // Use U128/U256 from ssz_primitives for SSZ-specific serialization
                match size {
                    128 => primitive_rust_type("U128"),
                    256 => primitive_rust_type("U256"),
                    _ => primitive_rust_type(&format!("u{size}")),
                }
            }
            TypeResolutionKind::Union(ident, _) => {
                let ident = Ident::new(ident, Span::call_site());
                parse_quote!(#ident)
            }
            TypeResolutionKind::Bytes(size) => parse_quote!(FixedBytes<#size>),
            TypeResolutionKind::External => self.unwrap_type(),
            TypeResolutionKind::BaseClass(_)
            | TypeResolutionKind::None
            | TypeResolutionKind::Constant(_)
            | TypeResolutionKind::Unresolved => {
                panic!("Expected type resolution to be a type")
            }
        }
    }

    /// Unwraps the [`BaseClass`] variant, panics if not a [`BaseClass`].
    ///
    /// # Returns
    ///
    /// The unwrapped [`BaseClass`] if this is a [`BaseClass`] variant
    ///
    /// # Panics
    ///
    /// Panics if the resolution is not a [`BaseClass`].
    pub fn unwrap_base_class(self) -> BaseClass {
        match self.resolution {
            TypeResolutionKind::BaseClass(base_class) => base_class,
            _ => panic!("Expected type resolution to be a base class"),
        }
    }

    /// Checks if this type is fixed-size in SSZ encoding.
    ///
    /// # Returns
    ///
    /// [`true`] if the type has a fixed size, [`false`] if variable-length.
    pub fn is_fixed_size(&self) -> bool {
        match &self.resolution {
            TypeResolutionKind::Boolean => true,
            TypeResolutionKind::UInt(_) => true,
            TypeResolutionKind::Bytes(_) => true,
            TypeResolutionKind::Bitvector(_) => true,
            TypeResolutionKind::Vector(inner, _) => inner.is_fixed_size(),
            TypeResolutionKind::List(_, _) => false,
            TypeResolutionKind::Bitlist(_) => false,
            TypeResolutionKind::Optional(_) => false,
            TypeResolutionKind::Option(_) => false,
            TypeResolutionKind::Union(_, _) => false,
            TypeResolutionKind::Class(_) => false, // Containers can have variable fields
            TypeResolutionKind::External => false, /* External types: unknown layout, treat as */
            // variable
            _ => false,
        }
    }

    /// Returns the fixed size of this type in bytes (if it's fixed-size).
    ///
    /// # Returns
    ///
    /// `Some(size)` if the type is fixed-size, [`None`] otherwise
    ///
    /// # Panics
    ///
    /// Panics if called on a variable-size type.
    pub fn fixed_size(&self) -> usize {
        match &self.resolution {
            TypeResolutionKind::Boolean => 1,
            TypeResolutionKind::UInt(bits) => bits / 8,
            TypeResolutionKind::Bytes(n) => *n,
            TypeResolutionKind::Bitvector(size_expr) => (size_expr.value() as usize).div_ceil(8), /* Round up to */
            // bytes
            TypeResolutionKind::Vector(inner, size_expr) => {
                if inner.is_fixed_size() {
                    inner.fixed_size() * (size_expr.value() as usize)
                } else {
                    // Vector of variable-size items: offsets + data
                    panic!("Cannot get fixed size of vector with variable-size items")
                }
            }
            _ => panic!("Type {:?} is not fixed-size", self.resolution),
        }
    }

    /// Converts an owned type to its corresponding view type for zero-copy decoding
    ///
    /// # Returns
    ///
    /// A [`Type`] representing the view type (e.g., `BytesRef`, `FixedBytesRef`)
    pub fn to_view_type(&self) -> Type {
        self.to_view_type_with_pragmas(&[])
    }

    /// Generate the view type with pragma information for external type handling
    pub fn to_view_type_with_pragmas(&self, pragmas: &[String]) -> Type {
        self.to_view_type_inner(false, pragmas)
    }

    /// Internal helper that can distinguish between direct field context and list/vector context
    #[allow(
        clippy::only_used_in_recursion,
        reason = "Parameter is intentionally passed through to nested types"
    )]
    fn to_view_type_inner(&self, in_collection: bool, pragmas: &[String]) -> Type {
        match &self.resolution {
            TypeResolutionKind::Class(class) => {
                if let Some(Type::Path(TypePath { path, .. })) = &self.ty {
                    let mut path_segments = path.segments.clone();
                    // Strip `crate::ssz::` prefix if present (all generated code is inside
                    // `ssz` module)
                    if let Some(converted_path) = convert_crate_ssz_to_super(path) {
                        path_segments = converted_path.segments;
                    }
                    if let Some(last_segment) = path_segments.last_mut() {
                        let ref_ident = Ident::new(
                            &format!("{}Ref", last_segment.ident),
                            last_segment.ident.span(),
                        );
                        last_segment.ident = ref_ident;
                    }
                    let view_path = Path {
                        leading_colon: path.leading_colon,
                        segments: path_segments,
                    };
                    parse_quote! { #view_path<'a> }
                } else {
                    let ref_ident = Ident::new(&format!("{}Ref", class), Span::call_site());
                    parse_quote!(#ref_ident<'a>)
                }
            }
            TypeResolutionKind::Boolean => primitive_rust_type("bool"),
            TypeResolutionKind::UInt(size) => {
                // Use U128/U256 from ssz_primitives for SSZ-specific serialization
                match size {
                    128 => primitive_rust_type("U128"),
                    256 => primitive_rust_type("U256"),
                    _ => primitive_rust_type(&format!("u{size}")),
                }
            }
            TypeResolutionKind::Vector(ty, size_expr) => {
                let size = size_expr.value() as usize;
                // Special case: Vector[byte, N] -> FixedBytesRef<'a, N>
                // This ensures proper trait implementations when used in Lists
                if matches!(ty.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(FixedBytesRef<'a, #size>)
                } else {
                    let inner_view_ty = ty.to_view_type_inner(true, pragmas);
                    parse_quote!(FixedVectorRef<'a, #inner_view_ty, #size>)
                }
            }
            TypeResolutionKind::List(ty, size_expr) => {
                let inner = &**ty;
                let size = size_expr.value() as usize;

                // Special case: List<u8, N> -> BytesRef<'a>
                if matches!(inner.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(BytesRef<'a>)
                } else {
                    let inner_view_ty = ty.to_view_type_inner(true, pragmas);
                    parse_quote!(VariableListRef<'a, #inner_view_ty, #size>)
                }
            }
            TypeResolutionKind::Bitvector(size_expr) => {
                let size = size_expr.value() as usize;
                parse_quote!(BitVectorRef<'a, #size>)
            }
            TypeResolutionKind::Bitlist(size_expr) => {
                let size = size_expr.value() as usize;
                parse_quote!(BitListRef<'a, #size>)
            }
            TypeResolutionKind::Optional(ty) => {
                let inner_view_ty = ty.to_view_type_inner(in_collection, pragmas);
                parse_quote!(Optional<#inner_view_ty>)
            }
            TypeResolutionKind::Option(ty) => {
                let inner_view_ty = ty.to_view_type_inner(in_collection, pragmas);
                parse_quote!(Option<#inner_view_ty>)
            }
            TypeResolutionKind::Union(ident, _) => {
                if let Some(Type::Path(TypePath { path, .. })) = &self.ty {
                    let mut path_segments = path.segments.clone();
                    // Strip `crate::ssz::` prefix if present (all generated code is inside
                    // `ssz` module)
                    if let Some(converted_path) = convert_crate_ssz_to_super(path) {
                        path_segments = converted_path.segments;
                    }
                    if let Some(last_segment) = path_segments.last_mut() {
                        let ref_ident = Ident::new(
                            &format!("{}Ref", last_segment.ident),
                            last_segment.ident.span(),
                        );
                        last_segment.ident = ref_ident;
                    }
                    let view_path = Path {
                        leading_colon: path.leading_colon,
                        segments: path_segments,
                    };
                    parse_quote! { #view_path<'a> }
                } else {
                    let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
                    parse_quote!(#ref_ident<'a>)
                }
            }
            TypeResolutionKind::Bytes(size) => {
                parse_quote!(FixedBytesRef<'a, #size>)
            }
            TypeResolutionKind::External => {
                // Check pragma to determine if external type is a container or primitive
                let is_container = pragmas.iter().any(|p| {
                    let trimmed = p.trim();
                    trimmed == "external_kind: container"
                });

                let is_primitive = pragmas.iter().any(|p| {
                    let trimmed = p.trim();
                    trimmed == "external_kind: primitive"
                });

                // Generate Ref variant if marked as container
                if is_container {
                    if let Some(Type::Path(TypePath { path, .. })) = &self.ty {
                        let mut path_segments = path.segments.clone();
                        if let Some(last_segment) = path_segments.last_mut() {
                            let ref_ident = Ident::new(
                                &format!("{}Ref", last_segment.ident),
                                last_segment.ident.span(),
                            );
                            last_segment.ident = ref_ident;
                        }
                        let view_path = Path {
                            leading_colon: path.leading_colon,
                            segments: path_segments,
                        };
                        parse_quote! { #view_path<'a> }
                    } else {
                        self.unwrap_type()
                    }
                } else if is_primitive {
                    // Use type directly for primitives
                    self.unwrap_type()
                } else {
                    // Default: use type directly (conservative - won't break existing code)
                    // Users should add pragma if they need Ref variant
                    self.unwrap_type()
                }
            }
            _ => panic!("Cannot convert {:?} to view type", self.resolution),
        }
    }

    /// Checks if the type is compatible with the other type
    ///
    /// # Arguments
    ///
    /// * `other` - The other type
    ///
    /// # Returns
    ///
    /// `true` if the type is compatible, `false` otherwise
    pub fn check_field_compatibility_for_profile(
        &self,
        other: &TypeResolution,
        resolver: &TypeResolver<'_>,
    ) -> bool {
        if self.resolution == other.resolution {
            return true;
        }

        if self.resolution == TypeResolutionKind::External
            || other.resolution == TypeResolutionKind::External
        {
            return true;
        }

        match (&self.resolution, &other.resolution) {
            // Fields MAY be required in Profile[B] by unwrapping them from Optional
            (TypeResolutionKind::Optional(original_inner_ty), _) => {
                original_inner_ty.check_field_compatibility_for_profile(other, resolver)
            }

            // Bitlist[N] / Bitvector[N] field types are compatible if they share the same capacity
            // N
            (TypeResolutionKind::Bitvector(original_cap), TypeResolutionKind::Bitlist(new_cap))
            | (TypeResolutionKind::Bitlist(original_cap), TypeResolutionKind::Bitvector(new_cap)) => {
                original_cap == new_cap
            }

            // List[T, N] / Vector[T, N] field types are compatible if T is compatible and if they
            // also share the same capacity N
            (
                TypeResolutionKind::List(original_ty, original_cap),
                TypeResolutionKind::Vector(new_ty, new_cap),
            )
            | (
                TypeResolutionKind::Vector(original_ty, original_cap),
                TypeResolutionKind::List(new_ty, new_cap),
            ) => {
                original_ty.check_field_compatibility_for_profile(new_ty, resolver)
                    && *original_cap == *new_cap
            }

            (TypeResolutionKind::Class(original_class), TypeResolutionKind::Class(new_class)) => {
                // Get class definitions
                let original_class_def = match resolver.classes.get(original_class).unwrap() {
                    ClassDefinition::Custom(class_def) => class_def,
                    _ => panic!("Expected defined class when checking for field compatibility"),
                };
                let new_class_def = match resolver.classes.get(new_class).unwrap() {
                    ClassDefinition::Custom(class_def) => class_def,
                    _ => panic!("Expected defined class when checking for field compatibility"),
                };

                match (&original_class_def.base, &new_class_def.base) {
                    (BaseClass::StableContainer(_), BaseClass::StableContainer(_)) => {
                        // Make sure they share the same capacity N
                        // Make sure they share all their fields in the same order and all their
                        // fields are compatible
                        original_class_def.check_capacity_compatibility(new_class_def)
                            && original_class_def.check_field_compatibility(new_class_def, resolver)
                    }
                    (BaseClass::Container, BaseClass::StableContainer(_))
                    | (BaseClass::StableContainer(_), BaseClass::Container)
                    | (BaseClass::Container, BaseClass::Container) => {
                        // Make sure they share all their fields in the same order and all their
                        // fields are compatible
                        original_class_def.check_field_compatibility(new_class_def, resolver)
                    }
                    (
                        BaseClass::Profile(Some((stable_container_name, _))),
                        BaseClass::StableContainer(_),
                    )
                    | (
                        BaseClass::StableContainer(_),
                        BaseClass::Profile(Some((stable_container_name, _))),
                    ) => {
                        // Get the original stable container definition the profile class was
                        // inheriting from
                        let original_stable_container_def =
                            match resolver.classes.get(stable_container_name).unwrap() {
                                ClassDefinition::Custom(class_def) => class_def,
                                _ => panic!(
                                    "Expected defined class when checking for field compatibility"
                                ),
                            };

                        // Get the class that was a stable container and not profile
                        let stable_container_def = if original_class_def.base.is_profile() {
                            &new_class_def
                        } else {
                            &original_class_def
                        };

                        // Make sure the stable containers share the same capacity N
                        // Make sure the stable containers share all their fields in the same order
                        // and all their fields are compatible
                        original_stable_container_def
                            .check_capacity_compatibility(stable_container_def)
                            && original_stable_container_def
                                .check_field_compatibility(stable_container_def, resolver)
                    }
                    (
                        BaseClass::Profile(Some((original_stable_container_name, _))),
                        BaseClass::Profile(Some((new_stable_container_name, _))),
                    ) => {
                        // Get the original stable container definitions the profile classes are
                        // inheriting from
                        let original_stable_container_def = match resolver
                            .classes
                            .get(original_stable_container_name)
                            .unwrap()
                        {
                            ClassDefinition::Custom(class_def) => class_def,
                            _ => panic!(
                                "Expected defined class when checking for field compatibility"
                            ),
                        };
                        let new_stable_container_def =
                            match resolver.classes.get(new_stable_container_name).unwrap() {
                                ClassDefinition::Custom(class_def) => class_def,
                                _ => panic!(
                                    "Expected defined class when checking for field compatibility"
                                ),
                            };

                        // Make sure the stable containers share the same capacity N
                        // Make sure the stable containers share all their fields in the same order
                        // and all their fields are compatible Make sure the
                        // profile classes share all their fields in the same order and all their
                        // fields are compatible
                        original_stable_container_def
                            .check_capacity_compatibility(new_stable_container_def)
                            && original_stable_container_def
                                .check_field_compatibility(new_stable_container_def, resolver)
                            && original_class_def.check_field_compatibility(new_class_def, resolver)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

/// Represents the base class types for SSZ data structures
#[derive(Clone, Debug, PartialEq)]
pub enum BaseClass {
    /// A container type
    Container,
    /// A stable container with optional maximum field count
    StableContainer(Option<u64>),
    /// A profile type with optional name
    Profile(Option<(String, u64)>),
    /// A union type
    Union,
}

impl BaseClass {
    /// Returns true if this is a Container base class
    ///
    /// # Returns
    ///
    /// `true` if this is a Container, `false` otherwise
    pub fn is_container(&self) -> bool {
        matches!(self, BaseClass::Container)
    }

    /// Returns true if this is a StableContainer base class
    ///
    /// # Returns
    ///
    /// `true` if this is a StableContainer, `false` otherwise
    pub fn is_stable_container(&self) -> bool {
        matches!(self, BaseClass::StableContainer(_))
    }

    /// Returns true if this is a Profile base class
    ///
    /// # Returns
    ///
    /// `true` if this is a Profile, `false` otherwise
    pub fn is_profile(&self) -> bool {
        matches!(self, BaseClass::Profile(_))
    }
}

/// Definition of a class field
#[derive(Clone, Debug)]
pub struct ClassFieldDef {
    /// Index of the field
    /// Useful for Profile classes since during merkleization, we need the original index of the
    /// field
    pub index: usize,
    /// The name of the field
    pub name: String,
    /// The type of the field
    pub ty: TypeResolution,
    /// Pragma comments for the field
    pub pragmas: Vec<String>,
    /// Doc comment for the field
    pub doc_comment: Option<String>,
}

/// Definition of a class with its base type and fields
#[derive(Clone, Debug)]
pub struct ClassDef {
    /// The base class type
    pub base: BaseClass,
    /// Field names
    pub field_index: HashMap<String, usize>,
    /// The field definitions for this class
    pub fields: Vec<ClassFieldDef>,
    /// Token streams for each field
    pub field_tokens: Vec<TokenStream>,
    /// Pragma comments for the class
    pub pragmas: Vec<String>,
    /// Doc comment for the class (from ### comments)
    pub doc_comment: Option<String>,
    /// Docstring for the class (from """ docstrings)
    pub doc: Option<String>,
}

impl ClassDef {
    /// Returns true if this class is a Container
    ///
    /// # Returns
    ///
    /// `true` if this class is a Container, `false` otherwise
    pub fn is_container(&self) -> bool {
        self.base.is_container()
    }

    /// Returns true if this class is a StableContainer
    ///
    /// # Returns
    ///
    /// `true` if this class is a StableContainer, `false` otherwise
    pub fn is_stable_container(&self) -> bool {
        self.base.is_stable_container()
    }

    /// Returns true if this class is a Profile
    ///
    /// # Returns
    ///
    /// `true` if this class is a Profile, `false` otherwise
    pub fn is_profile(&self) -> bool {
        self.base.is_profile()
    }

    /// Checks if the capacity of the two StableContainer classes is compatible by checking if they
    /// are equal
    ///
    /// # Arguments
    ///
    /// * `other` - The other class definition
    ///
    /// # Returns
    ///
    /// `true` if the capacity is compatible, `false` otherwise
    pub fn check_capacity_compatibility(&self, other: &ClassDef) -> bool {
        let self_cap = match self.base {
            BaseClass::StableContainer(cap) => cap.unwrap(),
            _ => panic!("Expected stable container when checking for field compatibility"),
        };
        let new_cap = match other.base {
            BaseClass::StableContainer(cap) => cap.unwrap(),
            _ => panic!("Expected stable container when checking for field compatibility"),
        };
        self_cap == new_cap
    }

    /// Checks if the fields of the two classes are compatible by checking order and type
    /// compatibility
    ///
    /// # Arguments
    ///
    /// * `other` - The other class definition
    ///
    /// # Returns
    ///
    /// `true` if the fields are compatible, `false` otherwise
    pub fn check_field_compatibility(&self, other: &ClassDef, resolver: &TypeResolver<'_>) -> bool {
        self.fields
            .iter()
            .zip(other.fields.iter())
            .all(|(self_field, other_field)| {
                self_field.name == other_field.name
                    && self_field
                        .ty
                        .check_field_compatibility_for_profile(&other_field.ty, resolver)
            })
    }

    /// Converts the class definition to a token stream with the given identifier
    ///
    /// # Arguments
    ///
    /// * `ident` - The identifier for the class
    ///
    /// # Returns
    ///
    /// A TokenStream containing the generated Rust code for the class
    pub fn to_token_stream(&self, ident: &Ident, derive_cfg: &DeriveConfig) -> TokenStream {
        use crate::pragma::ParsedPragma;

        let field_tokens = &self.field_tokens;
        let type_name = ident.to_string();

        // Parse pragmas
        let pragmas = ParsedPragma::parse(&self.pragmas);
        // Container, StableContainer, and Profile don't support PartialOrd/Ord
        let is_container = matches!(
            self.base,
            BaseClass::Container | BaseClass::StableContainer(_) | BaseClass::Profile(_)
        );
        let owned_derive =
            derive_cfg.owned_derive_attr_with_pragmas_filtered(&type_name, &pragmas, is_container);

        // Build struct-level attributes from pragmas
        let struct_attrs = if !pragmas.struct_attrs.is_empty() {
            let attrs = &pragmas.struct_attrs;
            quote! {
                #(#attrs)*
            }
        } else {
            quote! {}
        };

        // Format doc comment for the struct
        // Merge doc (""") and doc_comment (###) with doc taking priority
        let doc_comments = match (&self.doc, &self.doc_comment) {
            (Some(docstring), Some(doc_comment)) => {
                // Both exist: docstring first, then blank line, then doc_comment
                let merged = format!("{}\n\n{}", docstring.trim(), doc_comment.trim());
                Self::format_doc_comment(&merged)
            }
            (Some(docstring), None) => {
                // Only docstring
                Self::format_doc_comment(docstring)
            }
            (None, Some(doc_comment)) => {
                // Only doc_comment
                Self::format_doc_comment(doc_comment)
            }
            (None, None) => quote! {},
        };

        match self.base {
            BaseClass::Container => {
                quote! {
                    #doc_comments
                    #owned_derive
                    #struct_attrs
                    #[ssz(struct_behaviour="container")]
                    pub struct #ident {
                        #(#field_tokens),*
                    }
                }
            }
            BaseClass::StableContainer(Some(max)) => {
                let max = max as usize;
                quote! {
                    #doc_comments
                    #owned_derive
                    #struct_attrs
                    #[ssz(struct_behaviour="stable_container", max_fields=#max)]
                    pub struct #ident {
                        #(#field_tokens),*
                    }
                }
            }
            BaseClass::Profile(Some((_, _))) => {
                quote! {
                    #doc_comments
                    #owned_derive
                    #struct_attrs
                    #[ssz(struct_behaviour="profile")]
                    pub struct #ident {
                        #(#field_tokens),*
                    }
                }
            }
            BaseClass::Union => {
                quote! {}
            }
            _ => panic!("Base class arguments not set"),
        }
    }

    /// Computes the SSZ layout for this container.
    ///
    /// This determines field offsets and whether the container is fixed or variable-size.
    ///
    /// # Returns
    ///
    /// A tuple of:
    /// - fixed_portion_size: size in bytes of the fixed portion
    /// - fixed_size: `Some(size)` if all fields are fixed-size, `None` otherwise
    /// - num_variable_fields: count of variable-length fields
    fn compute_layout(&self) -> (usize, Option<usize>, usize) {
        let mut fixed_offset = 0usize;
        let mut num_variable_fields = 0usize;
        let mut has_variable_fields = false;

        for field in &self.fields {
            if field.ty.is_fixed_size() {
                fixed_offset += field.ty.fixed_size();
            } else {
                has_variable_fields = true;
                num_variable_fields += 1;
                // Variable-length fields add an offset pointer to the fixed portion
                fixed_offset += 4; // BYTES_PER_LENGTH_OFFSET
            }
        }

        let fixed_size = if has_variable_fields {
            None
        } else {
            Some(fixed_offset)
        };

        (fixed_offset, fixed_size, num_variable_fields)
    }

    /// Generates field layout information for a given field index.
    ///
    /// Returns (offset_expr, is_fixed, size_expr) where:
    /// - offset_expr: TokenStream to compute the field's byte offset
    /// - is_fixed: whether the field is at a fixed offset
    /// - size_expr: TokenStream to compute the field's size (or None for variable fields)
    fn generate_field_layout(
        &self,
        field_index: usize,
    ) -> (TokenStream, bool, Option<TokenStream>) {
        let field = &self.fields[field_index];

        // Calculate the offset based on preceding fields
        let mut offset = 0usize;
        let mut variable_index = 0usize;

        for (i, f) in self.fields.iter().enumerate() {
            if i == field_index {
                break;
            }
            if f.ty.is_fixed_size() {
                offset += f.ty.fixed_size();
            } else {
                variable_index += 1;
                offset += 4; // BYTES_PER_LENGTH_OFFSET
            }
        }

        if field.ty.is_fixed_size() {
            let size = field.ty.fixed_size();
            (quote! { #offset }, true, Some(quote! { #size }))
        } else {
            let (fixed_portion_size, _, num_variable_fields) = self.compute_layout();
            (
                quote! {
                    ssz::layout::read_variable_offset(
                        self.bytes,
                        #fixed_portion_size,
                        #num_variable_fields,
                        #variable_index
                    )?
                },
                false,
                None,
            )
        }
    }

    /// Generates the view struct definition for zero-copy decoding.
    ///
    /// Creates a thin wrapper around `&[u8]` instead of eagerly decoded fields.
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class (e.g., `Foo`)
    ///
    /// # Returns
    ///
    /// Formats a doc comment string into `///` lines with 80-character wrapping.
    /// The 80-character limit includes the `/// ` prefix.
    pub fn format_doc_comment(text: &str) -> TokenStream {
        if text.trim().is_empty() {
            return quote! {};
        }

        const MAX_LINE_LENGTH: usize = 80;
        const PREFIX_LENGTH: usize = 4; // "/// "

        let mut lines = Vec::new();
        let mut current_line = String::new();

        // Split by newlines first to preserve paragraph breaks
        for paragraph in text.split('\n') {
            let trimmed = paragraph.trim();
            if trimmed.is_empty() {
                // Empty line - add as blank doc comment
                if !current_line.is_empty() {
                    lines.push(format!("/// {}", current_line.trim()));
                    current_line.clear();
                }
                lines.push("///".to_string());
                continue;
            }

            // Word-wrap within paragraph
            for word in trimmed.split_whitespace() {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                // Check if line with prefix fits within 80 chars
                if test_line.len() + PREFIX_LENGTH <= MAX_LINE_LENGTH {
                    current_line = test_line;
                } else {
                    if !current_line.is_empty() {
                        lines.push(format!("/// {}", current_line.trim()));
                    }
                    current_line = word.to_string();
                    // If a single word is too long, we still need to add it
                    if current_line.len() + PREFIX_LENGTH > MAX_LINE_LENGTH {
                        lines.push(format!("/// {}", current_line.trim()));
                        current_line.clear();
                    }
                }
            }
        }

        // Add remaining line
        if !current_line.is_empty() {
            lines.push(format!("/// {}", current_line.trim()));
        }

        // Parse each line as a doc comment
        let doc_lines: Vec<TokenStream> = lines
            .iter()
            .map(|line| syn::parse_str::<TokenStream>(line).unwrap_or_else(|_| quote! {}))
            .collect();

        quote! {
            #(#doc_lines)*
        }
    }

    /// A [`TokenStream`] containing the generated Rust code for the view struct (e.g.,
    /// `FooRef<'a>`).
    pub fn to_view_struct(&self, ident: &Ident, derive_cfg: &DeriveConfig) -> TokenStream {
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let doc_comment = format!(
            "Zero-copy view over [`{}`].\n\n\
            This type wraps SSZ-encoded bytes without allocating. \
            Fields are accessed via lazy getter methods. \
            Use `.to_owned()` to convert to the owned type when needed.",
            ident
        );
        let doc_comments = Self::format_doc_comment(&doc_comment);
        let type_name = ident.to_string();
        let pragmas = ParsedPragma::parse(&self.pragmas);
        // Container, StableContainer, and Profile don't support PartialOrd/Ord
        let is_container = matches!(
            self.base,
            BaseClass::Container | BaseClass::StableContainer(_) | BaseClass::Profile(_)
        );
        let view_derive =
            derive_cfg.view_derive_attr_with_pragmas_filtered(&type_name, &pragmas, is_container);

        // All view structs are now thin wrappers around bytes
        match self.base {
            BaseClass::Container | BaseClass::StableContainer(_) | BaseClass::Profile(_) => {
                quote! {
                    #doc_comments
                    #[allow(dead_code, reason = "generated code using ssz-gen")]
                    #view_derive
                    pub struct #ref_ident<'a> {
                        bytes: &'a [u8],
                    }
                }
            }
            BaseClass::Union => {
                // Unions don't have a single view struct, they have selector methods
                // This is handled by the union_tracker
                quote! {}
            }
        }
    }

    /// Generates TreeHash implementation for view structs.
    ///
    /// Optimized to hash bytes directly for basic fields, avoiding decode overhead.
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class (e.g., `Foo`)
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the TreeHash implementation.
    pub fn to_view_tree_hash_impl(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());

        match self.base {
            BaseClass::Container => {
                // Generate optimized tree hashing that uses bytes directly for basic types
                let hash_operations: Vec<TokenStream> = self
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, field)| {
                        let field_name = Ident::new(&field.name, Span::call_site());
                        let (offset_expr, is_fixed, size_expr) = self.generate_field_layout(idx);

                        // Check if this is a basic packable type
                        let is_basic = matches!(
                            field.ty.resolution,
                            TypeResolutionKind::Boolean | TypeResolutionKind::UInt(_)
                        );

                        if is_basic && is_fixed {
                            // Optimize: hash bytes directly for basic types
                            let size = size_expr.unwrap();
                            quote! {
                                {
                                    let offset = #offset_expr;
                                    let field_bytes = &self.bytes[offset..offset + #size];
                                    hasher.write(field_bytes).expect("write field");
                                }
                            }
                        } else {
                            // For composite types, use getter and hash the result
                            quote! {
                                {
                                    let #field_name = self.#field_name().expect("valid view");
                                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<H>::tree_hash_root(&#field_name);
                                    hasher.write(root.as_ref()).expect("write field");
                                }
                            }
                        }
                    })
                    .collect();

                quote! {
                    impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ref_ident<'a> {
                        fn tree_hash_type() -> tree_hash::TreeHashType {
                            tree_hash::TreeHashType::Container
                        }

                        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                            unreachable!("Container should never be packed")
                        }

                        fn tree_hash_packing_factor() -> usize {
                            unreachable!("Container should never be packed")
                        }

                        fn tree_hash_root(&self) -> H::Output {
                            use tree_hash::TreeHash;

                            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(0);
                            #(#hash_operations)*

                            hasher.finish().expect("finish hasher")
                        }
                    }
                }
            }
            BaseClass::StableContainer(Some(max)) => {
                let max = max as usize;
                let field_names: Vec<Ident> = self
                    .fields
                    .iter()
                    .map(|f| Ident::new(&f.name, Span::call_site()))
                    .collect();

                quote! {
                    impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ref_ident<'a> {
                        fn tree_hash_type() -> tree_hash::TreeHashType {
                            tree_hash::TreeHashType::StableContainer
                        }

                        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                            unreachable!("StableContainer should never be packed")
                        }

                        fn tree_hash_packing_factor() -> usize {
                            unreachable!("StableContainer should never be packed")
                        }

                        fn tree_hash_root(&self) -> H::Output {
                            use tree_hash::TreeHash;

                            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(#max);
                            #(
                                let #field_names = self.#field_names().expect("valid view");
                                let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<H>::tree_hash_root(&#field_names);
                                hasher.write(root.as_ref()).expect("write field");
                            )*

                            hasher.finish().expect("finish hasher")
                        }
                    }
                }
            }
            BaseClass::Profile(Some((_, max))) => {
                let max = max as usize;
                let field_names: Vec<Ident> = self
                    .fields
                    .iter()
                    .map(|f| Ident::new(&f.name, Span::call_site()))
                    .collect();
                let indices: Vec<usize> = self.fields.iter().map(|f| f.index).collect();

                quote! {
                    impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ref_ident<'a> {
                        fn tree_hash_type() -> tree_hash::TreeHashType {
                            tree_hash::TreeHashType::Container
                        }

                        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                            unreachable!("Profile should never be packed")
                        }

                        fn tree_hash_packing_factor() -> usize {
                            unreachable!("Profile should never be packed")
                        }

                        fn tree_hash_root(&self) -> H::Output {
                            use tree_hash::TreeHash;

                            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(#max);
                            #(
                                {
                                    let #field_names = self.#field_names().expect("valid view");
                                    // Skip to the stable index
                                    for _ in 0..#indices {
                                        // Placeholder for proper index handling
                                    }
                                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<H>::tree_hash_root(&#field_names);
                                    hasher.write(root.as_ref()).expect("write field");
                                }
                            )*

                            hasher.finish().expect("finish hasher")
                        }
                    }
                }
            }
            BaseClass::Union => {
                quote! {}
            }
            _ => panic!("Base class arguments not set"),
        }
    }

    /// Generates generic TreeHash implementation for owned structs.
    ///
    /// This generates `impl<H: TreeHashDigest> TreeHash<H> for Type` instead of using
    /// the derive macro which only generates `TreeHash<Sha256Hasher>`.
    ///
    /// # Arguments
    ///
    /// * `ident` - The identifier for the class (e.g., `Foo`)
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the generic TreeHash implementation.
    pub fn to_owned_tree_hash_impl(&self, ident: &Ident) -> TokenStream {
        let field_names: Vec<Ident> = self
            .fields
            .iter()
            .map(|f| Ident::new(&f.name, Span::call_site()))
            .collect();

        match self.base {
            BaseClass::Container => {
                let num_leaves = field_names.len();
                quote! {
                    impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ident {
                        fn tree_hash_type() -> tree_hash::TreeHashType {
                            tree_hash::TreeHashType::Container
                        }

                        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                            unreachable!("Container should never be packed")
                        }

                        fn tree_hash_packing_factor() -> usize {
                            unreachable!("Container should never be packed")
                        }

                        fn tree_hash_root(&self) -> H::Output {
                            use tree_hash::TreeHash;
                            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(#num_leaves);
                            #(
                                hasher.write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.#field_names).as_ref())
                                    .expect("tree hash derive should not apply too many leaves");
                            )*
                            hasher.finish().expect("tree hash derive should not have a remaining buffer")
                        }
                    }
                }
            }
            BaseClass::StableContainer(Some(max)) | BaseClass::Profile(Some((_, max))) => {
                let max_fields = max as usize;
                let field_names: Vec<Ident> = self
                    .fields
                    .iter()
                    .map(|f| Ident::new(&f.name, Span::call_site()))
                    .collect();

                let hashes: Vec<TokenStream> = self
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| {
                        let field_name = &field_names[idx];
                        quote! {
                            if let ssz_types::Optional::Some(ref #field_name) = self.#field_name {
                                hasher.write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(#field_name).as_ref())
                                    .expect("tree hash derive should not apply too many leaves");
                            } else {
                                hasher.write(H::get_zero_hash_slice(0))
                                    .expect("tree hash derive should not apply too many leaves");
                            }
                        }
                    })
                    .collect();

                let set_active_fields: Vec<TokenStream> = self
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| {
                        let field_name = &field_names[idx];
                        quote! {
                            if self.#field_name.is_some() {
                                active_fields.set(#idx, true).expect("Should not be out of bounds");
                            }
                        }
                    })
                    .collect();

                quote! {
                    impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #ident {
                        fn tree_hash_type() -> tree_hash::TreeHashType {
                            tree_hash::TreeHashType::StableContainer
                        }

                        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                            unreachable!("StableContainer/Profile should never be packed")
                        }

                        fn tree_hash_packing_factor() -> usize {
                            unreachable!("StableContainer/Profile should never be packed")
                        }

                        fn tree_hash_root(&self) -> H::Output {
                            use tree_hash::TreeHash;
                            use ssz_types::BitVector;

                            // Construct BitVector
                            let mut active_fields = BitVector::<#max_fields>::new();

                            #(
                                #set_active_fields
                            )*

                            // Hash according to `max_fields` regardless of the actual number of fields
                            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(#max_fields);

                            #(
                                #hashes
                            )*

                            let hash = hasher.finish().expect("tree hash derive should not have a remaining buffer");
                            let active_fields_hash = <_ as tree_hash::TreeHash<H>>::tree_hash_root(&active_fields);

                            H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                        }
                    }
                }
            }
            BaseClass::Union => {
                quote! {}
            }
            _ => panic!("Base class arguments not set"),
        }
    }

    /// Generates getter methods for view struct fields.
    ///
    /// Each field gets a method that computes its position and decodes on-demand.
    /// For StableContainer/Profile, accounts for prepended bitvector.
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class (e.g., `Foo`)
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the impl block with getter methods.
    pub fn to_view_getters(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let (fixed_portion_size, _, num_variable_fields) = self.compute_layout();

        // Check if we need to account for bitvector (StableContainer/Profile with Optional fields)
        let optional_count = self
            .fields
            .iter()
            .filter(|f| matches!(f.ty.resolution, TypeResolutionKind::Optional(_)))
            .count();

        let bitvector_offset = match self.base {
            BaseClass::StableContainer(_) | BaseClass::Profile(_) if optional_count > 0 => {
                optional_count.div_ceil(8)
            }
            _ => 0,
        };

        let getters: Vec<TokenStream> = self
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_name = Ident::new(&field.name, Span::call_site());
                let view_ty = field.ty.to_view_type_with_pragmas(&field.pragmas);
                let (offset_expr, is_fixed, size_expr) = self.generate_field_layout(idx);

                // Special handling for Option types (from Union[null, T])
                // These are encoded as unions with a selector byte
                if matches!(field.ty.resolution, TypeResolutionKind::Option(_)) {
                    let inner_ty = match &field.ty.resolution {
                        TypeResolutionKind::Option(inner) => inner,
                        _ => unreachable!(),
                    };
                    let inner_view_ty = inner_ty.to_view_type_with_pragmas(&field.pragmas);

                    if is_fixed {
                        let size = size_expr.unwrap();
                        if bitvector_offset > 0 {
                            quote! {
                                pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                    let bitvector_offset = #bitvector_offset;
                                    let offset = bitvector_offset + #offset_expr;
                                    let end = offset + #size;
                                    if end > self.bytes.len() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: self.bytes.len(),
                                            expected: end,
                                        });
                                    }
                                    let bytes = &self.bytes[offset..end];
                                    if bytes.is_empty() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: 0,
                                            expected: 1,
                                        });
                                    }
                                    let selector = bytes[0];
                                    match selector {
                                        0 => Ok(None),
                                        1 => {
                                            let inner = <#inner_view_ty as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                                            Ok(Some(inner))
                                        }
                                        _ => Err(ssz::DecodeError::BytesInvalid(
                                            format!("Invalid union selector for Option: {}", selector)
                                        ))
                                    }
                                }
                            }
                        } else {
                            quote! {
                                pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                    let offset = #offset_expr;
                                    let end = offset + #size;
                                    if end > self.bytes.len() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: self.bytes.len(),
                                            expected: end,
                                        });
                                    }
                                    let bytes = &self.bytes[offset..end];
                                    if bytes.is_empty() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: 0,
                                            expected: 1,
                                        });
                                    }
                                    let selector = bytes[0];
                                    match selector {
                                        0 => Ok(None),
                                        1 => {
                                            let inner = <#inner_view_ty as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                                            Ok(Some(inner))
                                        }
                                        _ => Err(ssz::DecodeError::BytesInvalid(
                                            format!("Invalid union selector for Option: {}", selector)
                                        ))
                                    }
                                }
                            }
                        }
                    } else {
                        // Variable-length Option field
                        let mut variable_index = 0usize;
                        for (i, f) in self.fields.iter().enumerate() {
                            if i == idx {
                                break;
                            }
                            if !f.ty.is_fixed_size() {
                                variable_index += 1;
                            }
                        }
                        let next_variable_index = variable_index + 1;

                        if bitvector_offset > 0 {
                            quote! {
                                pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                    let bitvector_offset = #bitvector_offset;
                                    let container_bytes = &self.bytes[bitvector_offset..];
                                    let start = ssz::layout::read_variable_offset(
                                        container_bytes,
                                        #fixed_portion_size,
                                        #num_variable_fields,
                                        #variable_index
                                    )?;
                                    let end = ssz::layout::read_variable_offset_or_end(
                                        container_bytes,
                                        #fixed_portion_size,
                                        #num_variable_fields,
                                        #next_variable_index
                                    )?;
                                    if start > end || end > container_bytes.len() {
                                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                                    }
                                    let bytes = &container_bytes[start..end];
                                    if bytes.is_empty() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: 0,
                                            expected: 1,
                                        });
                                    }
                                    let selector = bytes[0];
                                    match selector {
                                        0 => Ok(None),
                                        1 => {
                                            let inner = <#inner_view_ty as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                                            Ok(Some(inner))
                                        }
                                        _ => Err(ssz::DecodeError::BytesInvalid(
                                            format!("Invalid union selector for Option: {}", selector)
                                        ))
                                    }
                                }
                            }
                        } else {
                            quote! {
                                pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                    let start = ssz::layout::read_variable_offset(
                                        self.bytes,
                                        #fixed_portion_size,
                                        #num_variable_fields,
                                        #variable_index
                                    )?;
                                    let end = ssz::layout::read_variable_offset_or_end(
                                        self.bytes,
                                        #fixed_portion_size,
                                        #num_variable_fields,
                                        #next_variable_index
                                    )?;
                                    if start > end || end > self.bytes.len() {
                                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                                    }
                                    let bytes = &self.bytes[start..end];
                                    if bytes.is_empty() {
                                        return Err(ssz::DecodeError::InvalidByteLength {
                                            len: 0,
                                            expected: 1,
                                        });
                                    }
                                    let selector = bytes[0];
                                    match selector {
                                        0 => Ok(None),
                                        1 => {
                                            let inner = <#inner_view_ty as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                                            Ok(Some(inner))
                                        }
                                        _ => Err(ssz::DecodeError::BytesInvalid(
                                            format!("Invalid union selector for Option: {}", selector)
                                        ))
                                    }
                                }
                            }
                        }
                    }
                } else if is_fixed {
                    let size = size_expr.unwrap();
                    if bitvector_offset > 0 {
                        // Account for bitvector at start
                        quote! {
                            pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                let bitvector_offset = #bitvector_offset;
                                let offset = bitvector_offset + #offset_expr;
                                let end = offset + #size;
                                if end > self.bytes.len() {
                                    return Err(ssz::DecodeError::InvalidByteLength {
                                        len: self.bytes.len(),
                                        expected: end,
                                    });
                                }
                                let bytes = &self.bytes[offset..end];
                                ssz::view::DecodeView::from_ssz_bytes(bytes)
                            }
                        }
                    } else {
                        quote! {
                            pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                let offset = #offset_expr;
                                let end = offset + #size;
                                if end > self.bytes.len() {
                                    return Err(ssz::DecodeError::InvalidByteLength {
                                        len: self.bytes.len(),
                                        expected: end,
                                    });
                                }
                                let bytes = &self.bytes[offset..end];
                                ssz::view::DecodeView::from_ssz_bytes(bytes)
                            }
                        }
                    }
                } else {
                    // Variable-length field
                    let mut variable_index = 0usize;
                    for (i, f) in self.fields.iter().enumerate() {
                        if i == idx {
                            break;
                        }
                        if !f.ty.is_fixed_size() {
                            variable_index += 1;
                        }
                    }
                    let next_variable_index = variable_index + 1;

                    if bitvector_offset > 0 {
                        // Account for bitvector at start
                        quote! {
                            pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                let bitvector_offset = #bitvector_offset;
                                let container_bytes = &self.bytes[bitvector_offset..];
                                let start = ssz::layout::read_variable_offset(
                                    container_bytes,
                                    #fixed_portion_size,
                                    #num_variable_fields,
                                    #variable_index
                                )?;
                                let end = ssz::layout::read_variable_offset_or_end(
                                    container_bytes,
                                    #fixed_portion_size,
                                    #num_variable_fields,
                                    #next_variable_index
                                )?;
                                if start > end || end > container_bytes.len() {
                                    return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                                }
                                let bytes = &container_bytes[start..end];
                                ssz::view::DecodeView::from_ssz_bytes(bytes)
                            }
                        }
                    } else {
                        quote! {
                            pub fn #field_name(&self) -> Result<#view_ty, ssz::DecodeError> {
                                let start = ssz::layout::read_variable_offset(
                                    self.bytes,
                                    #fixed_portion_size,
                                    #num_variable_fields,
                                    #variable_index
                                )?;
                                let end = ssz::layout::read_variable_offset_or_end(
                                    self.bytes,
                                    #fixed_portion_size,
                                    #num_variable_fields,
                                    #next_variable_index
                                )?;
                                if start > end || end > self.bytes.len() {
                                    return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                                }
                                let bytes = &self.bytes[start..end];
                                ssz::view::DecodeView::from_ssz_bytes(bytes)
                            }
                        }
                    }
                }
            })
            .collect();

        quote! {
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> #ref_ident<'a> {
                #(#getters)*
            }
        }
    }

    /// Generates the [`DecodeView`](ssz::view::DecodeView) implementation for the view struct
    ///
    /// Now performs validation-only construction - no eager field decoding.
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the [`DecodeView`](ssz::view::DecodeView) implementation
    pub fn to_view_decode_impl(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let (fixed_portion_size, fixed_size, num_variable_fields) = self.compute_layout();

        match self.base {
            BaseClass::Container => {
                // Generate validation code based on whether container is fixed or variable size
                let validation = if let Some(expected_size) = fixed_size {
                    // Fixed-size container: just check length
                    quote! {
                        if bytes.len() != #expected_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: #expected_size,
                            });
                        }
                    }
                } else {
                    // Variable-size container: validate offset table
                    quote! {
                        if bytes.len() < #fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: #fixed_portion_size,
                            });
                        }

                        // Validate offset table
                        let mut prev_offset: Option<usize> = None;
                        for i in 0..#num_variable_fields {
                            let offset = ssz::layout::read_variable_offset(
                                bytes,
                                #fixed_portion_size,
                                #num_variable_fields,
                                i
                            )?;

                            // First offset should point to start of variable portion
                            if i == 0 && offset != #fixed_portion_size {
                                return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                            }

                            // Offsets must not decrease
                            if let Some(prev) = prev_offset && offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }

                            // Offset must not exceed container length
                            if offset > bytes.len() {
                                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                            }

                            prev_offset = Some(offset);
                        }
                    }
                };

                quote! {
                    impl<'a> ssz::view::DecodeView<'a> for #ref_ident<'a> {
                        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                            #validation
                            Ok(Self { bytes })
                        }
                    }
                }
            }
            BaseClass::StableContainer(Some(max_fields))
            | BaseClass::Profile(Some((_, max_fields))) => {
                let max_fields_usize = max_fields as usize;

                // Count Optional fields for bitvector
                let optional_count = self
                    .fields
                    .iter()
                    .filter(|f| matches!(f.ty.resolution, TypeResolutionKind::Optional(_)))
                    .count();

                let bitvector_length = optional_count.div_ceil(8);

                // Generate validation for StableContainer/Profile
                let validation = if bitvector_length == 0 {
                    // No Optional fields - validate like a regular container
                    let (fixed_portion_size, fixed_size, num_variable_fields) =
                        self.compute_layout();

                    if let Some(expected_size) = fixed_size {
                        quote! {
                            if bytes.len() != #expected_size {
                                return Err(ssz::DecodeError::InvalidByteLength {
                                    len: bytes.len(),
                                    expected: #expected_size,
                                });
                            }
                        }
                    } else {
                        quote! {
                            if bytes.len() < #fixed_portion_size {
                                return Err(ssz::DecodeError::InvalidByteLength {
                                    len: bytes.len(),
                                    expected: #fixed_portion_size,
                                });
                            }

                            // Validate offset table
                            let mut prev_offset: Option<usize> = None;
                            for i in 0..#num_variable_fields {
                                let offset = ssz::layout::read_variable_offset(
                                    bytes,
                                    #fixed_portion_size,
                                    #num_variable_fields,
                                    i
                                )?;

                                if i == 0 && offset != #fixed_portion_size {
                                    return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                                }

                                if let Some(prev) = prev_offset && offset < prev {
                                    return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                                }

                                if offset > bytes.len() {
                                    return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                                }

                                prev_offset = Some(offset);
                            }
                        }
                    }
                } else {
                    // Has Optional fields - parse bitvector and validate
                    quote! {
                        // Import Decode trait for BitVector::from_ssz_bytes
                        use ssz::Decode;

                        // Parse bitvector
                        let bitvector_length = #bitvector_length;
                        if bytes.len() < bitvector_length {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: bitvector_length,
                            });
                        }

                        // Validate bitvector structure (don't need to store it)
                        let _bitvector = ssz_types::BitVector::<#max_fields_usize>::from_ssz_bytes(
                            &bytes[0..bitvector_length]
                        )?;

                        // Validate the container portion after the bitvector
                        // Note: More specific offset table validation based on active fields
                        // could be added here as a future optimization, but current validation
                        // is sufficient - getters will validate offsets when fields are accessed.
                        if bytes.len() < bitvector_length {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: bitvector_length,
                            });
                        }
                    }
                };

                quote! {
                    impl<'a> ssz::view::DecodeView<'a> for #ref_ident<'a> {
                        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                            #validation
                            Ok(Self { bytes })
                        }
                    }
                }
            }
            BaseClass::Union => {
                quote! {}
            }
            _ => panic!("Base class arguments not set"),
        }
    }

    /// Generates the [`SszTypeInfo`](ssz::view::SszTypeInfo) implementation for view structs.
    ///
    /// This is required for view types to be used in
    /// [`VariableListRef`](ssz_types::view::VariableListRef) and
    /// [`FixedVectorRef`](ssz_types::view::FixedVectorRef).
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the [`SszTypeInfo`](ssz::view::SszTypeInfo) implementation.
    pub fn to_view_ssz_type_info_impl(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let (_, fixed_size, _) = self.compute_layout();

        match fixed_size {
            Some(size) => {
                // Fixed-size container
                quote! {
                    impl<'a> ssz::view::SszTypeInfo for #ref_ident<'a> {
                        fn is_ssz_fixed_len() -> bool {
                            true
                        }

                        fn ssz_fixed_len() -> usize {
                            #size
                        }
                    }
                }
            }
            None => {
                // Variable-size container
                quote! {
                    impl<'a> ssz::view::SszTypeInfo for #ref_ident<'a> {
                        fn is_ssz_fixed_len() -> bool {
                            false
                        }

                        fn ssz_fixed_len() -> usize {
                            0
                        }
                    }
                }
            }
        }
    }

    /// Generates the [`ToOwnedSsz`](ssz_types::view::ToOwnedSsz) implementation for container view
    /// types.
    ///
    /// This is required for container views to be used in
    /// [`VariableListRef`](ssz_types::view::VariableListRef) and
    /// [`FixedVectorRef`](ssz_types::view::FixedVectorRef).
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the [`ToOwnedSsz`](ssz_types::view::ToOwnedSsz) implementation.
    pub fn to_view_to_owned_ssz_impl(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());

        quote! {
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<#ident> for #ref_ident<'a> {
                #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
                fn to_owned(&self) -> #ident {
                    <#ref_ident<'a>>::to_owned(self)
                }
            }
        }
    }

    /// Generates the to_owned implementation for converting view to owned
    ///
    /// Now uses getter methods instead of direct field access.
    ///
    /// # Arguments
    ///
    /// * `ident` - The base identifier for the class
    ///
    /// # Returns
    ///
    /// A [`TokenStream`] containing the `to_owned` method implementation.
    pub fn to_view_to_owned_impl(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());

        // Check if this is a StableContainer
        let is_stable_container = matches!(
            self.base,
            BaseClass::StableContainer(_) | BaseClass::Profile(_)
        );

        // Generate field conversions using getter methods
        let field_conversions: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| {
                let field_name = Ident::new(&field.name, Span::call_site());

                // For StableContainer, fields are wrapped in ssz_types::Optional<T>
                // Getter returns Result<Optional<TRef>, Error>
                // We need to convert Optional<TRef> to Optional<T>
                if is_stable_container {
                    match &field.ty.resolution {
                        TypeResolutionKind::Boolean | TypeResolutionKind::UInt(_) => {
                            // Primitives: Optional<T> -> Optional<T> (just copy)
                            quote! {
                                #field_name: self.#field_name().expect("valid view")
                            }
                        }
                        TypeResolutionKind::Union(_, _) => {
                            // Union types: getter returns Optional<UnionRef>
                            // Need to map the Optional and convert UnionRef to Union
                            quote! {
                                #field_name: match self.#field_name().expect("valid view") {
                                    ssz_types::Optional::Some(inner) => ssz_types::Optional::Some(inner.to_owned()),
                                    ssz_types::Optional::None => ssz_types::Optional::None,
                                }
                            }
                        }
                        _ => {
                            // Complex types: Optional<TRef> -> Optional<T>
                            // Use match to convert inner TRef to T
                            quote! {
                                #field_name: match self.#field_name().expect("valid view") {
                                    ssz_types::Optional::Some(inner) => ssz_types::Optional::Some(inner.to_owned()),
                                    ssz_types::Optional::None => ssz_types::Optional::None,
                                }
                            }
                        }
                    }
                } else {
                    // Regular Container (non-StableContainer)
                    match &field.ty.resolution {
                        TypeResolutionKind::Option(_) => {
                            // Option types (from Union[null, T]) - getter returns Result<Option<TRef>, Error>
                            // Need to convert Option<TRef> to Option<T> by calling to_owned() on inner if Some
                            quote! {
                                #field_name: self.#field_name().expect("valid view").map(|inner| inner.to_owned())
                            }
                        }
                        TypeResolutionKind::Boolean | TypeResolutionKind::UInt(_) => {
                            // Primitives can be copied directly from getter
                            quote! {
                                #field_name: self.#field_name().expect("valid view")
                            }
                        }
                        TypeResolutionKind::List(ty, _size) => {
                            // Check if it's List<u8, N> which uses BytesRef
                            let inner = &**ty;
                            if matches!(inner.resolution, TypeResolutionKind::UInt(8)) {
                                // BytesRef::to_owned() returns Vec<u8>, need to convert to VariableList
                                quote! {
                                    #field_name: self.#field_name().expect("valid view").to_owned().into()
                                }
                            } else {
                                // VariableListRef::to_owned() returns Result<VariableList<T, N>, Error>
                                quote! {
                                    #field_name: self.#field_name().expect("valid view").to_owned().expect("valid view")
                                }
                            }
                        }
                        TypeResolutionKind::Vector(ty, _size) => {
                            // Check if it's Vector[byte, N] which uses FixedBytesRef
                            let inner = &**ty;
                            if matches!(inner.resolution, TypeResolutionKind::UInt(8)) {
                                // FixedBytesRef::to_owned() returns [u8; N], need to wrap in FixedBytes<N>
                                quote! {
                                    #field_name: ssz_types::FixedBytes(self.#field_name().expect("valid view").to_owned())
                                }
                            } else {
                                // FixedVectorRef::to_owned() returns Result<FixedVector<T, N>, Error>
                                quote! {
                                    #field_name: self.#field_name().expect("valid view").to_owned().expect("valid view")
                                }
                            }
                        }
                        _ => {
                            // For other complex types, call getter and then to_owned()
                            quote! {
                                #field_name: self.#field_name().expect("valid view").to_owned()
                            }
                        }
                    }
                }
            })
            .collect();

        quote! {
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> #ref_ident<'a> {
                #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
                pub fn to_owned(&self) -> #ident {
                    #ident {
                        #(#field_conversions),*
                    }
                }
            }
        }
    }
}

/// Represents the different types that can be defined in SSZ
#[derive(Clone, Debug)]
pub enum TypeDefinition {
    /// Boolean type
    Boolean,
    /// Unsigned integer with specified bit width
    UInt(usize),
    /// Fixed-length vector
    Vector,
    /// Variable-length list
    List,
    /// Fixed-length vector of bits
    Bitvector,
    /// Variable-length list of bits
    Bitlist,
    /// Optional type (can be None) for use in stable containers only
    Optional,
    /// Union type
    Union,
    /// Fixed-length byte array
    Bytes(usize),
    /// Custom user-defined type
    CustomType(Box<TypeResolution>),
}

/// Represents the different class definitions that can be used
#[derive(Clone, Debug)]
pub enum ClassDefinition {
    /// Standard container class
    Container,
    /// Stable container class
    StableContainer,
    /// Profile class
    Profile,
    /// Union class
    Union,
    /// Custom class definition
    Custom(ClassDef),
}
