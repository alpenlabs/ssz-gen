// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tree_hash::Hash256;

/// For `Optional` fields in used in `StableContainer` for SSZ
/// We're using `Option<T>` for `Union[None, T]` so we need a separate type for this
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Optional<T> {
    /// Same as `None` in `Option<T>`
    #[default]
    None,
    /// Same as `Some(T)` in `Option<T>`
    Some(T),
}

impl<T> Optional<T> {
    /// Returns `true` if the optional contains a value.
    pub fn is_some(&self) -> bool {
        matches!(self, Optional::Some(_))
    }

    /// Returns `true` if the optional does not contain a value.
    pub fn is_none(&self) -> bool {
        matches!(self, Optional::None)
    }

    /// Returns the contained value, consuming the self value.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None` with the message "Optional is None".
    pub fn unwrap(self) -> T {
        match self {
            Optional::Some(value) => value,
            Optional::None => panic!("Optional is None"),
        }
    }

    /// Returns the contained value or a provided default.
    ///
    /// Consumes the `self` value.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Optional::Some(value) => value,
            Optional::None => default,
        }
    }

    /// Returns the contained value or computes it from a closure.
    ///
    /// Consumes the `self` value.
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Optional::Some(value) => value,
            Optional::None => f(),
        }
    }

    /// Converts from `&Optional<T>` to `Optional<&T>`.
    pub fn as_ref(&self) -> Optional<&T> {
        match self {
            Optional::Some(value) => Optional::Some(value),
            Optional::None => Optional::None,
        }
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Optional::Some(value),
            None => Optional::None,
        }
    }
}

impl<T: Clone> From<Optional<T>> for Option<T> {
    fn from(optional: Optional<T>) -> Self {
        match optional {
            Optional::Some(value) => Some(value),
            Optional::None => None,
        }
    }
}

impl<T> tree_hash::TreeHash for Optional<T>
where
    T: tree_hash::TreeHash,
{
    fn tree_hash_type() -> tree_hash::TreeHashType {
        T::tree_hash_type()
    }

    fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
        match self {
            Optional::Some(inner) => inner.tree_hash_packed_encoding(),
            Optional::None => unreachable!(),
        }
    }

    fn tree_hash_packing_factor() -> usize {
        T::tree_hash_packing_factor()
    }

    fn tree_hash_root(&self) -> Hash256 {
        match self {
            Optional::Some(inner) => inner.tree_hash_root(),
            Optional::None => unreachable!(),
        }
    }
}

impl<T> ssz::Encode for Optional<T>
where
    T: ssz::Encode,
{
    fn is_ssz_fixed_len() -> bool {
        T::is_ssz_fixed_len()
    }

    fn ssz_fixed_len() -> usize {
        T::ssz_fixed_len()
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        match self {
            Optional::None => {}
            Optional::Some(_) => {
                if let Optional::Some(inner) = self.as_ref() {
                    inner.ssz_append(buf);
                }
            }
        }
    }

    fn ssz_bytes_len(&self) -> usize {
        match self {
            Optional::None => 0,
            Optional::Some(inner) => inner.ssz_bytes_len(),
        }
    }
}

impl<T> ssz::Decode for Optional<T>
where
    T: ssz::Decode,
{
    fn is_ssz_fixed_len() -> bool {
        T::is_ssz_fixed_len()
    }

    fn ssz_fixed_len() -> usize {
        T::ssz_fixed_len()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Optional<T>, ssz::DecodeError> {
        if bytes.is_empty() {
            Ok(Optional::None)
        } else {
            T::from_ssz_bytes(bytes).map(Optional::Some)
        }
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Optional<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Optional::Some(value) => serializer.serialize_some(value),
            Optional::None => serializer.serialize_none(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Optional<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let option: Option<T> = Deserialize::deserialize(deserializer)?;
        Ok(option.into())
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T: arbitrary::Arbitrary<'a>> arbitrary::Arbitrary<'a> for Optional<T> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let inner = T::arbitrary(u)?;
        Ok(Optional::Some(inner))
    }
}

#[cfg(test)]
mod test {
    use ssz::*;

    use super::*;

    #[test]
    fn new() {
        let inner = 42;
        let optional: Optional<u64> = Optional::Some(inner);
        assert_eq!(optional, Optional::Some(42));
    }

    #[test]
    fn ssz_encode() {
        let inner = 42;
        let optional: Optional<u64> = Optional::Some(inner);
        assert_eq!(optional.as_ssz_bytes(), vec![42, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(optional.as_ssz_bytes(), 42u64.as_ssz_bytes());
        assert_eq!(
            <Optional<u64> as Encode>::ssz_fixed_len(),
            <u64 as Encode>::ssz_fixed_len()
        );
    }

    fn ssz_round_trip<T: Encode + Decode + std::fmt::Debug + PartialEq>(item: T) {
        let encoded = &item.as_ssz_bytes();
        assert_eq!(item.ssz_bytes_len(), encoded.len());
        assert_eq!(T::from_ssz_bytes(encoded), Ok(item));
    }

    #[test]
    fn test_ssz_round_trip() {
        ssz_round_trip::<Optional<u64>>(Optional::Some(42));
        ssz_round_trip::<Optional<u64>>(Optional::None);
    }

    #[test]
    #[should_panic]
    fn tree_hash_none() {
        let optional: Optional<u64> = Optional::None;
        optional.tree_hash_root();
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        use serde_json;

        let json = serde_json::json!(null);
        let result: Result<Optional<u64>, _> = serde_json::from_value(json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Optional::None);

        let json = serde_json::json!(10);
        let result: Result<Optional<u64>, _> = serde_json::from_value(json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Optional::Some(10));
    }
}
