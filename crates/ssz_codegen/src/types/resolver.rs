use super::{BaseClass, ClassDef, TypeResolution, TypeDefinition, ClassDefinition};
use proc_macro2::TokenStream;
use quote::quote;
use sizzle_parser::tysys::{Ty, TyExpr};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use syn::parse_quote;

pub fn primitive_rust_type(base_name: &str) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path::from(syn::Ident::new(base_name, proc_macro2::Span::call_site())),
    })
}

pub struct TypeResolver {
    pub types: HashMap<String, TypeDefinition>,
    pub classes: HashMap<String, ClassDefinition>,
    pub base_classes: HashMap<String, BaseClass>,
    pub union_tracker: Rc<RefCell<HashMap<u64, TokenStream>>>,
}

impl TypeResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            types: HashMap::new(),
            classes: HashMap::new(),
            base_classes: HashMap::new(),
            union_tracker: Rc::new(RefCell::new(HashMap::new())),
        };

        // Built-in types
        resolver.types.insert("boolean".to_string(), TypeDefinition::Boolean);
        for i in [8, 16, 32, 64, 128, 256] {
            let uint_name = format!("uint{}", i);
            resolver.types.insert(uint_name, TypeDefinition::UInt(i));
        }
        resolver.types.insert("bit".to_string(), TypeDefinition::Boolean);
        resolver.types.insert("null".to_string(), TypeDefinition::Boolean);
        resolver.types.insert("byte".to_string(), TypeDefinition::UInt(8));
        resolver.types.insert("Vector".to_string(), TypeDefinition::Vector);
        resolver.types.insert("List".to_string(), TypeDefinition::List);
        resolver.types.insert("Bitvector".to_string(), TypeDefinition::Bitvector);
        resolver.types.insert("Bitlist".to_string(), TypeDefinition::Bitlist);
        resolver.types.insert("Optional".to_string(), TypeDefinition::Optional);
        resolver.types.insert("Union".to_string(), TypeDefinition::Union);
        for i in 1..=64 {
            let bytes_name = format!("Bytes{}", i);
            resolver.types.insert(bytes_name, TypeDefinition::Bytes(i));
        }

        // Built-in classes
        resolver.base_classes.insert("Container".to_string(), BaseClass::Container);
        resolver.base_classes.insert("StableContainer".to_string(), BaseClass::StableContainer(None));
        resolver.base_classes.insert("Profile".to_string(), BaseClass::Profile(None));
        resolver.classes.insert("Container".to_string(), ClassDefinition::Container);
        resolver.classes.insert("StableContainer".to_string(), ClassDefinition::StableContainer);
        resolver.classes.insert("Profile".to_string(), ClassDefinition::Profile);

        resolver
    }

    pub fn resolve_type(&self, ty: &Ty) -> TypeResolution {
        // Check if the type is a base class (Container, StableContainer, Profile or aliases to them)
        let base_class = self.resolve_base_class(ty);
        if let Some(base_class) = base_class {
            return base_class;
        }

        // Extract the type arguments
        let args = match ty {
            Ty::Simple(_) => vec![],
            Ty::Complex(_, args) => {
                let mut resolved_args = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    match self.resolve_type_expr(arg) {
                        TypeResolution::Type(resolved) => resolved_args.push(resolved),
                        TypeResolution::None => return TypeResolution::None,
                        TypeResolution::BaseClass(_) => {
                            panic!("BaseClass in type arguments are not allowed")
                        }
                    }
                }
                resolved_args
            }
        };

        // Resolve the type definition using the type arguments
        let type_def = self.types.get(ty.base_name().0.as_str());
        match type_def {
            Some(def) => self.resolve_type_definition(def, args),
            None => TypeResolution::None,
        }
    }

    fn resolve_type_expr(&self, ty_expr: &TyExpr) -> TypeResolution {
        match ty_expr {
            TyExpr::Ty(ty) => self.resolve_type(ty),
            TyExpr::Int(int) => {
                let typenum_int = syn::Ident::new(&format!("U{}", int.eval()), proc_macro2::Span::call_site());
                TypeResolution::Type(parse_quote!(typenum::#typenum_int))
            }
        }
    }

    fn resolve_base_class(&self, ty: &Ty) -> Option<TypeResolution> {
        let base_class = self.base_classes.get(ty.base_name().0.as_str());
        let result = base_class.map(|base_class| {
            match base_class {
                BaseClass::Container => TypeResolution::BaseClass(BaseClass::Container),
                BaseClass::StableContainer(max) => {
                    let max = max.unwrap_or({
                        match ty {
                            Ty::Simple(_) => panic!("Stable container must have a max field count as first argument"),
                            Ty::Complex(_, args) => {
                                match args.first() {
                                    Some(TyExpr::Int(int)) => int.eval(),
                                    _ => panic!("Stable container must have a max field count as first argument"),
                                }
                            },
                        }
                    });
                    TypeResolution::BaseClass(BaseClass::StableContainer(Some(max)))
                }
                BaseClass::Profile(name) => {
                    let name = name.clone().unwrap_or({
                        match ty {
                            Ty::Simple(_) => panic!("Profile must inherit from a stable container"),
                            Ty::Complex(_, args) => match args.first() {
                                Some(TyExpr::Ty(ty)) => ty.base_name().0.clone(),
                                _ => panic!("Profile must inherit from a class"),
                            },
                        }
                    });
                    TypeResolution::BaseClass(BaseClass::Profile(Some(name)))
                }
            }
        });
        result
    }

    fn resolve_type_definition(&self, def: &TypeDefinition, args: Vec<syn::Type>) -> TypeResolution {
        match def {
            TypeDefinition::Boolean => TypeResolution::Type(primitive_rust_type("bool")),
            TypeDefinition::UInt(size) => TypeResolution::Type(primitive_rust_type(&format!("u{}", size))),
            TypeDefinition::Vector => TypeResolution::Type(parse_quote!(FixedVector<#(#args),*>)),
            TypeDefinition::List => TypeResolution::Type(parse_quote!(VariableList<#(#args),*>)),
            TypeDefinition::Bitvector => TypeResolution::Type(parse_quote!(BitVector<#(#args),*>)),
            TypeDefinition::Bitlist => TypeResolution::Type(parse_quote!(BitList<#(#args),*>)),
            TypeDefinition::Optional => TypeResolution::Type(parse_quote!(Option<#(#args),*>)),
            TypeDefinition::Union => {
                // Since unions are represented as enums in rust, we need to generate a unique identifier for each union
                // Here we hash the type arguments to generate a unique identifier
                let mut hasher = DefaultHasher::new();
                args.iter().for_each(|ty| {
                    ty.hash(&mut hasher);
                });
                let hash = hasher.finish();
                let ident = syn::Ident::new(&format!("Union_{}", hash), proc_macro2::Span::call_site());

                // If we haven't seen this union before, we need to generate the enum definition
                if !self.union_tracker.borrow().contains_key(&hash) {
                    // Generate the enum variants Selector0, Selector1, etc. and insert the union into our union tracker
                    let variants: Vec<syn::Variant> = args
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let ident = syn::Ident::new(&format!("Selector{}", i), proc_macro2::Span::call_site());
                            parse_quote!(#ident(#ty))
                        })
                        .collect::<Vec<_>>();

                    self.union_tracker.borrow_mut().insert(
                        hash,
                        quote! {
                            #[derive(Encode, Decode, TreeHash)]
                            #[ssz(enum_behaviour="union")]
                            #[tree_hash(enum_behaviour="union")]
                            pub enum #ident {
                                #(#variants),*
                            }
                        },
                    );
                }

                TypeResolution::Type(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path::from(ident),
                }))
            },
            TypeDefinition::Bytes(size) => {
                let typenum_int = syn::Ident::new(&format!("U{}", size), proc_macro2::Span::call_site());
                TypeResolution::Type(parse_quote!(FixedVector<u8, typenum::#typenum_int>))
            },
            TypeDefinition::CustomType(resolution) => resolution.clone(),
        }
    }

    pub fn resolve_class(&self, ty: &Ty) -> Option<ClassDef> {
        let class_def = self.classes.get(ty.base_name().0.as_str())?;
        
        let args = match ty {
            Ty::Simple(_) => vec![],
            Ty::Complex(_, args) => args.clone(),
        };
        
        Some(self.resolve_class_definition(class_def, &args))
    }

    fn resolve_class_definition(&self, def: &ClassDefinition, args: &[TyExpr]) -> ClassDef {
        match def {
            ClassDefinition::Container => ClassDef {
                base: BaseClass::Container,
                fields: vec![],
                field_tokens: vec![],
            },
            ClassDefinition::StableContainer => {
                let max = match args.first() {
                    Some(TyExpr::Int(int)) => int.eval(),
                    _ => panic!("Expected stable container to have a max field count as first argument"),
                };

                ClassDef {
                    base: BaseClass::StableContainer(Some(max)),
                    fields: vec![],
                    field_tokens: vec![],
                }
            },
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
                    },
                    _ => panic!("Expected profile to inherit from a class"),
                };

                ClassDef {
                    base: BaseClass::Profile(Some(name)),
                    fields: class_def.fields,
                    field_tokens: class_def.field_tokens,
                }
            },
            ClassDefinition::Custom(class_def) => class_def.clone(),
        }
    }

    pub fn resolve_type_and_add(&mut self, ty: &Ty, alias_ident: &syn::Ident) -> TypeResolution {
        let resolved = self.resolve_type(ty);

        let alias_str = alias_ident.to_string();
        if resolved.is_type() && !self.types.contains_key(&alias_str) {
            // Add the new type to the types map so it can be referenced by other types
            self.types.insert(
                alias_str.clone(), 
                TypeDefinition::CustomType(resolved.clone())
            );
        };

        if resolved.is_base_class() && !self.base_classes.contains_key(&alias_str) {
            // Add the base class to the base classes map so it can be aliased if needed
            let base_class = resolved.clone().unwrap_base_class();
            self.base_classes.insert(alias_str.clone(), base_class.clone());

            // Add the base class as an empty version of the base class itself for inheritance purposes
            let class_def = match &base_class {
                BaseClass::Container => {
                    ClassDefinition::Custom(ClassDef {
                        base: BaseClass::Container,
                        fields: vec![],
                        field_tokens: vec![],
                    })
                },
                BaseClass::StableContainer(Some(max)) => {
                    ClassDefinition::Custom(ClassDef {
                        base: BaseClass::StableContainer(Some(*max)),
                        fields: vec![],
                        field_tokens: vec![],
                    })
                },
                BaseClass::Profile(Some(name)) => {
                    let class_def = self.classes.get(name).unwrap();
                    let resolved_def = self.resolve_class_definition(class_def, &[]);
                    ClassDefinition::Custom(ClassDef {
                        base: BaseClass::Profile(Some(name.clone())),
                        fields: resolved_def.fields,
                        field_tokens: resolved_def.field_tokens,
                    })
                },
                _ => panic!("Expected base class alias to have the necessary fields for the base class it aliases"),
            };
            
            self.classes.insert(alias_str, class_def);
        }

        resolved
    }

    pub fn add_class(&mut self, class_ident: &syn::Ident, class_def: ClassDef) {
        let class_str = class_ident.to_string();

        // Add the class to the classes map so it can be inherited from
        if !self.classes.contains_key(&class_str) {
            self.classes.insert(
                class_str.clone(), 
                ClassDefinition::Custom(class_def)
            );
        }

        // Add the class to the types map so it can be referenced by other types
        if !self.types.contains_key(&class_str) {
            let class_type = TypeResolution::Type(syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path::from(class_ident.clone()),
            }));
            
            self.types.insert(class_str, TypeDefinition::CustomType(class_type));
        }
    }
}
