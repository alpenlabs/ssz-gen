// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Serialize `VariableList<u8, N>` as 0x-prefixed hex string.
use serde::{Deserializer, Serializer};
use ssz::serde_utils::hex::{self, PrefixedHexVisitor};

use crate::VariableList;

/// Serialize a `VariableList<u8, N>` as a 0x-prefixed hex string.
pub fn serialize<S, const N: usize>(
    bytes: &VariableList<u8, N>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(&**bytes))
}

/// Deserialize a `VariableList<u8, N>` from a 0x-prefixed hex string.
pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<VariableList<u8, N>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = deserializer.deserialize_str(PrefixedHexVisitor)?;
    VariableList::new(bytes)
        .map_err(|e| serde::de::Error::custom(format!("invalid variable list: {e:?}")))
}
