//! Code generation module for converting SSZ schemas into Rust code.

use crate::types::resolver::TypeResolver;
use proc_macro2::TokenStream;
use quote::quote;
use sizzle_parser::{AliasDef, ClassDef, SszSchema};
use syn::parse_quote;

/// Represents either an alias or class definition from the SSZ schema.
#[derive(Debug)]
enum AliasOrClass<'a> {
    /// Reference to an alias definition
    Alias(&'a AliasDef),
    /// Reference to a class definition
    Class(&'a ClassDef),
}

/// Helper struct for processing interdependent type definitions.
///
/// This implements a circular buffer algorithm to handle potential dependencies between
/// type definitions, allowing them to be processed in the correct order.
struct CircleBufferCodegen<'a> {
    /// Collection of items to be processed
    items: Vec<AliasOrClass<'a>>,
    /// Generated token streams for each processed item
    tokens: Vec<TokenStream>,
}

impl<'a> CircleBufferCodegen<'a> {
    /// Creates a new CircleBufferCodegen with the given aliases and classes.
    ///
    /// # Arguments
    ///
    /// * `aliases` - The alias definitions to process
    /// * `classes` - The class definitions to process
    ///
    /// # Returns
    ///
    /// A new CircleBufferCodegen instance
    fn new(aliases: &'a [AliasDef], classes: &'a [ClassDef]) -> Self {
        let items: Vec<AliasOrClass<'a>> = aliases
            .iter()
            .map(AliasOrClass::Alias)
            .chain(classes.iter().map(AliasOrClass::Class))
            .collect();
        Self {
            items,
            tokens: Vec::new(),
        }
    }

    /// Processes an alias definition and generates the corresponding Rust type.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias definition to process
    /// * `type_resolver` - The type resolver to use for resolving types
    ///
    /// # Returns
    ///
    /// `true` if processing was successful, `false` if dependencies are not yet resolved.
    fn process_alias(&mut self, alias: &AliasDef, type_resolver: &mut TypeResolver) -> bool {
        let ident = syn::Ident::new(&alias.name().0, proc_macro2::Span::call_site());

        let type_def = type_resolver.resolve_type_and_add(alias.ty(), &ident);
        if type_def.is_none() {
            return false;
        }

        if type_def.is_type() && !type_resolver.union_tracker.borrow().contains_key(&ident.to_string()) {
            let type_def = type_def.unwrap_type();
            self.tokens.push(quote! {
                pub type #ident = #type_def;
            });
        }

        true
    }

    /// Processes a class definition and generates the corresponding Rust struct.
    ///
    /// # Arguments
    ///
    /// * `class` - The class definition to process
    /// * `type_resolver` - The type resolver to use for resolving types
    ///
    /// # Returns
    ///
    /// `true` if processing was successful, `false` if dependencies are not yet resolved.
    fn process_class(&mut self, class: &ClassDef, type_resolver: &mut TypeResolver) -> bool {
        let ident = syn::Ident::new(&class.name().0, proc_macro2::Span::call_site());
        let parent_ty = class.parent_ty();
        let parent_class = type_resolver.resolve_class(parent_ty);
        if parent_class.is_none() {
            return false;
        }

        let mut class_def = parent_class.unwrap();
        for field in class.fields() {
            let field_ident = syn::Ident::new(&field.name().0, proc_macro2::Span::call_site());
            let field_ty = field.ty();
            let field_type = type_resolver.resolve_type(field_ty, None);
            if field_type.is_none() {
                return false;
            }

            if field_type.is_type() {
                let field_type = field_type.unwrap_type();
                class_def.fields.push(field.clone());
                class_def.field_tokens.push(parse_quote! {
                    pub #field_ident: #field_type
                });
            } else {
                panic!("Expected field type to be a type or base class");
            }
        }

        self.tokens.push(class_def.to_token_stream(&ident));
        type_resolver.add_class(&ident, class_def);
        true
    }

    /// Processes all items in the buffer, handling dependencies between them.
    ///
    /// This uses a circular buffer algorithm to ensure that all types are processed in an order
    /// that respects their dependencies. If a circular dependency is detected, this will panic.
    ///
    /// # Arguments
    ///
    /// * `type_resolver` - The type resolver to use for resolving types
    ///
    /// # Returns
    ///
    /// A vector of TokenStreams containing the generated Rust code for each item
    fn process(mut self, type_resolver: &mut TypeResolver) -> Vec<TokenStream> {
        let vec_len = self.items.len();
        if vec_len == 0 {
            return self.tokens;
        }

        let mut start = 0;
        let mut end = vec_len - 1;

        loop {
            let queue_size = if end >= start {
                end - start + 1
            } else {
                vec_len - start + end + 1
            };

            let mut items_processed = 0;
            let mut made_progress = false;

            let cached_end = end;
            while items_processed < queue_size {
                let item = &self.items[start];
                let process_success = match item {
                    AliasOrClass::Alias(alias) => self.process_alias(alias, type_resolver),
                    AliasOrClass::Class(class) => self.process_class(class, type_resolver),
                };

                if process_success {
                    made_progress = true;
                } else {
                    end = (end + 1) % vec_len;
                    self.items.swap(start, end);
                }
                start = (start + 1) % vec_len;

                items_processed += 1;
            }

            if !made_progress {
                panic!("Potential circular dependency");
            }

            if cached_end == end {
                break;
            }
        }

        self.tokens
    }
}

/// Converts an SSZ schema into a Rust code token stream
///
/// # Arguments
///
/// * `schema` - The SSZ schema to convert
///
/// # Returns
///
/// A TokenStream containing the generated Rust code
pub fn schema_to_rust_code(schema: &SszSchema) -> TokenStream {
    // Constants

    // Sizzle parser automatically handles constants in alias and class definitions so no need to keep track of them
    // We just collect them here and add them to the top of the file
    let constants = schema.constants().iter().map(|constant| {
        let ident = syn::Ident::new(&constant.name().0, proc_macro2::Span::call_site());
        let value = constant.value().eval();

        quote! {
            pub const #ident: u64 = #value;
        }
    });

    // Aliases and Classes can reference each other so we need to process them together
    let mut type_resolver = TypeResolver::default();
    let codegen = CircleBufferCodegen::new(schema.aliases(), schema.classes());
    let tokens = codegen.process(&mut type_resolver);

    let union_tracker = type_resolver.union_tracker.borrow();
    let mut unions: Vec<_> = union_tracker.iter().collect();
    // Sort unions by key to ensure deterministic output
    unions.sort_by_key(|(key, _)| *key);
    let unions = unions.into_iter().map(|(_, ty)| {
        quote! {
            #ty
        }
    });

    quote! {
        use ssz_types::*;
        use ssz_derive::{Encode, Decode};
        use tree_hash_derive::TreeHash;
        use typenum::Unsigned;

        #(#unions)*

        #(#constants)*

        #(#tokens)*
    }
}
