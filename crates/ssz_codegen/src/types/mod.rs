//! The types used in the SSZ codegen

use proc_macro2::TokenStream;
use quote::quote;
use sizzle_parser::ClassFieldDef;
pub mod resolver;

/// Represents the resolution status of a type reference
#[derive(Clone, Debug)]
pub enum TypeResolution {
    /// Type has not been resolved yet
    None,
    /// Type resolves to a base class
    BaseClass(BaseClass),
    /// Type resolves to a Rust type
    Type(Box<syn::Type>),
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
        matches!(self, TypeResolution::Type(_))
    }

    /// Unwraps the Type variant, panics if not a Type
    ///
    /// # Returns
    ///
    /// The unwrapped syn::Type if this is a Type variant
    ///
    /// # Panics
    ///
    /// Panics if the resolution is not a Type
    pub fn unwrap_type(self) -> Box<syn::Type> {
        match self {
            TypeResolution::Type(t) => t,
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
}

/// Represents the base class types for SSZ data structures
#[derive(Clone, Debug)]
pub enum BaseClass {
    /// A container type
    Container,
    /// A stable container with optional maximum field count
    StableContainer(Option<u64>),
    /// A profile type with optional name
    Profile(Option<String>),
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

/// Definition of a class with its base type and fields
#[derive(Clone, Debug)]
pub struct ClassDef {
    /// The base class type
    pub base: BaseClass,
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
        let derives = match self.base {
            BaseClass::Container => {
                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="container")]
                    #[tree_hash(struct_behaviour="container")]
                }
            }
            BaseClass::StableContainer(max) => {
                let max = format!("typenum::U{}", max.unwrap());
                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="stable_container", max_fields=#max)]
                    #[tree_hash(struct_behaviour="stable_container", max_fields=#max)]
                }
            }
            BaseClass::Profile(_) => {
                quote! {
                    #[derive(Encode, Decode, TreeHash)]
                    #[ssz(struct_behaviour="profile")]
                    #[tree_hash(struct_behaviour="profile")]
                }
            }
        };

        let field_tokens = &self.field_tokens;
        quote! {
            #derives
            pub struct #ident {
                #(#field_tokens),*
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
