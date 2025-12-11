//! Code generation module for converting SSZ schemas into Rust code.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use sizzle_parser::{AliasDef as ParserAliasDef, ClassDef as ParserClassDef, SszSchema, tysys::Ty};
use syn::{Ident, parse_quote};

use crate::{
    ModuleGeneration,
    derive_config::DeriveConfig,
    types::{
        BaseClass, ClassDef, ClassDefinition, ClassFieldDef, TypeResolutionKind,
        resolver::TypeResolver,
    },
};

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
    /// Derive configuration used during generation
    derive_cfg: &'a DeriveConfig,
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
    fn new(
        aliases: &'a [ParserAliasDef],
        classes: &'a [ParserClassDef],
        derive_cfg: &'a DeriveConfig,
    ) -> Self {
        let items: Vec<AliasOrClass<'a>> = aliases
            .iter()
            .map(AliasOrClass::Alias)
            .chain(classes.iter().map(AliasOrClass::Class))
            .collect();
        Self {
            items,
            tokens: Vec::new(),
            derive_cfg,
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
    fn process_alias(
        &mut self,
        alias: &ParserAliasDef,
        type_resolver: &mut TypeResolver<'_>,
    ) -> bool {
        let ident = Ident::new(&alias.name().0, Span::call_site());

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
            if type_def.is_constant() {
                let ty = type_def.unwrap_type();
                self.tokens.push(quote! {
                    #[allow(dead_code, reason = "generated code using ssz-gen")]
                    pub const #ident: u64 = #ty;
                });
            } else {
                let ty = if type_def.contains_const_ref() {
                    type_def.unwrap_type_preserving_const_names()
                } else {
                    type_def.unwrap_type()
                };
                self.tokens.push(quote! {
                    pub type #ident = #ty;
                });
            }
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
    fn process_class(
        &mut self,
        class: &ParserClassDef,
        type_resolver: &mut TypeResolver<'_>,
    ) -> bool {
        let ident = Ident::new(&class.name().0, Span::call_site());

        let parent_ty = class.parent_ty();
        let parent_path = match parent_ty {
            Ty::Imported(path, _, _) => Some(path),
            _ => None,
        };
        let parent_class = type_resolver.resolve_class(parent_ty);
        if parent_class.is_none() {
            return false;
        }

        let mut parent_class_def = parent_class.unwrap();

        // Copy pragmas and doc comments from the class definition
        parent_class_def.pragmas = class.pragmas().to_vec();
        parent_class_def.doc_comment = class.doc_comment().map(|s| s.to_string());
        parent_class_def.doc = class.doc().map(|s| s.to_string());

        let success = match parent_class_def.base {
            BaseClass::Container | BaseClass::StableContainer(_) => {
                self.process_simple_inheritance(&mut parent_class_def, class, type_resolver)
            }
            BaseClass::Profile(_) => self.process_profile_inheritance(
                &mut parent_class_def,
                class,
                type_resolver,
                parent_path,
            ),
            BaseClass::Union => {
                // Union classes generate code directly in process_union_class and store in
                // union_tracker Skip the normal class generation methods
                if self.process_union_class(&ident, class, type_resolver) {
                    // Union code is already generated and stored in union_tracker
                    return true;
                }
                false
            }
        };

        if success {
            // Generate owned struct
            self.tokens
                .push(parent_class_def.to_token_stream(&ident, self.derive_cfg));

            // Generate generic TreeHash implementation for owned struct
            self.tokens
                .push(parent_class_def.to_owned_tree_hash_impl(&ident));

            // Generate view struct (thin wrapper)
            self.tokens
                .push(parent_class_def.to_view_struct(&ident, self.derive_cfg));

            // Generate getter methods for view struct
            self.tokens.push(parent_class_def.to_view_getters(&ident));

            // Generate TreeHash implementation for view struct
            self.tokens
                .push(parent_class_def.to_view_tree_hash_impl(&ident));

            // Generate DecodeView implementation (validation-only)
            self.tokens
                .push(parent_class_def.to_view_decode_impl(&ident));

            // Generate SszTypeInfo implementation (required for lists/vectors)
            self.tokens
                .push(parent_class_def.to_view_ssz_type_info_impl(&ident));

            // Generate ToOwnedSsz implementation (required for lists/vectors)
            self.tokens
                .push(parent_class_def.to_view_to_owned_ssz_impl(&ident));

            // Generate to_owned implementation (uses getters)
            self.tokens
                .push(parent_class_def.to_view_to_owned_impl(&ident));

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
    fn process(mut self, type_resolver: &mut TypeResolver<'_>) -> Vec<TokenStream> {
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
        type_resolver: &mut TypeResolver<'_>,
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
            let field_ident = Ident::new(&field.name().0, Span::call_site());

            let field_ty = field.ty();
            let field_type = type_resolver.resolve_type(field_ty, None);
            if field_type.is_unresolved() {
                return false;
            }

            let ty = field_type.unwrap_type();
            let field_ty_token = quote! { #ty };

            // Make sure the field is compatible with the parent class
            match parent_class_def.base {
                BaseClass::Container => {
                    if matches!(field_type.resolution, TypeResolutionKind::Optional(_)) {
                        panic!("Optional fields are not allowed in Container classes");
                    }
                }
                BaseClass::StableContainer(_) => {
                    if !matches!(field_type.resolution, TypeResolutionKind::Optional(_))
                        && !matches!(field_type.resolution, TypeResolutionKind::External)
                    {
                        panic!("All fields in StableContainer classes must be optional");
                    }
                }
                _ => panic!("Simple inheritance is only allowed for Container and StableContainer"),
            }

            if field_type.is_type() {
                let new_field = ClassFieldDef {
                    index: curr_index,
                    name: field.name().0.to_string(),
                    ty: field_type,
                    pragmas: field.pragmas().to_vec(),
                    doc_comment: field.doc_comment().map(|s| s.to_string()),
                };

                // Build field token with pragma attributes and doc comment
                let field_pragmas = crate::pragma::ParsedPragma::parse(field.pragmas());
                let field_doc = if let Some(doc) = &new_field.doc_comment {
                    ClassDef::format_doc_comment(doc)
                } else {
                    quote! {}
                };
                let has_field_doc = new_field.doc_comment.is_some();
                let field_attr_tokens = if !field_pragmas.field_attrs.is_empty() {
                    let attrs = &field_pragmas.field_attrs;
                    quote! {
                        #field_doc
                        #(#attrs)*
                        pub #field_ident: #field_ty_token
                    }
                } else if has_field_doc {
                    quote! {
                        #field_doc
                        pub #field_ident: #field_ty_token
                    }
                } else {
                    parse_quote! {
                        pub #field_ident: #field_ty_token
                    }
                };

                if parent_class_def.fields.len() > curr_index {
                    // Replacing existing field
                    parent_class_def.fields[curr_index] = new_field;
                    parent_class_def.field_tokens[curr_index] = field_attr_tokens;
                } else {
                    // Adding new field
                    parent_class_def.fields.push(new_field);
                    parent_class_def.field_tokens.push(field_attr_tokens);
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
        type_resolver: &mut TypeResolver<'_>,
        parent_path: Option<&PathBuf>,
    ) -> bool {
        // Get the original stable container's definition
        // Needed in case we're inheriting from a profile class into a new profile class
        let stable_contaienr_name = match &parent_class_def.base {
            BaseClass::Profile(Some((name, _))) => name,
            _ => panic!("Expected profile to inherit from a stable container"),
        };

        // If it's imported, we need to get the original stable container's definition from another
        // module
        let resolvers = type_resolver.resolvers.borrow();
        let stable_container_def = if let Some(parent_path) = parent_path {
            let resolver = resolvers.get(parent_path).unwrap();
            resolver.classes.get(stable_contaienr_name)
        } else {
            type_resolver.classes.get(stable_contaienr_name)
        };

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

            let field_ident = Ident::new(&field.name().0, Span::call_site());
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
                    pragmas: field.pragmas().to_vec(),
                    doc_comment: field.doc_comment().map(|s| s.to_string()),
                };

                // Build field token with pragma attributes and doc comment
                let field_pragmas = crate::pragma::ParsedPragma::parse(field.pragmas());
                let field_doc = if let Some(doc) = &new_field.doc_comment {
                    ClassDef::format_doc_comment(doc)
                } else {
                    quote! {}
                };
                let has_field_doc = new_field.doc_comment.is_some();
                let field_attr_tokens = if !field_pragmas.field_attrs.is_empty() {
                    let attrs = &field_pragmas.field_attrs;
                    quote! {
                        #field_doc
                        #(#attrs)*
                        pub #field_ident: #field_ty
                    }
                } else if has_field_doc {
                    quote! {
                        #field_doc
                        pub #field_ident: #field_ty
                    }
                } else {
                    parse_quote! {
                        pub #field_ident: #field_ty
                    }
                };

                new_field_index.insert(field.name().0.to_string(), new_fields.len());
                new_fields.push(new_field.clone());
                new_field_tokens.push(field_attr_tokens);
            } else {
                panic!("Expected field type to be a type or base class");
            }
        }

        parent_class_def.fields = new_fields;
        parent_class_def.field_index = new_field_index;
        parent_class_def.field_tokens = new_field_tokens;

        true
    }

    fn process_union_class(
        &mut self,
        union_ident: &Ident,
        class: &ParserClassDef,
        type_resolver: &mut TypeResolver<'_>,
    ) -> bool {
        let union_name = union_ident.to_string();
        let mut args = Vec::new();
        let mut variant_names = Vec::new();
        let mut variant_pragmas = Vec::new();
        let mut variant_doc_comments: Vec<Option<String>> = Vec::new();

        for field in class.fields() {
            let field_ty = field.ty();
            let field_type = type_resolver.resolve_type(field_ty, None);
            if field_type.is_unresolved() {
                return false;
            }

            args.push(field_type);
            variant_names.push(field.name().0.clone());
            variant_pragmas.push(field.pragmas().to_vec());
            variant_doc_comments.push(field.doc_comment().map(|s| s.to_string()));
        }

        // Generate doc comments for the union type itself
        let union_doc_comments = {
            let doc = class.doc().map(|s| s.to_string());
            let doc_comment = class.doc_comment().map(|s| s.to_string());
            match (&doc, &doc_comment) {
                (Some(docstring), Some(comment)) => {
                    let merged = format!("{}\n\n{}", docstring.trim(), comment.trim());
                    ClassDef::format_doc_comment(&merged)
                }
                (Some(docstring), None) => ClassDef::format_doc_comment(docstring),
                (None, Some(comment)) => ClassDef::format_doc_comment(comment),
                (None, None) => quote! {},
            }
        };

        let variants: Vec<TokenStream> = args
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                let field_name = &class.fields()[i].name().0;
                let variant_name = field_name.clone();
                let ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

                // Generate doc comment for this variant
                let variant_doc = variant_doc_comments
                    .get(i)
                    .and_then(|opt| opt.as_ref())
                    .map(|doc| ClassDef::format_doc_comment(doc))
                    .unwrap_or_else(|| quote! {});

                match ty.resolution {
                    crate::types::TypeResolutionKind::None => {
                        if i == 0 {
                            quote! {
                                #variant_doc
                                #ident
                            }
                        } else {
                            panic!("None is only allowed as the first variant in a Union")
                        }
                    }
                    _ => {
                        let variant_ty = ty.unwrap_type();
                        quote! {
                            #variant_doc
                            #ident(#variant_ty)
                        }
                    }
                }
            })
            .collect();

        let owned_tree_hash_arms: Vec<TokenStream> = args
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                let selector_value = i as u8;
                let field_name = &class.fields()[i].name().0;
                let variant_ident = Ident::new(field_name, Span::call_site());

                match ty.resolution {
                    crate::types::TypeResolutionKind::None => {
                        quote! {
                            #union_ident::#variant_ident => {
                                tree_hash::mix_in_selector_with_hasher::<H>(
                                    &tree_hash::Hash256::ZERO,
                                    #selector_value
                                ).expect("valid selector")
                            }
                        }
                    }
                    _ => {
                        quote! {
                            #union_ident::#variant_ident(inner) => {
                                let root = <_ as tree_hash::TreeHash<H>>::tree_hash_root(inner);
                                tree_hash::mix_in_selector_with_hasher::<H>(
                                    &root,
                                    #selector_value
                                ).expect("valid selector")
                            }
                        }
                    }
                }
            })
            .collect();

        let union_code = quote! {
            #union_doc_comments
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour="union")]
            pub enum #union_ident {
                #(#variants),*
            }

            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for #union_ident {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }

                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }

                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }

                fn tree_hash_root(&self) -> H::Output {
                    match self {
                        #(#owned_tree_hash_arms),*
                    }
                }
            }
        };

        type_resolver
            .union_tracker
            .borrow_mut()
            .insert(union_name.clone(), union_code);

        let view_union_code = type_resolver.generate_union_view_code(
            &union_name,
            union_ident,
            &args,
            &variant_names,
            &variant_pragmas,
        );

        type_resolver
            .union_tracker
            .borrow_mut()
            .insert(format!("{}Ref", union_name), view_union_code);

        let class_def = ClassDef {
            base: BaseClass::Union,
            fields: vec![],
            field_tokens: vec![],
            field_index: HashMap::new(),
            pragmas: class.pragmas().to_vec(),
            doc_comment: class.doc_comment().map(|s| s.to_string()),
            doc: class.doc().map(|s| s.to_string()),
        };
        type_resolver.add_class(union_ident, class_def);

        true
    }
}

/// Represents a node in the module hierarchy
#[derive(Debug)]
struct ModuleNode {
    /// Full path to this module
    path: String,
    /// Child modules
    children: Vec<ModuleNode>,
}

/// Recursively generates the module structure for a node and its children
fn generate_module_code(
    node: &ModuleNode,
    module_tokens: &HashMap<&PathBuf, TokenStream>,
) -> TokenStream {
    let path = PathBuf::from(&node.path);
    let module_name = path.file_name().unwrap().to_string_lossy();
    let module_ident = Ident::new(&module_name, Span::call_site());

    // Get the code for this module if it exists
    let module_code = module_tokens.get(&path).cloned();

    // Generate code for all children
    let child_modules: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_module_code(child, module_tokens))
        .collect();

    // Combine the module's own code with its children's modules
    quote! {
        pub mod #module_ident {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            #module_code

            #(#child_modules)*
        }
    }
}

/// Checks if any type in the schema uses serde derives
fn schema_uses_serde(schema: &SszSchema) -> bool {
    use crate::pragma::ParsedPragma;

    // Check classes for serde derives in pragmas
    for class in schema.classes() {
        let pragmas = ParsedPragma::parse(class.pragmas());
        if pragmas
            .derives
            .iter()
            .any(|d| d == "Serialize" || d == "Deserialize")
        {
            return true;
        }
    }

    false
}

/// Generates a single flat module with all definitions at the root level
fn single_module_rust_code(
    schema_map: &HashMap<&PathBuf, TokenStream>,
    entry_point_paths: &HashSet<PathBuf>,
    needs_serde: bool,
) -> TokenStream {
    let mut all_tokens = Vec::new();

    // Sort paths to ensure consistent ordering
    let mut paths: Vec<_> = schema_map.keys().collect();
    paths.sort();

    for path in paths {
        // Only include tokens from actual entry points to avoid duplicates
        if entry_point_paths.contains(*path)
            && let Some(tokens) = schema_map.get(path)
        {
            all_tokens.push(tokens.clone());
        }
    }

    let serde_imports = if needs_serde {
        quote! { use serde::{Serialize, Deserialize}; }
    } else {
        quote! {}
    };

    quote! {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        use ssz_types::*;
        use ssz_types::view::{FixedVectorRef, VariableListRef};
        use ssz_primitives::{U128, U256};
        use ssz_derive::{Encode, Decode};
        use tree_hash::TreeHashDigest;
        use tree_hash_derive::TreeHash;
        use ssz::view::*;
        #serde_imports

        #(#all_tokens)*
    }
}

/// Generates flat modules without deep nesting (one level per file)
fn flat_modules_rust_code(
    schema_map: &HashMap<&PathBuf, TokenStream>,
    entry_point_paths: &HashSet<PathBuf>,
    needs_serde: bool,
) -> TokenStream {
    let mut modules = Vec::new();

    // Sort paths to ensure consistent ordering
    let mut paths: Vec<_> = schema_map.keys().collect();
    paths.sort();

    let serde_imports = if needs_serde {
        quote! { use serde::{Serialize, Deserialize}; }
    } else {
        quote! {}
    };

    for path in paths {
        // Only include modules from actual entry points to avoid duplicates
        if entry_point_paths.contains(*path) {
            let module_name = path.file_stem().unwrap_or_default().to_string_lossy();
            let module_ident = Ident::new(&module_name, Span::call_site());

            if let Some(content_tokens) = schema_map.get(path) {
                modules.push(quote! {
                    pub mod #module_ident {
                        #![allow(unused_imports, reason = "generated code using ssz-gen")]
                        use ssz_types::*;
                        use ssz_types::view::{FixedVectorRef, VariableListRef};
                        use ssz_primitives::{U128, U256};
                        use ssz_derive::{Encode, Decode};
                        use tree_hash::TreeHashDigest;
                        use tree_hash_derive::TreeHash;
                        use ssz::view::*;
                        #serde_imports

                        #content_tokens
                    }
                });
            }
        }
    }

    quote! {
        #(#modules)*
    }
}

fn module_tokens_to_rust_code(schema_map: &HashMap<&PathBuf, TokenStream>) -> TokenStream {
    let mut root_nodes = Vec::new();

    // Sort paths to ensure consistent ordering
    let mut paths: Vec<_> = schema_map.keys().collect();
    paths.sort();

    for path in paths {
        let path_str = path.to_string_lossy().to_string();

        // Split path into components
        let components: Vec<&str> = path_str.split('/').collect();

        // Build the hierarchy
        let mut current_path = String::new();
        let mut current_nodes: &mut Vec<ModuleNode> = &mut root_nodes;

        for (i, component) in components.iter().enumerate() {
            if i > 0 {
                current_path.push(std::path::MAIN_SEPARATOR);
            }
            current_path.push_str(component);

            // Find or create the node at this level
            let node_index = current_nodes.iter().position(|n| n.path == current_path);
            let node = if let Some(idx) = node_index {
                &mut current_nodes[idx]
            } else {
                let new_node = ModuleNode {
                    path: current_path.clone(),
                    children: Vec::new(),
                };
                current_nodes.push(new_node);
                current_nodes.last_mut().unwrap()
            };

            // Move to children for next iteration
            current_nodes = &mut node.children;
        }
    }

    // Generate the final code by recursively processing each root node
    let module_code: Vec<TokenStream> = root_nodes
        .iter()
        .map(|node| generate_module_code(node, schema_map))
        .collect();

    quote! {
        #(#module_code)*
    }
}

/// Converts mapping of module path => SSZ schemas into a Rust code token stream
///
/// # Arguments
///
/// * `parsing_order` - The order in which to process the schemas
/// * `schema_map` - The mapping of module path => SSZ schema to convert
/// * `module_generation` - Controls how modules are structured in the generated code
/// * `derive_cfg` - Configuration for derive macros
/// * `entry_point_paths` - Set of paths that are actual entry points (vs imported dependencies)
///
/// # Returns
///
/// A TokenStream containing the generated Rust code
pub fn schema_map_to_rust_code(
    parsing_order: &[PathBuf],
    schema_map: &HashMap<PathBuf, SszSchema>,
    module_generation: ModuleGeneration,
    derive_cfg: &DeriveConfig,
    entry_point_paths: &HashSet<PathBuf>,
) -> TokenStream {
    let mut module_tokens = HashMap::new();
    let mut module_content_tokens = HashMap::new(); // Content without imports for `SingleModule`
    let resolvers = RefCell::new(HashMap::new());

    // Detect if any schema uses serde derives
    let needs_serde = parsing_order
        .iter()
        .filter_map(|path| schema_map.get(path))
        .any(schema_uses_serde);

    for path in parsing_order {
        let schema = schema_map.get(path).unwrap();
        let mut type_resolver = TypeResolver::new_with_builtins(&resolvers);

        // Constants
        let constants = schema
            .constants()
            .iter()
            .map(|constant| {
                let ident = Ident::new(&constant.name().0, Span::call_site());
                let value = constant.value().eval();
                type_resolver.add_constant(&ident, value);

                quote! {
                    #[allow(dead_code, reason = "generated code using ssz-gen")]
                    pub const #ident: u64 = #value;
                }
            })
            .collect::<Vec<_>>();

        // Aliases and Classes can reference each other so we need to process them together
        let codegen = CircleBufferCodegen::new(schema.aliases(), schema.classes(), derive_cfg);
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

        let content_tokens = quote! {
            #(#unions)*

            #(#constants)*

            #(#tokens)*
        };

        // Store content without imports for SingleModule mode
        module_content_tokens.insert(path, content_tokens.clone());

        // Conditional serde import
        let serde_imports = if needs_serde {
            quote! { use serde::{Serialize, Deserialize}; }
        } else {
            quote! {}
        };

        // Store full module with imports for other modes
        module_tokens.insert(
            path,
            quote! {
                use ssz_types::*;
                use ssz_types::view::{FixedVectorRef, VariableListRef};
                use ssz_primitives::{U128, U256};
                use ssz_derive::{Encode, Decode};
                use tree_hash::TreeHashDigest;
                use tree_hash_derive::TreeHash;
                use ssz::view::*;
                #serde_imports

                #content_tokens
            },
        );

        drop(union_tracker);
        resolvers.borrow_mut().insert(path.clone(), type_resolver);
    }

    match module_generation {
        ModuleGeneration::SingleModule => {
            single_module_rust_code(&module_content_tokens, entry_point_paths, needs_serde)
        }
        ModuleGeneration::FlatModules => {
            flat_modules_rust_code(&module_content_tokens, entry_point_paths, needs_serde)
        }
        ModuleGeneration::NestedModules => module_tokens_to_rust_code(&module_tokens),
    }
}
