use proc_macro2::TokenStream;
use quote::quote;
use sizzle_parser::ClassFieldDef;
pub mod resolver;

#[derive(Clone)]
pub enum TypeResolution {
    None,
    BaseClass(BaseClass),
    Type(syn::Type),
}

impl TypeResolution {
    pub fn is_none(&self) -> bool {
        matches!(self, TypeResolution::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_base_class(&self) -> bool {
        matches!(self, TypeResolution::BaseClass(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, TypeResolution::Type(_))
    }

    pub fn unwrap_type(self) -> syn::Type {
        match self {
            TypeResolution::Type(t) => t,
            _ => panic!("Expected type resolution to be a type"),
        }
    }

    pub fn unwrap_base_class(self) -> BaseClass {
        match self {
            TypeResolution::BaseClass(base_class) => base_class,
            _ => panic!("Expected type resolution to be a base class"),
        }
    }
}

#[derive(Clone)]
pub enum BaseClass {
    Container,
    StableContainer(Option<u64>),
    Profile(Option<String>),
}

impl BaseClass {
    pub fn is_container(&self) -> bool {
        matches!(self, BaseClass::Container)
    }

    pub fn is_stable_container(&self) -> bool {
        matches!(self, BaseClass::StableContainer(_))
    }

    pub fn is_profile(&self) -> bool {
        matches!(self, BaseClass::Profile(_))
    }
}

#[derive(Clone)]
pub struct ClassDef {
    pub base: BaseClass,
    pub fields: Vec<ClassFieldDef>,
    pub field_tokens: Vec<TokenStream>,
}

impl ClassDef {
    pub fn take(self) -> (BaseClass, Vec<ClassFieldDef>, Vec<TokenStream>) {
        (self.base, self.fields, self.field_tokens)
    }

    pub fn is_container(&self) -> bool {
        self.base.is_container()
    }

    pub fn is_stable_container(&self) -> bool {
        self.base.is_stable_container()
    }

    pub fn is_profile(&self) -> bool {
        self.base.is_profile()
    }

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

#[derive(Clone)]
pub enum TypeDefinition {
    Boolean,
    UInt(usize),
    Vector,
    List,
    Bitvector,
    Bitlist,
    Optional,
    Union,
    Bytes(usize),
    CustomType(TypeResolution),
}

#[derive(Clone)]
pub enum ClassDefinition {
    Container,
    StableContainer,
    Profile,
    Custom(ClassDef),
}
