//! Derive configuration for generated Rust types.
//!
//! Provides defaults, TOML parsing, and derive attribute construction for owned and
//! zero-copy view types.
use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
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
    fn parse_derive_path(name: &str) -> syn::Path {
        syn::parse_str::<syn::Path>(name).expect("invalid derive path")
    }

    fn canonical_path_key(path: &syn::Path) -> String {
        quote!(#path)
            .to_string()
            .replace(' ', "")
            .trim_start_matches("::")
            .to_string()
    }

    fn parse_derive_paths(derives: Vec<String>) -> Vec<syn::Path> {
        derives
            .into_iter()
            .map(|n| Self::parse_derive_path(&n))
            .collect()
    }

    fn dedup_derive_paths(paths: Vec<syn::Path>) -> Vec<syn::Path> {
        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped: Vec<syn::Path> = Vec::new();

        for path in paths {
            let key = Self::canonical_path_key(&path);
            if seen.insert(key) {
                deduped.push(path);
            }
        }

        deduped
    }

    fn is_view_filtered_derive(path: &syn::Path) -> bool {
        matches!(
            Self::canonical_path_key(path).as_str(),
            "ssz_derive::Encode" | "ssz_derive::Decode" | "tree_hash_derive::TreeHash"
        )
    }

    fn is_container_ordering_derive(path: &syn::Path) -> bool {
        matches!(
            Self::canonical_path_key(path).as_str(),
            "std::cmp::Ord" | "core::cmp::Ord" | "std::cmp::PartialOrd" | "core::cmp::PartialOrd"
        )
    }

    fn derive_attr_from_paths(paths: Vec<syn::Path>) -> TokenStream {
        if paths.is_empty() {
            quote! {}
        } else {
            quote! { #[derive( #(#paths),* )] }
        }
    }

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
                "std::clone::Clone".into(),
                "std::fmt::Debug".into(),
                "std::cmp::PartialEq".into(),
                "std::cmp::Eq".into(),
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
        combined.push("std::clone::Clone".to_string());
        combined.push("ssz_derive::Encode".to_string());
        combined.push("ssz_derive::Decode".to_string());
        let deduped_paths = Self::dedup_derive_paths(Self::parse_derive_paths(combined));
        Self::derive_attr_from_paths(deduped_paths)
    }

    /// Build a #[derive(...)] attribute token stream for a view type.
    /// Same configured derives as owned, but never includes SSZ derives.
    pub fn view_derive_attr(&self, type_name: &str) -> TokenStream {
        // Start from configured derives, but strip SSZ derives for view types; ensure Copy+Clone
        let mut paths = Self::parse_derive_paths(self.derives_for_type(type_name));
        paths.retain(|p| !Self::is_view_filtered_derive(p));
        paths.push(Self::parse_derive_path("std::marker::Copy"));
        paths.push(Self::parse_derive_path("std::clone::Clone"));

        let deduped_paths = Self::dedup_derive_paths(paths);
        Self::derive_attr_from_paths(deduped_paths)
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
        let mut combined = self.derives_for_type(type_name);
        combined.extend(pragmas.derives.iter().cloned());
        let mut paths = Self::parse_derive_paths(combined);

        // Filter out ordering derives for Container types
        if is_container {
            paths.retain(|p| !Self::is_container_ordering_derive(p));
        }

        paths.push(Self::parse_derive_path("std::clone::Clone"));
        paths.push(Self::parse_derive_path("ssz_derive::Encode"));
        paths.push(Self::parse_derive_path("ssz_derive::Decode"));
        // Note: TreeHash is NOT included here - we generate a generic TreeHash<H> implementation
        // manually instead of using the derive macro which only generates TreeHash<Sha256Hasher>
        let deduped_paths = Self::dedup_derive_paths(paths);
        Self::derive_attr_from_paths(deduped_paths)
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
        let combined: Vec<String> = self
            .derives_for_type(type_name)
            .into_iter()
            .chain(pragmas.derives.iter().cloned())
            .collect();
        let mut paths = Self::parse_derive_paths(combined);
        paths.retain(|p| !Self::is_view_filtered_derive(p));

        // Filter out PartialOrd and Ord for Container types
        if is_container {
            paths.retain(|p| !Self::is_container_ordering_derive(p));
        }

        paths.push(Self::parse_derive_path("std::marker::Copy"));
        paths.push(Self::parse_derive_path("std::clone::Clone"));

        let deduped_paths = Self::dedup_derive_paths(paths);
        Self::derive_attr_from_paths(deduped_paths)
    }
}
