//! The types used in the SSZ codegen

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Path, Type, TypePath, parse_quote};

use crate::{derive_config::DeriveConfig, pragma::ParsedPragma, types::resolver::TypeResolver};
pub mod resolver;

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
    /// Fixed-length vector
    Vector(Box<TypeResolution>, u64),
    /// Variable-length list
    List(Box<TypeResolution>, u64),
    /// Fixed-length vector of bits
    Bitvector(u64),
    /// Variable-length list of bits
    Bitlist(u64),
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
            TypeResolutionKind::UInt(size) => primitive_rust_type(&format!("u{size}")),
            TypeResolutionKind::Vector(ty, size) => {
                let size = *size as usize;
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
            TypeResolutionKind::List(ty, size) => {
                let ty = ty.unwrap_type();
                let size = *size as usize;
                parse_quote!(VariableList<#ty, #size>)
            }
            TypeResolutionKind::Bitvector(size) => {
                let size = *size as usize;
                parse_quote!(BitVector<#size>)
            }
            TypeResolutionKind::Bitlist(size) => {
                let size = *size as usize;
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
            TypeResolutionKind::External => {
                // For external types, use the stored syn::Type
                if let Some(ty) = &self.ty {
                    ty.clone()
                } else {
                    panic!("External type without stored syn::Type")
                }
            }
            TypeResolutionKind::Unresolved => {
                // For unresolved types (type parameters), use the stored syn::Type
                if let Some(ty) = &self.ty {
                    ty.clone()
                } else {
                    panic!("Unresolved type without stored syn::Type")
                }
            }
            _ => panic!("Expected type resolution to be a type"),
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
            TypeResolutionKind::Bitvector(bits) => (*bits as usize).div_ceil(8), /* Round up to */
            // bytes
            TypeResolutionKind::Vector(inner, count) => {
                if inner.is_fixed_size() {
                    inner.fixed_size() * (*count as usize)
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
        self.to_view_type_inner(false)
    }

    /// Internal helper that can distinguish between direct field context and list/vector context
    #[allow(
        clippy::only_used_in_recursion,
        reason = "Parameter is intentionally passed through to nested types"
    )]
    fn to_view_type_inner(&self, in_collection: bool) -> Type {
        match &self.resolution {
            TypeResolutionKind::Class(class) => {
                if let Some(Type::Path(TypePath { path, .. })) = &self.ty {
                    let mut path_segments = path.segments.clone();
                    // Strip `crate::ssz::` prefix if present (all generated code is inside
                    // `ssz` module)
                    if let Some(converted_path) = convert_crate_ssz_to_super(path) {
                        path_segments = converted_path.segments;
                    }

                    // Extract generic arguments from the last segment if present
                    let generic_args = if let Some(last_segment) = path_segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            Some(args.args.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(last_segment) = path_segments.last_mut() {
                        let ref_ident = Ident::new(
                            &format!("{}Ref", last_segment.ident),
                            last_segment.ident.span(),
                        );
                        last_segment.ident = ref_ident;

                        // Build new generic arguments: <'a, ...existing_args>
                        use syn::{GenericArgument, Lifetime};
                        let lifetime =
                            GenericArgument::Lifetime(Lifetime::new("'a", Span::call_site()));
                        let mut new_args = syn::punctuated::Punctuated::new();
                        new_args.push(lifetime);

                        if let Some(args) = generic_args {
                            new_args.extend(args);
                        }

                        last_segment.arguments = syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: syn::token::Lt::default(),
                                args: new_args,
                                gt_token: syn::token::Gt::default(),
                            },
                        );
                    }
                    let view_path = Path {
                        leading_colon: path.leading_colon,
                        segments: path_segments,
                    };
                    parse_quote! { #view_path }
                } else {
                    let ref_ident = Ident::new(&format!("{}Ref", class), Span::call_site());
                    parse_quote!(#ref_ident<'a>)
                }
            }
            TypeResolutionKind::Boolean => primitive_rust_type("bool"),
            TypeResolutionKind::UInt(size) => primitive_rust_type(&format!("u{size}")),
            TypeResolutionKind::Vector(ty, size) => {
                let size = *size as usize;
                // Special case: Vector[byte, N] -> FixedBytesRef<'a, N>
                // This ensures proper trait implementations when used in Lists
                if matches!(ty.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(FixedBytesRef<'a, #size>)
                } else {
                    let inner_view_ty = ty.to_view_type_inner(true);
                    parse_quote!(FixedVectorRef<'a, #inner_view_ty, #size>)
                }
            }
            TypeResolutionKind::List(ty, size) => {
                let inner = &**ty;
                let size = *size as usize;

                // Special case: List<u8, N> -> BytesRef<'a>
                if matches!(inner.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(BytesRef<'a>)
                } else {
                    let inner_view_ty = ty.to_view_type_inner(true);
                    parse_quote!(VariableListRef<'a, #inner_view_ty, #size>)
                }
            }
            TypeResolutionKind::Bitvector(size) => {
                let size = *size as usize;
                parse_quote!(BitVectorRef<'a, #size>)
            }
            TypeResolutionKind::Bitlist(size) => {
                let size = *size as usize;
                parse_quote!(BitListRef<'a, #size>)
            }
            TypeResolutionKind::Optional(ty) => {
                let inner_view_ty = ty.to_view_type_inner(in_collection);
                parse_quote!(Optional<#inner_view_ty>)
            }
            TypeResolutionKind::Option(ty) => {
                let inner_view_ty = ty.to_view_type_inner(in_collection);
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
                // For external types, use Ref variant if the type name suggests it's a container
                // (ends with "Payload", "Message", "State", "Proof", "Claim", etc.)
                // Otherwise, use the type itself (for primitive-like types that implement
                // DecodeView directly)
                if let Some(ty) = &self.ty {
                    match ty {
                        Type::Path(TypePath { path, .. }) => {
                            if let Some(last_segment) = path.segments.last() {
                                let name = last_segment.ident.to_string();
                                // Check if type name suggests it's a container that needs Ref
                                if name.ends_with("Payload")
                                    || name.ends_with("Message")
                                    || name.ends_with("State")
                                    || name.ends_with("Proof")
                                    || name.ends_with("Claim")
                                {
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
                                    parse_quote! {
                                        #view_path<'a>
                                    }
                                } else {
                                    // For primitive-like types, use the type itself
                                    self.unwrap_type()
                                }
                            } else {
                                self.unwrap_type()
                            }
                        }
                        _ => self.unwrap_type(),
                    }
                } else {
                    self.unwrap_type()
                }
            }
            TypeResolutionKind::Unresolved => {
                // For unresolved types, we need to distinguish between:
                // 1. Simple type parameters like `H` - use as-is
                // 2. Generic class references like `RawMerkleProof<H>` - convert to view type

                if let Some(Type::Path(TypePath { path, .. })) = &self.ty {
                    let last_segment = path.segments.last();

                    // Check if this looks like a class reference (not a simple type parameter)
                    // Simple type parameters are single identifiers, while class references might
                    // have:
                    // - Multiple segments (e.g., `module::Class`)
                    // - Or generic arguments (e.g., `Class<T>`)
                    let is_likely_class = path.segments.len() > 1
                        || (last_segment.is_some()
                            && matches!(
                                last_segment.unwrap().arguments,
                                syn::PathArguments::AngleBracketed(_)
                            ));

                    if is_likely_class {
                        // This looks like a generic class reference, convert to view type
                        let mut path_segments = path.segments.clone();

                        // Extract generic arguments from the last segment if present
                        let generic_args = if let Some(last_seg) = path_segments.last() {
                            if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                                Some(args.args.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(last_segment) = path_segments.last_mut() {
                            // Change class name to ClassRef
                            let ref_ident = Ident::new(
                                &format!("{}Ref", last_segment.ident),
                                last_segment.ident.span(),
                            );
                            last_segment.ident = ref_ident;

                            // Build new generic arguments: <'a, ...existing_args>
                            use syn::{GenericArgument, Lifetime};
                            let lifetime =
                                GenericArgument::Lifetime(Lifetime::new("'a", Span::call_site()));
                            let mut new_args = syn::punctuated::Punctuated::new();
                            new_args.push(lifetime);

                            if let Some(args) = generic_args {
                                new_args.extend(args);
                            }

                            last_segment.arguments = syn::PathArguments::AngleBracketed(
                                syn::AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: syn::token::Lt::default(),
                                    args: new_args,
                                    gt_token: syn::token::Gt::default(),
                                },
                            );
                        }

                        let view_path = Path {
                            leading_colon: path.leading_colon,
                            segments: path_segments,
                        };
                        parse_quote! { #view_path }
                    } else {
                        // Simple type parameter, use as-is
                        self.unwrap_type()
                    }
                } else {
                    // No stored type, use as-is
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

/// Type parameter in a generic class definition
#[derive(Clone, Debug)]
pub struct TypeParam {
    /// Name of the type parameter (e.g., "H", "T", "N")
    pub name: String,
    /// Type of the parameter (Type or Const)
    pub kind: TypeParamKind,
}

/// Kind of type parameter
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeParamKind {
    /// Type variable (e.g., T, U, H)
    Type,
    /// Const variable (e.g., N for array sizes)
    Const,
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
    /// Type parameters for generic classes
    pub type_params: Vec<TypeParam>,
}

impl ClassDef {
    /// Build generic type parameters with trait bounds
    ///
    /// Returns a TokenStream for generic parameters like `<H: MerkleHash, N: usize>`
    fn build_generic_params(&self, pragmas: &crate::pragma::ParsedPragma) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            return quote! {};
        }

        let params: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());

                match tp.kind {
                    TypeParamKind::Type => {
                        // Type parameters get trait bounds from pragmas
                        // Default bounds: ssz::Encode + ssz::Decode for all type params
                        let mut bounds: Vec<TokenStream> =
                            vec![parse_quote!(ssz::Encode), parse_quote!(ssz::Decode)];

                        // Add custom bounds from pragmas
                        if let Some(pragma_bounds) = pragmas.type_param_bounds.get(&tp.name) {
                            for bound_str in pragma_bounds {
                                // Parse the bound string as a path (e.g., "MerkleHash" or
                                // "tree_hash::TreeHash")
                                if let Ok(bound_path) = syn::parse_str::<syn::Path>(bound_str) {
                                    bounds.push(quote! { #bound_path });
                                }
                            }
                        }

                        quote! { #param_ident: #(#bounds)+* }
                    }
                    TypeParamKind::Const => {
                        // Const parameters are always usize
                        quote! { const #param_ident: usize }
                    }
                }
            })
            .collect();

        quote! { <#(#params),*> }
    }

    /// Build generic parameters for view structs (includes lifetime + type params)
    fn build_view_generic_params(&self, pragmas: &crate::pragma::ParsedPragma) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            // No type params, just lifetime
            return quote! { <'a> };
        }

        // Build type parameters with bounds
        let type_params: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());

                match tp.kind {
                    TypeParamKind::Type => {
                        // Type parameters get trait bounds from pragmas
                        // Default bounds for view types: need all SSZ traits plus lifetime
                        // These are required for types to be usable in collections and for
                        // to_owned()
                        let mut bounds: Vec<TokenStream> = vec![
                            parse_quote!(ssz::Encode),
                            parse_quote!(ssz::Decode),
                            parse_quote!(ssz::view::DecodeView<'a>),
                            parse_quote!(ssz::view::SszTypeInfo),
                            parse_quote!('a), // Lifetime bound to ensure H lives as long as 'a
                        ];

                        // Add custom bounds from pragmas
                        if let Some(pragma_bounds) = pragmas.type_param_bounds.get(&tp.name) {
                            for bound_str in pragma_bounds {
                                // Parse the bound string as a path (e.g., "MerkleHash" or
                                // "tree_hash::TreeHash")
                                if let Ok(bound_path) = syn::parse_str::<syn::Path>(bound_str) {
                                    bounds.push(quote! { #bound_path });
                                }
                            }
                        }

                        quote! { #param_ident: #(#bounds)+* }
                    }
                    TypeParamKind::Const => {
                        // Const parameters are always usize
                        quote! { const #param_ident: usize }
                    }
                }
            })
            .collect();

        // View structs have lifetime 'a first, then type params
        quote! { <'a, #(#type_params),*> }
    }

    /// Build generic parameters without bounds for type references (just names for instantiation)
    fn build_view_generic_names(&self) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            // No type params, just lifetime
            return quote! { <'a> };
        }

        // Build just the parameter names without bounds
        let param_names: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());
                match tp.kind {
                    TypeParamKind::Type => quote! { #param_ident },
                    TypeParamKind::Const => quote! { #param_ident },
                }
            })
            .collect();

        // View type names have: <'a, type_param_names>
        quote! { <'a, #(#param_names),*> }
    }

    /// Build generic parameter names for owned types (without bounds)
    fn build_generic_names(&self, _pragmas: &crate::pragma::ParsedPragma) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            return quote! {};
        }

        // Build just the parameter names without bounds
        let param_names: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());
                match tp.kind {
                    TypeParamKind::Type => quote! { #param_ident },
                    TypeParamKind::Const => quote! { #param_ident },
                }
            })
            .collect();

        quote! { <#(#param_names),*> }
    }

    /// Build generic parameters for TreeHash impl of owned structs (includes H and type params with
    /// bounds)
    fn build_tree_hash_owned_generic_params(
        &self,
        pragmas: &crate::pragma::ParsedPragma,
    ) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            // No type params, just H
            return quote! { <H: tree_hash::TreeHashDigest> };
        }

        // For TreeHash impl on owned types, add TreeHashDigest bound to type params
        let type_params: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());

                match tp.kind {
                    TypeParamKind::Type => {
                        // Start with TreeHashDigest, TreeHash and Encode/Decode bounds
                        let mut bounds: Vec<TokenStream> = vec![
                            parse_quote!(tree_hash::TreeHashDigest),
                            parse_quote!(tree_hash::TreeHash),
                            parse_quote!(ssz::Encode),
                            parse_quote!(ssz::Decode),
                        ];

                        // Add custom bounds from pragmas
                        if let Some(pragma_bounds) = pragmas.type_param_bounds.get(&tp.name) {
                            for bound_str in pragma_bounds {
                                if let Ok(bound_path) = syn::parse_str::<syn::Path>(bound_str) {
                                    bounds.push(quote! { #bound_path });
                                }
                            }
                        }

                        quote! { #param_ident: #(#bounds)+* }
                    }
                    TypeParamKind::Const => {
                        quote! { const #param_ident: usize }
                    }
                }
            })
            .collect();

        // TreeHash impl has: <type_params> (H is in type_params)
        quote! { <#(#type_params),*> }
    }

    /// Build generic parameters for TreeHash impl of view structs (includes 'a, H, and type params)
    fn build_tree_hash_view_generic_params(
        &self,
        pragmas: &crate::pragma::ParsedPragma,
    ) -> TokenStream {
        use syn::Ident;

        if self.type_params.is_empty() {
            // No type params, just 'a and H (add H for the TreeHash trait parameter)
            return quote! { <'a, H: tree_hash::TreeHashDigest> };
        }

        // For TreeHash impl, we need TreeHashDigest + TreeHash bounds on type params
        // that will be used as the hash digest type
        let type_params: Vec<TokenStream> = self
            .type_params
            .iter()
            .map(|tp| {
                let param_ident = Ident::new(&tp.name, Span::call_site());

                match tp.kind {
                    TypeParamKind::Type => {
                        // Start with TreeHashDigest, TreeHash, and SSZ bounds for all type params
                        let mut bounds: Vec<TokenStream> = vec![
                            parse_quote!(tree_hash::TreeHashDigest),
                            parse_quote!(tree_hash::TreeHash),
                            parse_quote!(ssz::Encode),
                            parse_quote!(ssz::Decode),
                            parse_quote!(ssz::view::DecodeView<'a>),
                            parse_quote!(ssz::view::SszTypeInfo),
                            parse_quote!('a),
                        ];

                        // Add custom bounds from pragmas
                        if let Some(pragma_bounds) = pragmas.type_param_bounds.get(&tp.name) {
                            for bound_str in pragma_bounds {
                                if let Ok(bound_path) = syn::parse_str::<syn::Path>(bound_str) {
                                    bounds.push(quote! { #bound_path });
                                }
                            }
                        }

                        quote! { #param_ident: #(#bounds)+* }
                    }
                    TypeParamKind::Const => {
                        quote! { const #param_ident: usize }
                    }
                }
            })
            .collect();

        // TreeHash impl has: <'a, type_params>
        // We implement TreeHash<TypeParam> where TypeParam is the first type parameter
        quote! { <'a, #(#type_params),*> }
    }

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

        // Build generic type parameters with bounds
        let generics = self.build_generic_params(&pragmas);

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
                    pub struct #ident #generics {
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
                    pub struct #ident #generics {
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
                    pub struct #ident #generics {
                        #(#field_tokens),*
                    }
                }
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

        // Build generic parameters for view struct (includes lifetime + type params)
        let view_generics = self.build_view_generic_params(&pragmas);

        // All view structs are now thin wrappers around bytes
        // Add PhantomData for type parameters if any
        let type_params: Vec<syn::Ident> = self
            .type_params
            .iter()
            .filter_map(|tp| {
                if matches!(tp.kind, TypeParamKind::Type) {
                    Some(syn::Ident::new(&tp.name, proc_macro2::Span::call_site()))
                } else {
                    None
                }
            })
            .collect();

        match self.base {
            BaseClass::Container | BaseClass::StableContainer(_) | BaseClass::Profile(_) => {
                if type_params.is_empty() {
                    quote! {
                        #doc_comments
                        #[allow(dead_code, reason = "generated code using ssz-gen")]
                        #view_derive
                        pub struct #ref_ident #view_generics {
                            bytes: &'a [u8],
                        }
                    }
                } else {
                    // Use a tuple of all type parameters in PhantomData
                    quote! {
                        #doc_comments
                        #[allow(dead_code, reason = "generated code using ssz-gen")]
                        #view_derive
                        pub struct #ref_ident #view_generics {
                            bytes: &'a [u8],
                            _phantom: core::marker::PhantomData<(#(#type_params,)*)>,
                        }
                    }
                }
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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);

        // Build generic params for TreeHash impl (includes 'a, H, and type params)
        let tree_hash_impl_generics = self.build_tree_hash_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();

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
                    impl #tree_hash_impl_generics tree_hash::TreeHash<H> for #ref_ident #view_generic_names {
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
                    impl #tree_hash_impl_generics tree_hash::TreeHash<H> for #ref_ident #view_generic_names {
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
                    impl #tree_hash_impl_generics tree_hash::TreeHash<H> for #ref_ident #view_generic_names {
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

        // Get generic parameter names for the struct
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let generic_names = self.build_generic_names(&pragmas);
        let tree_hash_impl_generics = self.build_tree_hash_owned_generic_params(&pragmas);

        match self.base {
            BaseClass::Container => {
                let num_leaves = field_names.len();
                quote! {
                    impl #tree_hash_impl_generics tree_hash::TreeHash<H> for #ident #generic_names {
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
                            if let Some(ref #field_name) = self.#field_name {
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
                    impl #tree_hash_impl_generics tree_hash::TreeHash<H> for #ident #generic_names {
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
                            let mut active_fields = BitVector::<#max>::new();

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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let view_generics = self.build_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();
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
                let view_ty = field.ty.to_view_type();
                let (offset_expr, is_fixed, size_expr) = self.generate_field_layout(idx);

                if is_fixed {
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
            impl #view_generics #ref_ident #view_generic_names {
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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let view_generics = self.build_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();
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

                // Check if we need to initialize _phantom field
                let struct_init = if self
                    .type_params
                    .iter()
                    .any(|tp| matches!(tp.kind, TypeParamKind::Type))
                {
                    quote! { Self { bytes, _phantom: core::marker::PhantomData } }
                } else {
                    quote! { Self { bytes } }
                };

                quote! {
                    impl #view_generics ssz::view::DecodeView<'a> for #ref_ident #view_generic_names {
                        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                            #validation
                            Ok(#struct_init)
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
                        // Parse bitvector
                        let bitvector_length = #bitvector_length;
                        if bytes.len() < bitvector_length {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: bitvector_length,
                            });
                        }

                        // Validate bitvector structure (don't need to store it)
                        let _bitvector = ssz::BitVector::<#max_fields_usize>::from_ssz_bytes(
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

                // Check if we need to initialize _phantom field
                let struct_init = if self
                    .type_params
                    .iter()
                    .any(|tp| matches!(tp.kind, TypeParamKind::Type))
                {
                    quote! { Self { bytes, _phantom: core::marker::PhantomData } }
                } else {
                    quote! { Self { bytes } }
                };

                quote! {
                    impl #view_generics ssz::view::DecodeView<'a> for #ref_ident #view_generic_names {
                        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                            #validation
                            Ok(#struct_init)
                        }
                    }
                }
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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let view_generics = self.build_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();
        let (_, fixed_size, _) = self.compute_layout();

        match fixed_size {
            Some(size) => {
                // Fixed-size container
                quote! {
                    impl #view_generics ssz::view::SszTypeInfo for #ref_ident #view_generic_names {
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
                    impl #view_generics ssz::view::SszTypeInfo for #ref_ident #view_generic_names {
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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let view_generics = self.build_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();
        let owned_generic_names = self.build_generic_names(&pragmas);

        quote! {
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl #view_generics ssz_types::view::ToOwnedSsz<#ident #owned_generic_names> for #ref_ident #view_generic_names {
                #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
                fn to_owned(&self) -> #ident #owned_generic_names {
                    <#ref_ident #view_generic_names>::to_owned(self)
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
        use crate::pragma::ParsedPragma;

        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
        let pragmas = ParsedPragma::parse(&self.pragmas);
        let view_generics = self.build_view_generic_params(&pragmas);
        let view_generic_names = self.build_view_generic_names();

        // For the return type, we need the owned struct's generic param names (without bounds)
        let owned_generic_names = self.build_generic_names(&pragmas);

        // Generate field conversions using getter methods
        let field_conversions: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| {
                let field_name = Ident::new(&field.name, Span::call_site());

                // All fields now use getter methods and expect()
                // We expect because if the view was constructed successfully, getters should work
                match &field.ty.resolution {
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
                    TypeResolutionKind::Unresolved => {
                        // For unresolved types (generics), to_owned() returns the owned type directly
                        // (not a Result) because they're calling the custom to_owned() impl
                        quote! {
                            #field_name: self.#field_name().expect("valid view").to_owned()
                        }
                    }
                    TypeResolutionKind::Class(_) => {
                        // For class types (including generic classes like RawMerkleProof[H]),
                        // to_owned() returns the owned type directly, not a Result
                        quote! {
                            #field_name: self.#field_name().expect("valid view").to_owned()
                        }
                    }
                    _ => {
                        // For other complex types, call getter and then to_owned()
                        quote! {
                            #field_name: self.#field_name().expect("valid view").to_owned()
                        }
                    }
                }
            })
            .collect();

        quote! {
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl #view_generics #ref_ident #view_generic_names {
                #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
                pub fn to_owned(&self) -> #ident #owned_generic_names {
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
#[allow(clippy::large_enum_variant)]
pub enum ClassDefinition {
    /// Standard container class
    Container,
    /// Stable container class
    StableContainer,
    /// Profile class
    Profile,
    /// Custom class definition
    Custom(ClassDef),
}
