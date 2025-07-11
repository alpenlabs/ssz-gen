// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Provides procedural derive macros for the `tree_hash::TreeHash` trait of `tree_hash` crate.

#![recursion_limit = "256"]
use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::convert::TryInto;
use syn::{DataEnum, DataStruct, DeriveInput, Expr, Ident, parse_macro_input};

/// The highest possible union selector value (higher values are reserved for backwards compatible
/// extensions).
const MAX_UNION_SELECTOR: u8 = 127;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tree_hash))]
struct StructOpts {
    #[darling(default)]
    struct_behaviour: Option<String>,
    #[darling(default)]
    enum_behaviour: Option<String>,
    #[darling(default)]
    max_fields: Option<usize>,
}

/// Field-level configuration.
#[derive(Debug, Default, FromMeta)]
struct FieldOpts {
    #[darling(default)]
    skip_hashing: bool,
    #[darling(default)]
    stable_index: Option<usize>,
}

const STRUCT_CONTAINER: &str = "container";
const STRUCT_STABLE_CONTAINER: &str = "stable_container";
const STRUCT_PROFILE: &str = "profile";
const STRUCT_VARIANTS: &[&str] = &[STRUCT_CONTAINER, STRUCT_STABLE_CONTAINER, STRUCT_PROFILE];

const ENUM_TRANSPARENT: &str = "transparent";
const ENUM_TRANSPARENT_STABLE: &str = "transparent_stable";
const ENUM_UNION: &str = "union";
const ENUM_VARIANTS: &[&str] = &[ENUM_TRANSPARENT, ENUM_UNION];
const NO_ENUM_BEHAVIOUR_ERROR: &str = "enums require an \"enum_behaviour\" attribute, \
    e.g., #[tree_hash(enum_behaviour = \"transparent\")]";

enum StructBehaviour {
    Container,
    StableContainer,
    Profile,
}

impl StructBehaviour {
    pub(crate) fn new(s: Option<String>) -> Option<Self> {
        s.map(|s| match s.as_ref() {
            STRUCT_CONTAINER => StructBehaviour::Container,
            STRUCT_STABLE_CONTAINER => StructBehaviour::StableContainer,
            STRUCT_PROFILE => StructBehaviour::Profile,
            other => {
                panic!("{other} is an invalid struct_behaviour, use one of: {STRUCT_VARIANTS:?}")
            }
        })
    }
}

enum EnumBehaviour {
    Transparent,
    TransparentStable,
    Union,
}

impl EnumBehaviour {
    pub(crate) fn new(s: Option<String>) -> Option<Self> {
        s.map(|s| match s.as_ref() {
            ENUM_TRANSPARENT => EnumBehaviour::Transparent,
            ENUM_TRANSPARENT_STABLE => EnumBehaviour::TransparentStable,
            ENUM_UNION => EnumBehaviour::Union,
            other => panic!("{other} is an invalid enum_behaviour, use either {ENUM_VARIANTS:?}"),
        })
    }
}

/// Return a Vec of `syn::Ident` for each named field in the struct, whilst filtering out fields
/// that should not be hashed.
///
/// # Panics
/// Any unnamed struct field (like in a tuple struct) will raise a panic at compile time.
fn get_hashable_fields(struct_data: &syn::DataStruct) -> Vec<&syn::Ident> {
    get_hashable_fields_and_their_caches(struct_data)
        .into_iter()
        .map(|(ident, _)| ident)
        .collect()
}

/// Return a Vec of the hashable fields of a struct, and each field's type and optional cache field.
fn get_hashable_fields_and_their_caches(
    struct_data: &syn::DataStruct,
) -> Vec<(&syn::Ident, syn::Type)> {
    struct_data
        .fields
        .iter()
        .filter_map(|f| {
            if should_skip_hashing(f) {
                None
            } else {
                let ident = f
                    .ident
                    .as_ref()
                    .expect("tree_hash_derive only supports named struct fields");
                Some((ident, f.ty.clone()))
            }
        })
        .collect()
}

/// Returns true if some field has an attribute declaring it should not be hashed.
///
/// The field attribute is: `#[tree_hash(skip_hashing)]`
fn should_skip_hashing(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("tree_hash") && attr.parse_args::<Ident>().unwrap() == "skip_hashing"
    })
}

fn parse_tree_hash_fields(
    struct_data: &syn::DataStruct,
) -> Vec<(&syn::Type, Option<&syn::Ident>, FieldOpts)> {
    struct_data
        .fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let ident = field.ident.as_ref();

            let field_opts_candidates = field
                .attrs
                .iter()
                .filter(|attr| {
                    attr.path()
                        .get_ident()
                        .is_some_and(|ident| *ident == "tree_hash")
                })
                .collect::<Vec<_>>();

            if field_opts_candidates.len() > 1 {
                panic!("more than one field-level \"tree_hash\" attribute provided")
            }

            let field_opts = field_opts_candidates
                .first()
                .map(|attr| FieldOpts::from_meta(&attr.meta).unwrap())
                .unwrap_or_default();

            (ty, ident, field_opts)
        })
        .collect()
}

/// Implements `tree_hash::TreeHash` for some `struct`.
///
/// Fields are hashed in the order they are defined.
#[proc_macro_derive(TreeHash, attributes(tree_hash))]
pub fn tree_hash_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let opts = StructOpts::from_derive_input(&item).unwrap();
    let enum_opt = EnumBehaviour::new(opts.enum_behaviour);
    let struct_opt = StructBehaviour::new(opts.struct_behaviour);

    match &item.data {
        syn::Data::Struct(s) => {
            if enum_opt.is_some() {
                panic!("cannot use \"enum_behaviour\" for a struct");
            }
            match struct_opt {
                Some(StructBehaviour::Container) => tree_hash_derive_struct_container(&item, s),
                Some(StructBehaviour::StableContainer) => {
                    if let Some(max_fields_value) = opts.max_fields {
                        tree_hash_derive_struct_stable_container(&item, s, max_fields_value)
                    } else {
                        panic!("stable_container requires \"max_fields\"")
                    }
                }
                Some(StructBehaviour::Profile) => {
                    if let Some(max_fields_value) = opts.max_fields {
                        tree_hash_derive_struct_profile(&item, s, max_fields_value)
                    } else {
                        panic!("profile requires \"max_fields\"")
                    }
                }
                // Default to container.
                None => tree_hash_derive_struct_container(&item, s),
            }
        }
        syn::Data::Enum(s) => {
            if struct_opt.is_some() {
                panic!("cannot use \"struct_behaviour\" for an enum");
            }
            match enum_opt.expect(NO_ENUM_BEHAVIOUR_ERROR) {
                EnumBehaviour::Transparent => tree_hash_derive_enum_transparent(
                    &item,
                    s,
                    syn::parse_str("Container").unwrap(),
                ),
                EnumBehaviour::TransparentStable => tree_hash_derive_enum_transparent(
                    &item,
                    s,
                    syn::parse_str("StableContainer").unwrap(),
                ),
                EnumBehaviour::Union => tree_hash_derive_enum_union(&item, s),
            }
        }
        _ => panic!("tree_hash_derive only supports structs and enums."),
    }
}

fn tree_hash_derive_struct_container(item: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

    let idents = get_hashable_fields(struct_data);
    let num_leaves = idents.len();

    let output = quote! {
        impl #impl_generics tree_hash::TreeHash for #name #ty_generics #where_clause {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::Container
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                let mut hasher = tree_hash::MerkleHasher::with_leaves(#num_leaves);

                #(
                    hasher.write(self.#idents.tree_hash_root().as_slice())
                        .expect("tree hash derive should not apply too many leaves");
                )*

                hasher.finish().expect("tree hash derive should not have a remaining buffer")
            }
        }
    };
    output.into()
}

fn tree_hash_derive_struct_stable_container(
    item: &DeriveInput,
    struct_data: &DataStruct,
    max_fields: usize,
) -> TokenStream {
    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

    let idents = get_hashable_fields(struct_data);

    for (ty, _, _) in parse_tree_hash_fields(struct_data) {
        ty_inner_type("Optional", ty).expect("Use Optional<T> for StableContainer");
    }

    let output = quote! {
        impl #impl_generics tree_hash::TreeHash for #name #ty_generics #where_clause {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::StableContainer
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                // Construct BitVector
                let mut active_fields = BitVector::<#max_fields>::new();

                let mut working_field: usize = 0;

                #(
                    if self.#idents.is_some() {
                        active_fields.set(working_field, true).expect("Should not be out of bounds");
                    }
                    working_field += 1;
                )*

                // Hash according to `max_fields` regardless of the actual number of fields on the struct.
                let mut hasher = tree_hash::MerkleHasher::with_leaves(#max_fields);

                #(
                    if self.#idents.is_some() {
                        hasher.write(self.#idents.tree_hash_root().as_slice())
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher.write(tree_hash::Hash256::ZERO.as_slice())
                            .expect("tree hash derive should not apply too many leaves");
                    }
                )*

                let hash = hasher.finish().expect("tree hash derive should not have a remaining buffer");

                tree_hash::mix_in_aux(&hash, &active_fields.tree_hash_root())
            }
        }
    };
    output.into()
}

fn tree_hash_derive_struct_profile(
    item: &DeriveInput,
    struct_data: &DataStruct,
    max_fields: usize,
) -> TokenStream {
    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

    let set_active_fields = &mut vec![];
    let hashes = &mut vec![];

    // Assume a starting index of 0.
    let mut index = 0;

    for (ty, ident, field_opt) in parse_tree_hash_fields(struct_data) {
        let mut is_optional = false;
        if field_opt.skip_hashing {
            continue;
        }

        let ident = match ident {
            Some(ident) => ident,
            _ => {
                panic!(
                    "#[tree_hash(struct_behaviour = \"profile\")] only supports named struct fields."
                )
            }
        };

        if let Some(new_index) = field_opt.stable_index {
            if new_index > index {
                // If we're skipping fields, we need to add zero hash for the skipped fields.
                for _ in index..new_index {
                    hashes.push(quote! {
                        hasher.write(tree_hash::Hash256::ZERO.as_slice())
                            .expect("tree hash derive should not apply too many leaves");
                    });
                }
            }

            index = new_index;
        }

        if ty_inner_type("Optional", ty).is_some() {
            is_optional = true;
        }

        if is_optional {
            set_active_fields.push(quote! {
                if self.#ident.is_some() {
                    active_fields.set(#index, true).expect("Should not be out of bounds");
                }
            });

            hashes.push(quote! {
                if active_fields.get(#index).unwrap_or(false) {
                    hasher.write(self.#ident.tree_hash_root().as_slice())
                        .expect("tree hash derive should not apply too many leaves");
                } else {
                    hasher.write(tree_hash::Hash256::ZERO.as_slice())
                        .expect("tree hash derive should not apply too many leaves");
                }
            });
        } else {
            set_active_fields.push(quote! {
                active_fields.set(#index, true).expect("Should not be out of bounds");
            });
            hashes.push(quote! {
                hasher.write(self.#ident.tree_hash_root().as_slice())
                    .expect("tree hash derive should not apply too many leaves");
            });
        }

        // Increment the index.
        index += 1;
    }

    let output = quote! {
        impl #impl_generics tree_hash::TreeHash for #name #ty_generics #where_clause {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::StableContainer
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Struct should never be packed.")
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                // Construct BitVector
                let mut active_fields = BitVector::<#max_fields>::new();

                #(
                    #set_active_fields
                )*

                // Hash according to `max_fields` regardless of the actual number of fields on the struct.
                let mut hasher = tree_hash::MerkleHasher::with_leaves(#max_fields);

                #(
                    #hashes
                )*

                let hash = hasher.finish().expect("tree hash derive should not have a remaining buffer");

                tree_hash::mix_in_aux(&hash, &active_fields.tree_hash_root())
            }
        }
    };
    output.into()
}

/// Derive `TreeHash` for an enum in the "transparent" method.
///
/// The "transparent" method is distinct from the "union" method specified in the SSZ specification.
/// When using "transparent", the enum will be ignored and the contained field will be hashed as if
/// the enum does not exist.
///
///## Limitations
///
/// Only supports:
/// - Enums with a single field per variant, where
///     - All fields are "container" types.
///
/// ## Panics
///
/// Will panic at compile-time if the single field requirement isn't met, but will panic *at run
/// time* if the container type requirement isn't met.
fn tree_hash_derive_enum_transparent(
    derive_input: &DeriveInput,
    enum_data: &DataEnum,
    inner_container_type: Expr,
) -> TokenStream {
    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let (patterns, type_exprs): (Vec<_>, Vec<_>) = enum_data
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            if variant.fields.len() != 1 {
                panic!("TreeHash can only be derived for enums with 1 field per variant");
            }

            let pattern = quote! {
                #name::#variant_name(inner)
            };

            let ty = &(&variant.fields).into_iter().next().unwrap().ty;
            let type_expr = quote! {
                <#ty as tree_hash::TreeHash>::tree_hash_type()
            };
            (pattern, type_expr)
        })
        .unzip();

    let output = quote! {
        impl #impl_generics tree_hash::TreeHash for #name #ty_generics #where_clause {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                #(
                    assert_eq!(
                        #type_exprs,
                        tree_hash::TreeHashType::#inner_container_type,
                        "all variants must be of container type"
                    );
                )*
                tree_hash::TreeHashType::#inner_container_type
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                unreachable!("Enum should never be packed")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Enum should never be packed")
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                match self {
                    #(
                        #patterns => inner.tree_hash_root(),
                    )*
                }
            }
        }
    };
    output.into()
}

/// Derive `TreeHash` for an `enum` following the "union" SSZ spec.
///
/// The union selector will be determined based upon the order in which the enum variants are
/// defined. E.g., the top-most variant in the enum will have a selector of `0`, the variant
/// beneath it will have a selector of `1` and so on.
///
/// # Limitations
///
/// Only supports enums where each variant has a single field.
fn tree_hash_derive_enum_union(derive_input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let patterns: Vec<_> = enum_data
        .variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let variant_name = &variant.ident;
            let fields_len = variant.fields.len();
            let selector_index: u8 = i
                .try_into()
                .expect("union selector exceeds u8::max_value, union has too many variants");

            // First variant can have no fields (Union[None, ...])
            if fields_len != 1 && !(fields_len == 0 && i == 0) {
                panic!("TreeHash can only be derived for enums with 1 field per variant");
            }

            if fields_len == 0 {
                quote! {
                    #name::#variant_name => {
                        let root = tree_hash::Hash256::ZERO;
                        let selector = #selector_index;
                        tree_hash::mix_in_selector(&root, selector)
                            .expect("derive macro should prevent out-of-bounds selectors")
                    }
                }
            } else {
                quote! {
                    #name::#variant_name(inner) => {
                        let root = inner.tree_hash_root();
                        let selector = #selector_index;
                        tree_hash::mix_in_selector(&root, selector)
                            .expect("derive macro should prevent out-of-bounds selectors")
                    }
                }
            }
        })
        .collect();

    // Panic if there's any issues with the union's variant count
    compute_union_selectors(patterns.len());

    let output = quote! {
        impl #impl_generics tree_hash::TreeHash for #name #ty_generics #where_clause {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::Container
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                unreachable!("Enum should never be packed")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Enum should never be packed")
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                match self {
                    #(#patterns,)*
                }
            }
        }
    };
    output.into()
}

fn compute_union_selectors(num_variants: usize) {
    // Check for 0-variant union
    if num_variants == 0 {
        panic!("0-variant union is not permitted");
    }

    // Get the highest selector (which is num_variants - 1)
    let highest_selector: u8 = (num_variants - 1)
        .try_into()
        .expect("union selector exceeds u8::max_value, union has too many variants");

    assert!(
        highest_selector <= MAX_UNION_SELECTOR,
        "union selector {highest_selector} exceeds limit of {MAX_UNION_SELECTOR}, enum has too many variants"
    );
}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(inner_ty) = &p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}
