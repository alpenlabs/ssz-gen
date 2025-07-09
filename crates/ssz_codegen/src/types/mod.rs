//! The types used in the SSZ codegen

use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::parse_quote;

use crate::types::resolver::TypeResolver;
pub mod resolver;

/// Converts a primitive type name into a Rust syn::Type
///
/// # Arguments
///
/// * `base_name` - The name of the primitive type (e.g., "bool", "u32")
///
/// # Returns
///
/// A syn::Type representing the Rust primitive type
pub fn primitive_rust_type(base_name: &str) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path::from(syn::Ident::new(base_name, proc_macro2::Span::call_site())),
    })
}

/// Represents the resolution status of a type reference
#[derive(Clone, Debug, PartialEq)]
pub enum TypeResolution {
    /// Type has not been resolved yet
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
    /// Union type
    Union(String, Vec<TypeResolution>),
    /// Fixed-length byte array
    Bytes(usize),
}

impl TypeResolution {
    /// Returns true if the resolution is None
    ///
    /// # Returns
    ///
    /// `true` if the resolution is None, `false` otherwise
    pub fn is_none(&self) -> bool {
        matches!(self, TypeResolution::None)
    }

    /// Returns true if the resolution is not None
    ///
    /// # Returns
    ///
    /// `true` if the resolution is not None, `false` otherwise
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Returns true if the resolution is a BaseClass
    ///
    /// # Returns
    ///
    /// `true` if the resolution is a BaseClass, `false` otherwise
    pub fn is_base_class(&self) -> bool {
        matches!(self, TypeResolution::BaseClass(_))
    }

    /// Returns true if the resolution is a Type
    ///
    /// # Returns
    ///
    /// `true` if the resolution is a Type, `false` otherwise
    pub fn is_type(&self) -> bool {
        !self.is_base_class() && !self.is_none()
    }

    /// Unwraps any of the type variants, panics if not a type variant
    ///
    /// # Arguments
    ///
    /// * `type_resolver` - The type resolver to use to resolve the type
    ///
    /// # Returns
    ///
    /// The unwrapped syn::Type if this is one of the type variants
    ///
    /// # Panics
    ///
    /// Panics if the resolution is not a type variant
    pub fn unwrap_type(&self) -> syn::Type {
        match self {
            TypeResolution::Class(class) => {
                let class = syn::Ident::new(class, proc_macro2::Span::call_site());
                parse_quote!(#class)
            }
            TypeResolution::Boolean => primitive_rust_type("bool"),
            TypeResolution::UInt(size) => primitive_rust_type(&format!("u{size}")),
            TypeResolution::Vector(ty, size) => {
                let ty = ty.unwrap_type();
                let constant = syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
                parse_quote!(FixedVector<#ty, typenum::#constant>)
            }
            TypeResolution::List(ty, size) => {
                let ty = ty.unwrap_type();
                let constant = syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
                parse_quote!(VariableList<#ty, typenum::#constant>)
            }
            TypeResolution::Bitvector(size) => {
                let constant = syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
                parse_quote!(BitVector<typenum::#constant>)
            }
            TypeResolution::Bitlist(size) => {
                let constant = syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
                parse_quote!(BitList<typenum::#constant>)
            }
            TypeResolution::Optional(ty) => {
                let ty = ty.unwrap_type();
                parse_quote!(Option<#ty>)
            }
            TypeResolution::Union(ident, _) => {
                let ident = syn::Ident::new(ident, proc_macro2::Span::call_site());
                parse_quote!(#ident)
            }
            TypeResolution::Bytes(size) => {
                let typenum_int =
                    syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
                parse_quote!(FixedVector<u8, typenum::#typenum_int>)
            }
            _ => panic!("Expected type resolution to be a type"),
        }
    }

    /// Unwraps the BaseClass variant, panics if not a BaseClass
    ///
    /// # Returns
    ///
    /// The unwrapped BaseClass if this is a BaseClass variant
    ///
    /// # Panics
    ///
    /// Panics if the resolution is not a BaseClass
    pub fn unwrap_base_class(self) -> BaseClass {
        match self {
            TypeResolution::BaseClass(base_class) => base_class,
            _ => panic!("Expected type resolution to be a base class"),
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
        resolver: &TypeResolver,
    ) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            // Fields MAY be required in Profile[B] by unwrapping them from Optional
            (TypeResolution::Optional(original_inner_ty), _) => {
                original_inner_ty.check_field_compatibility_for_profile(other, resolver)
            }

            // Bitlist[N] / Bitvector[N] field types are compatible if they share the same capacity N
            (TypeResolution::Bitvector(original_cap), TypeResolution::Bitlist(new_cap))
            | (TypeResolution::Bitlist(original_cap), TypeResolution::Bitvector(new_cap)) => {
                original_cap == new_cap
            }

            // List[T, N] / Vector[T, N] field types are compatible if T is compatible and if they also share the same capacity N
            (
                TypeResolution::List(original_ty, original_cap),
                TypeResolution::Vector(new_ty, new_cap),
            )
            | (
                TypeResolution::Vector(original_ty, original_cap),
                TypeResolution::List(new_ty, new_cap),
            ) => {
                original_ty.check_field_compatibility_for_profile(new_ty, resolver)
                    && *original_cap == *new_cap
            }

            (TypeResolution::Class(original_class), TypeResolution::Class(new_class)) => {
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
    pub fn check_field_compatibility(&self, other: &ClassDef, resolver: &TypeResolver) -> bool {
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
    pub fn to_token_stream(&self, ident: &syn::Ident) -> TokenStream {
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
                let max = format!("typenum::U{max}");
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
                let max = format!("typenum::U{max}");
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
    CustomType(TypeResolution),
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
