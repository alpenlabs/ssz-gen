//! The types used in the SSZ codegen

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, Path, Type, TypePath, parse_quote};

use crate::types::resolver::TypeResolver;
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
                let ty = ty.unwrap_type();
                let size = *size as usize;
                parse_quote!(FixedVector<#ty, #size>)
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
                parse_quote!(FixedVector<u8, #size>)
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
            TypeResolutionKind::External => false, // External types: unknown layout, treat as variable
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
            TypeResolutionKind::Bitvector(bits) => (*bits as usize).div_ceil(8), // Round up to bytes
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
        match &self.resolution {
            TypeResolutionKind::Class(class) => {
                let ref_ident = Ident::new(&format!("{}Ref", class), Span::call_site());
                parse_quote!(#ref_ident<'a>)
            }
            TypeResolutionKind::Boolean => primitive_rust_type("bool"),
            TypeResolutionKind::UInt(size) => primitive_rust_type(&format!("u{size}")),
            TypeResolutionKind::Vector(ty, size) => {
                let inner_view_ty = ty.to_view_type();
                let size = *size as usize;
                parse_quote!(FixedVectorRef<'a, #inner_view_ty, #size>)
            }
            TypeResolutionKind::List(ty, size) => {
                let inner = &**ty;
                let size = *size as usize;

                // Special case: List<u8, N> -> BytesRef<'a>
                if matches!(inner.resolution, TypeResolutionKind::UInt(8)) {
                    parse_quote!(BytesRef<'a>)
                } else {
                    let inner_view_ty = ty.to_view_type();
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
                let inner_view_ty = ty.to_view_type();
                parse_quote!(Optional<#inner_view_ty>)
            }
            TypeResolutionKind::Option(ty) => {
                let inner_view_ty = ty.to_view_type();
                parse_quote!(Option<#inner_view_ty>)
            }
            TypeResolutionKind::Union(ident, _) => {
                let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());
                parse_quote!(#ref_ident<'a>)
            }
            TypeResolutionKind::Bytes(size) => {
                parse_quote!(FixedBytesRef<'a, #size>)
            }
            TypeResolutionKind::External => {
                // External types remain as-is since we don't control them
                self.unwrap_type()
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

            // Bitlist[N] / Bitvector[N] field types are compatible if they share the same capacity N
            (TypeResolutionKind::Bitvector(original_cap), TypeResolutionKind::Bitlist(new_cap))
            | (TypeResolutionKind::Bitlist(original_cap), TypeResolutionKind::Bitvector(new_cap)) => {
                original_cap == new_cap
            }

            // List[T, N] / Vector[T, N] field types are compatible if T is compatible and if they also share the same capacity N
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
                        // Make sure they share all their fields in the same order and all their fields are compatible
                        original_class_def.check_capacity_compatibility(new_class_def)
                            && original_class_def.check_field_compatibility(new_class_def, resolver)
                    }
                    (BaseClass::Container, BaseClass::StableContainer(_))
                    | (BaseClass::StableContainer(_), BaseClass::Container)
                    | (BaseClass::Container, BaseClass::Container) => {
                        // Make sure they share all their fields in the same order and all their fields are compatible
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
                        // Get the original stable container definition the profile class was inheriting from
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
                        // Make sure the stable containers share all their fields in the same order and all their fields are compatible
                        original_stable_container_def
                            .check_capacity_compatibility(stable_container_def)
                            && original_stable_container_def
                                .check_field_compatibility(stable_container_def, resolver)
                    }
                    (
                        BaseClass::Profile(Some((original_stable_container_name, _))),
                        BaseClass::Profile(Some((new_stable_container_name, _))),
                    ) => {
                        // Get the original stable container definitions the profile classes are inheriting from
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
                        // Make sure the stable containers share all their fields in the same order and all their fields are compatible
                        // Make sure the profile classes share all their fields in the same order and all their fields are compatible
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
    /// Useful for Profile classes since during merkleization, we need the original index of the field
    pub index: usize,
    /// The name of the field
    pub name: String,
    /// The type of the field
    pub ty: TypeResolution,
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

    /// Checks if the capacity of the two StableContainer classes is compatible by checking if they are equal
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

    /// Checks if the fields of the two classes are compatible by checking order and type compatibility
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
    pub fn to_token_stream(&self, ident: &Ident) -> TokenStream {
        let field_tokens = &self.field_tokens;
        match self.base {
            BaseClass::Container => {
                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="container")]
                    #[tree_hash(struct_behaviour="container")]
                    pub struct #ident {
                        #(#field_tokens),*
                    }
                }
            }
            BaseClass::StableContainer(Some(max)) => {
                let max = max as usize;
                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="stable_container", max_fields=#max)]
                    #[tree_hash(struct_behaviour="stable_container", max_fields=#max)]
                    pub struct #ident {
                        #(#field_tokens),*
                    }
                }
            }
            BaseClass::Profile(Some((_, max))) => {
                let max = max as usize;
                let index = self
                    .fields
                    .iter()
                    .map(|field| field.index)
                    .collect::<Vec<_>>();

                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="profile")]
                    #[tree_hash(struct_behaviour="profile", max_fields=#max)]
                    pub struct #ident {
                        #(
                            #[tree_hash(stable_index = #index)]
                            #field_tokens
                        ),*
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
    /// A [`TokenStream`] containing the generated Rust code for the view struct (e.g., `FooRef<'a>`).
    pub fn to_view_struct(&self, ident: &Ident) -> TokenStream {
        let ref_ident = Ident::new(&format!("{}Ref", ident), Span::call_site());

        // All view structs are now thin wrappers around bytes
        // TreeHash will be implemented by calling getters
        match self.base {
            BaseClass::Container | BaseClass::StableContainer(_) | BaseClass::Profile(_) => {
                quote! {
                    #[derive(Debug, Copy, Clone)]
                    pub struct #ref_ident<'a> {
                        bytes: &'a [u8],
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
                                    hasher.write(#field_name.tree_hash_root().as_ref()).expect("write field");
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
                                hasher.write(#field_names.tree_hash_root().as_ref()).expect("write field");
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
                                    hasher.write(#field_names.tree_hash_root().as_ref()).expect("write field");
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
                                    #variable_index + 1
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
                                    #variable_index + 1
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
                            if let Some(prev) = prev_offset {
                                if offset < prev {
                                    return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                                }
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

                                if let Some(prev) = prev_offset {
                                    if offset < prev {
                                        return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                                    }
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
                        // For now, just ensure we have remaining bytes
                        // TODO: Could add more specific validation of offset table based on active fields
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
            _ => panic!("Base class arguments not set"),
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
                    _ => {
                        // For complex types, call getter and then to_owned()
                        quote! {
                            #field_name: self.#field_name().expect("valid view").to_owned()
                        }
                    }
                }
            })
            .collect();

        quote! {
            impl<'a> #ref_ident<'a> {
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
    /// Custom class definition
    Custom(ClassDef),
}
