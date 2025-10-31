//! Pragma parsing and processing utilities

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, parse_str};

/// Parsed pragma directives
#[derive(Debug, Clone, Default)]
pub struct ParsedPragma {
    /// Additional derive traits to add
    pub derives: Vec<String>,
    /// Additional struct-level attributes
    pub struct_attrs: Vec<TokenStream>,
    /// Field-level attributes (applied to all fields, or via field-specific pragmas)
    pub field_attrs: Vec<TokenStream>,
}

impl ParsedPragma {
    /// Parse a list of pragma strings into structured directives
    pub fn parse(pragmas: &[String]) -> Self {
        let mut derives = Vec::new();
        let mut struct_attrs = Vec::new();
        let mut field_attrs = Vec::new();

        for pragma in pragmas {
            let trimmed = pragma.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Parse derive: Trait1, Trait2, ...
            if let Some(rest) = trimmed.strip_prefix("derive:") {
                let traits: Vec<String> = rest
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                derives.extend(traits);
            }
            // Parse attr: #[attribute] or #[attribute(arg)]
            else if let Some(rest) = trimmed.strip_prefix("attr:") {
                let attr_str = rest.trim();
                if let Ok(attr) = parse_str::<TokenStream>(attr_str) {
                    struct_attrs.push(attr);
                }
            }
            // Parse field_attr: #[attribute] (for struct-level, applies to all fields)
            else if let Some(rest) = trimmed.strip_prefix("field_attr:") {
                let attr_str = rest.trim();
                if let Ok(attr) = parse_str::<TokenStream>(attr_str) {
                    field_attrs.push(attr);
                }
            }
        }

        Self {
            derives,
            struct_attrs,
            field_attrs,
        }
    }

    /// Merge two parsed pragmas (self takes precedence for duplicates)
    pub fn merge(&mut self, other: Self) {
        // For derives, combine and deduplicate
        let mut combined_derives = other.derives.clone();
        combined_derives.extend(self.derives.iter().cloned());
        // Deduplicate while preserving order
        let mut seen = std::collections::HashSet::new();
        self.derives = combined_derives
            .into_iter()
            .filter(|d| seen.insert(d.clone()))
            .collect();

        // For attributes, self takes precedence (append first)
        let mut combined_struct = other.struct_attrs.clone();
        combined_struct.extend(self.struct_attrs.iter().cloned());
        self.struct_attrs = combined_struct;

        let mut combined_field = other.field_attrs.clone();
        combined_field.extend(self.field_attrs.iter().cloned());
        self.field_attrs = combined_field;
    }

    /// Build additional derive attributes from pragmas
    pub fn derive_attr(&self) -> TokenStream {
        if self.derives.is_empty() {
            return quote! {};
        }

        let idents: Vec<Ident> = self
            .derives
            .iter()
            .map(|n| Ident::new(n, Span::call_site()))
            .collect();

        quote! { #[derive( #(#idents),* )] }
    }
}
