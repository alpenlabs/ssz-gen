// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Serde utilities for `FixedVector`

use crate::FixedVector;
use serde::{Deserializer, Serializer};
use ssz::serde_utils::hex::{self, PrefixedHexVisitor};

/// Serialize a `FixedVector` as a hex string.
pub fn serialize<S, const U: usize>(
    bytes: &FixedVector<u8, U>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(&bytes[..]))
}

/// Deserialize a `FixedVector` from a hex string.
pub fn deserialize<'de, D, const U: usize>(deserializer: D) -> Result<FixedVector<u8, U>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = deserializer.deserialize_string(PrefixedHexVisitor)?;
    FixedVector::new(vec)
        .map_err(|e| serde::de::Error::custom(format!("invalid fixed vector: {e:?}")))
}
