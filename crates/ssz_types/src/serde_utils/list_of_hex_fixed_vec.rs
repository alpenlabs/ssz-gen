// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Serialize `VariableList<FixedVector<u8, M>, N>` as list of 0x-prefixed hex string.
use crate::{FixedVector, VariableList};
use serde::{Deserializer, Serializer, ser::SerializeSeq};
use serde_derive::{Deserialize, Serialize};

/// A wrapper for a `FixedVector<u8, N>`
#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct WrappedListOwned<const N: usize>(
    #[serde(with = "crate::serde_utils::hex_fixed_vec")] FixedVector<u8, N>,
);

/// A wrapper for a `&FixedVector<u8, N>`
#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct WrappedListRef<'a, const N: usize>(
    #[serde(with = "crate::serde_utils::hex_fixed_vec")] &'a FixedVector<u8, N>,
);

/// Serialize a `VariableList<FixedVector<u8, M>, N>`
pub fn serialize<S, const M: usize, const N: usize>(
    list: &VariableList<FixedVector<u8, M>, N>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(list.len()))?;
    for bytes in list {
        seq.serialize_element(&WrappedListRef(bytes))?;
    }
    seq.end()
}

/// Visitor for deserializing a `VariableList<FixedVector<u8, M>, N>`
#[derive(Debug)]
pub struct Visitor<const M: usize, const N: usize>;

impl<'a, const M: usize, const N: usize> serde::de::Visitor<'a> for Visitor<M, N> {
    type Value = VariableList<FixedVector<u8, M>, N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a list of 0x-prefixed hex bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'a>,
    {
        let mut list: VariableList<FixedVector<u8, M>, N> = <_>::default();

        while let Some(val) = seq.next_element::<WrappedListOwned<M>>()? {
            list.push(val.0).map_err(|e| {
                serde::de::Error::custom(format!("failed to push value to list: {e:?}."))
            })?;
        }

        Ok(list)
    }
}

/// Deserialize a `VariableList<FixedVector<u8, M>, N>`
pub fn deserialize<'de, D, const M: usize, const N: usize>(
    deserializer: D,
) -> Result<VariableList<FixedVector<u8, M>, N>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(Visitor)
}
