// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

use serde_derive::{Deserialize, Serialize};

/// Wrapper struct for serializing u64 values as quoted strings.
///
/// This allows u64 values to be serialized as strings (e.g., "123") while
/// still supporting deserialization from both quoted and unquoted formats.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct QuotedIntWrapper {
    /// The u64 value to be serialized as a quoted string.
    #[serde(with = "crate::serde_utils::quoted_u64::_quoted_u64")]
    pub int: u64,
}
