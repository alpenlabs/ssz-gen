//! Code generation module for converting SSZ schemas into Rust code.

use crate::types::{
    BaseClass, ClassDef, ClassDefinition, ClassFieldDef, TypeResolution, resolver::TypeResolver,
};
use proc_macro2::TokenStream;
use quote::quote;
use sizzle_parser::{AliasDef as ParserAliasDef, ClassDef as ParserClassDef, SszSchema};
use std::collections::HashMap;
use syn::parse_quote;

/// Represents either an alias or class definition from the SSZ schema.
#[derive(Debug)]
enum AliasOrClass<'a> {
    /// Reference to an alias definition
    Alias(&'a ParserAliasDef),
    /// Reference to a class definition
    Class(&'a ParserClassDef),
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
    fn new(aliases: &'a [ParserAliasDef], classes: &'a [ParserClassDef]) -> Self {
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
    fn process_alias(&mut self, alias: &ParserAliasDef, type_resolver: &mut TypeResolver) -> bool {
        let ident = syn::Ident::new(&alias.name().0, proc_macro2::Span::call_site());

        let type_def = type_resolver.resolve_type_and_add(alias.ty(), &ident);
        if type_def.is_unresolved() {
            return false;
        }

        if type_def.is_type()
            && !type_resolver
                .union_tracker
                .borrow()
                .contains_key(&ident.to_string())
        {
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
    fn process_class(&mut self, class: &ParserClassDef, type_resolver: &mut TypeResolver) -> bool {
        let ident = syn::Ident::new(&class.name().0, proc_macro2::Span::call_site());
        let parent_ty = class.parent_ty();
        let parent_class = type_resolver.resolve_class(parent_ty);
        if parent_class.is_none() {
            return false;
        }

        let mut parent_class_def = parent_class.unwrap();
        let success = match parent_class_def.base {
            BaseClass::Container | BaseClass::StableContainer(_) => {
                self.process_simple_inheritance(&mut parent_class_def, class, type_resolver)
            }
            BaseClass::Profile(_) => {
                self.process_profile_inheritance(&mut parent_class_def, class, type_resolver)
            }
        };

        if success {
            self.tokens.push(parent_class_def.to_token_stream(&ident));
            type_resolver.add_class(&ident, parent_class_def);
            return true;
        }

        false
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

    fn process_simple_inheritance(
        &mut self,
        parent_class_def: &mut ClassDef,
        class: &ParserClassDef,
        type_resolver: &mut TypeResolver,
    ) -> bool {
        // Get capacity of parent class
        let capacity = match parent_class_def.base {
            BaseClass::StableContainer(Some(cap)) => cap,
            BaseClass::StableContainer(None) => {
                panic!("Expected parent class used for inheritance to have all arguments")
            }
            BaseClass::Container => u64::MAX,
            _ => panic!("Simple inheritance is only allowed for Container and StableContainer"),
        };

        let mut curr_index = 0;
        for field in class.fields() {
            // If name overlap -> replace field type
            if let Some(parent_field_index) = parent_class_def.field_index.get(&field.name().0) {
                if *parent_field_index < curr_index {
                    panic!("Inheritance field order violation");
                }
                curr_index = *parent_field_index;
            } else {
                curr_index = parent_class_def.fields.len();
            }

            // Check for capacity overflow
            if curr_index >= capacity as usize {
                panic!("Capacity overflow");
            }

            // Resolve the field type
            let field_ident = syn::Ident::new(&field.name().0, proc_macro2::Span::call_site());
            let field_ty = field.ty();
            let field_type = type_resolver.resolve_type(field_ty, None);
            if field_type.is_unresolved() {
                return false;
            }

            // Make sure the field is compatible with the parent class
            match parent_class_def.base {
                BaseClass::Container => {
                    if matches!(field_type, TypeResolution::Optional(_)) {
                        panic!("Optional fields are not allowed in Container classes");
                    }
                }
                BaseClass::StableContainer(_) => {
                    if !matches!(field_type, TypeResolution::Optional(_)) {
                        panic!("All fields in StableContainer classes must be optional");
                    }
                }
                _ => panic!("Simple inheritance is only allowed for Container and StableContainer"),
            }

            if field_type.is_type() {
                let field_ty = field_type.unwrap_type();
                let new_field = ClassFieldDef {
                    index: curr_index,
                    name: field.name().0.to_string(),
                    ty: field_type,
                };

                if parent_class_def.fields.len() > curr_index {
                    // Replacing existing field
                    parent_class_def.fields[curr_index] = new_field;
                    parent_class_def.field_tokens[curr_index] = parse_quote! {
                        pub #field_ident: #field_ty
                    };
                } else {
                    // Adding new field
                    parent_class_def.fields.push(new_field);
                    parent_class_def.field_tokens.push(parse_quote! {
                        pub #field_ident: #field_ty
                    });
                    parent_class_def
                        .field_index
                        .insert(field.name().0.to_string(), curr_index);
                }
            } else {
                panic!("Expected field type to be a type or base class");
            }
        }
        true
    }

    fn process_profile_inheritance(
        &mut self,
        parent_class_def: &mut ClassDef,
        class: &ParserClassDef,
        type_resolver: &mut TypeResolver,
    ) -> bool {
        // Get the original stable container's definition
        // Needed in case we're inheriting from a profile class into a new profile class
        let stable_contaienr_name = match &parent_class_def.base {
            BaseClass::Profile(Some((name, _))) => name,
            _ => panic!("Expected profile to inherit from a stable container"),
        };

        let stable_container_def = type_resolver.classes.get(stable_contaienr_name);
        if stable_container_def.is_none() {
            panic!("Expected stable container parent of profile class to be defined");
        }
        let stable_container_def = match stable_container_def.unwrap() {
            ClassDefinition::Custom(class_def) => class_def,
            _ => panic!("Expected stable container parent of profile class to be defined"),
        };

        // Profile classes are not allowed to add extra fields to their parent class
        // -> No need for capacity overflow check

        // Profile classes contain a subset of the original StableContainer's fields
        // the field types can only be changed if the new field type is considered "equivalent"
        // to the original field type
        let mut new_fields = vec![];
        let mut new_field_index = HashMap::new();
        let mut new_field_tokens = vec![];

        let mut curr_index = 0;
        for field in class.fields() {
            // Check if field exists in parent class
            let original_field_index: usize;
            if let Some(parent_field_index) = parent_class_def.field_index.get(&field.name().0) {
                original_field_index = parent_class_def.fields[*parent_field_index].index;
            } else if let Some(stable_field_index) =
                stable_container_def.field_index.get(&field.name().0)
            {
                // Check if field exists in original stable container
                original_field_index = *stable_field_index;
            } else {
                panic!("Profile classes cannot add new fields to their parent classes");
            }

            // Make sure ordering is maintained
            if original_field_index < curr_index {
                panic!("Inheritance field order violation");
            }
            curr_index = original_field_index;

            let field_ident = syn::Ident::new(&field.name().0, proc_macro2::Span::call_site());
            let field_ty = field.ty();
            let field_type = type_resolver.resolve_type(field_ty, None);
            if field_type.is_unresolved() {
                return false;
            }

            // Make sure the field is compatible
            if !stable_container_def.fields[original_field_index]
                .ty
                .check_field_compatibility_for_profile(&field_type, type_resolver)
            {
                panic!("Field type is not compatible with parent class");
            }

            if field_type.is_type() {
                let field_ty = field_type.unwrap_type();
                let new_field = ClassFieldDef {
                    index: curr_index,
                    name: field.name().0.to_string(),
                    ty: field_type,
                };

                new_field_index.insert(field.name().0.to_string(), new_fields.len());
                new_fields.push(new_field.clone());
                new_field_tokens.push(parse_quote! {
                    pub #field_ident: #field_ty
                });
            } else {
                panic!("Expected field type to be a type or base class");
            }
        }

        parent_class_def.fields = new_fields;
        parent_class_def.field_index = new_field_index;
        parent_class_def.field_tokens = new_field_tokens;

        true
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
