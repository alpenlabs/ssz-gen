//! Derive configuration for generated Rust types.
//!
//! Provides defaults, TOML parsing, and derive attribute construction for owned and
//! zero-copy view types.
use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

use crate::pragma::ParsedPragma;

/// Configuration for which Rust traits to derive on generated types.
///
/// Defaults apply to all types. Per-type entries replace the defaults for that type.
#[derive(Debug, Clone, Deserialize)]
pub struct DeriveConfigToml {
    /// Default derives for all types
    pub default: Option<Vec<String>>, // optional in file
    /// Per-type overrides (replace semantics)
    #[serde(default)]
    pub types: HashMap<String, Vec<String>>, // type name -> list
}

/// In-memory derive configuration
#[derive(Debug, Clone, Default)]
pub struct DeriveConfig {
    /// Default derives applied to all generated types when no per-type override is provided.
    pub default: Vec<String>,
    /// Per-type override derives (replace semantics), keyed by the Rust type name.
    pub types: HashMap<String, Vec<String>>, // replace semantics
}

impl DeriveConfig {
    /// Parse a [`DeriveConfig`] from a TOML string.
    pub fn from_toml_str(s: &str) -> Result<Self, toml::de::Error> {
        #[derive(Deserialize)]
        struct Root {
            derives: Option<DeriveConfigToml>,
        }

        let parsed: Root = toml::from_str(s)?;
        let mut cfg = DeriveConfig::default_defaults();
        if let Some(derives) = parsed.derives {
            if let Some(default) = derives.default {
                cfg.default = default;
            }
            cfg.types.extend(derives.types);
        }
        Ok(cfg)
    }

    /// Built-in ergonomic defaults for derives.
    pub fn default_defaults() -> Self {
        // Reasonable ergonomic defaults; users can override
        // Note: PartialOrd and Ord are excluded because they don't work for
        // Container types or VariableList types. Users can add them explicitly
        // via TOML or pragmas for types that support them.
        Self {
            default: vec![
                "Clone".into(),
                "Debug".into(),
                "PartialEq".into(),
                "Eq".into(),
            ],
            types: HashMap::new(),
        }
    }

    /// Returns the derive identifiers for a given Rust type name.
    /// Per-type entry replaces defaults.
    pub fn derives_for_type(&self, type_name: &str) -> Vec<String> {
        if let Some(list) = self.types.get(type_name) {
            return list.clone();
        }
        self.default.clone()
    }

    /// Build a #[derive(...)] attribute token stream for an owned type, ensuring
    /// required SSZ derives are present and deduplicated in-order.
    pub fn owned_derive_attr(&self, type_name: &str) -> TokenStream {
        // Combine configured derives + required SSZ derives + ensure Clone for owned types
        // Note: TreeHash is NOT included here - we generate a generic TreeHash<H> implementation
        // manually instead of using the derive macro which only generates TreeHash<Sha256Hasher>
        let mut combined: Vec<String> = self.derives_for_type(type_name);
        combined.push("Clone".to_string());
        combined.push("Encode".to_string());
        combined.push("Decode".to_string());

        // Deduplicate while preserving order
        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped: Vec<String> = Vec::new();
        for s in combined.into_iter() {
            if seen.insert(s.clone()) {
                deduped.push(s);
            }
        }

        let paths: Vec<Ident> = deduped
            .into_iter()
            .map(|n| Ident::new(&n, Span::call_site()))
            .collect();
        quote! { #[derive( #(#paths),* )] }
    }

    /// Build a #[derive(...)] attribute token stream for a view type.
    /// Same configured derives as owned, but never includes SSZ derives.
    pub fn view_derive_attr(&self, type_name: &str) -> TokenStream {
        // Start from configured derives, but strip SSZ derives for view types; ensure Copy+Clone
        let mut combined: Vec<String> = self
            .derives_for_type(type_name)
            .into_iter()
            .filter(|n| n != "Encode" && n != "Decode" && n != "TreeHash")
            .collect();
        combined.push("Copy".to_string());
        combined.push("Clone".to_string());

        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped: Vec<String> = Vec::new();
        for s in combined.into_iter() {
            if seen.insert(s.clone()) {
                deduped.push(s);
            }
        }

        let idents: Vec<Ident> = deduped
            .into_iter()
            .map(|n| Ident::new(&n, Span::call_site()))
            .collect();
        if idents.is_empty() {
            quote! {}
        } else {
            quote! { #[derive( #(#idents),* )] }
        }
    }

    /// Build a #[derive(...)] attribute token stream for an owned type, incorporating pragmas
    pub fn owned_derive_attr_with_pragmas(
        &self,
        type_name: &str,
        pragmas: &ParsedPragma,
    ) -> TokenStream {
        self.owned_derive_attr_with_pragmas_filtered(type_name, pragmas, false)
    }

    /// Build a `#[derive(...)]` attribute token stream for an owned type, incorporating pragmas.
    /// If `is_container` is `true`, filters out `PartialOrd` and `Ord` (which don't work for
    /// `Container` types).
    pub fn owned_derive_attr_with_pragmas_filtered(
        &self,
        type_name: &str,
        pragmas: &ParsedPragma,
        is_container: bool,
    ) -> TokenStream {
        // Combine configured derives + pragma derives + required SSZ derives
        let mut combined: Vec<String> = self.derives_for_type(type_name);
        combined.extend(pragmas.derives.iter().cloned());

        // Filter out PartialOrd and Ord for Container types
        if is_container {
            combined.retain(|d| d != "PartialOrd" && d != "Ord");
        }

        combined.push("Clone".to_string());
        combined.push("Encode".to_string());
        combined.push("Decode".to_string());
        // Note: TreeHash is NOT included here - we generate a generic TreeHash<H> implementation
        // manually instead of using the derive macro which only generates TreeHash<Sha256Hasher>

        // Deduplicate while preserving order
        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped: Vec<String> = Vec::new();
        for s in combined.into_iter() {
            if seen.insert(s.clone()) {
                deduped.push(s);
            }
        }

        let paths: Vec<Ident> = deduped
            .into_iter()
            .map(|n| Ident::new(&n, Span::call_site()))
            .collect();
        quote! { #[derive( #(#paths),* )] }
    }

    /// Build a #[derive(...)] attribute token stream for a view type, incorporating pragmas
    pub fn view_derive_attr_with_pragmas(
        &self,
        type_name: &str,
        pragmas: &ParsedPragma,
    ) -> TokenStream {
        self.view_derive_attr_with_pragmas_filtered(type_name, pragmas, false)
    }

    /// Build a `#[derive(...)]` attribute token stream for a view type, incorporating pragmas.
    /// If `is_container` is `true`, filters out `PartialOrd` and `Ord` (which don't work for
    /// `Container` types).
    pub fn view_derive_attr_with_pragmas_filtered(
        &self,
        type_name: &str,
        pragmas: &ParsedPragma,
        is_container: bool,
    ) -> TokenStream {
        // Start from configured derives + pragma derives, but strip SSZ derives for view types
        let mut combined: Vec<String> = self
            .derives_for_type(type_name)
            .into_iter()
            .chain(pragmas.derives.iter().cloned())
            .filter(|n| n != "Encode" && n != "Decode" && n != "TreeHash")
            .collect();

        // Filter out PartialOrd and Ord for Container types
        if is_container {
            combined.retain(|d| d != "PartialOrd" && d != "Ord");
        }

        combined.push("Copy".to_string());
        combined.push("Clone".to_string());

        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped: Vec<String> = Vec::new();
        for s in combined.into_iter() {
            if seen.insert(s.clone()) {
                deduped.push(s);
            }
        }

        let idents: Vec<Ident> = deduped
            .into_iter()
            .map(|n| Ident::new(&n, Span::call_site()))
            .collect();
        if idents.is_empty() {
            quote! {}
        } else {
            quote! { #[derive( #(#idents),* )] }
        }
    }
}
