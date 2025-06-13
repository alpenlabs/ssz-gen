//! Primitive types for SSZ serialization.
//!
//! This module provides primitive types that were previously imported from alloy-primitives,
//! now implemented locally to remove the ethereum-specific dependency.

use ruint::Uint;

/// A 256-bit unsigned integer type.
///
/// This replaces alloy_primitives::U256 to remove the Ethereum-specific dependency.
/// It's exactly the same type - ruint::Uint<256, 4> where:
/// - 256 = number of bits
/// - 4 = number of 64-bit limbs (256/64 = 4)
pub type U256 = Uint<256, 4>;

/// A 128-bit unsigned integer type.
pub type U128 = Uint<128, 2>;

/// A fixed-size byte array type.
///
/// This replaces alloy_primitives::FixedBytes<N> to remove the Ethereum dependency.
/// It's a simple wrapper around [u8; N] with convenient methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> FixedBytes<N> {
    /// Create a new FixedBytes filled with zeros
    pub const fn zero() -> Self {
        Self([0u8; N])
    }

    /// Create filled with a specific byte
    pub const fn repeat_byte(byte: u8) -> Self {
        Self([byte; N])
    }

    /// Create from a slice, padding with zeros if needed
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut result = [0u8; N];
        let len = slice.len().min(N);
        result[..len].copy_from_slice(&slice[..len]);
        Self(result)
    }

    /// Create with right padding from the given slice
    pub fn right_padding_from(slice: &[u8]) -> Self {
        let mut result = [0u8; N];
        let len = slice.len().min(N);
        result[..len].copy_from_slice(&slice[..len]);
        Self(result)
    }

    /// Create with left padding from the given slice  
    pub fn left_padding_from(slice: &[u8]) -> Self {
        let mut result = [0u8; N];
        let len = slice.len().min(N);
        let offset = N - len;
        result[offset..].copy_from_slice(&slice[..len]);
        Self(result)
    }

    /// Get as a byte slice
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Convert to the inner byte array
    pub const fn into_inner(self) -> [u8; N] {
        self.0
    }

    /// Create from a hex string (with or without 0x prefix)
    pub fn from_str(s: &str) -> Result<Self, hex::FromHexError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s)?;
        if bytes.len() != N {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut result = [0u8; N];
        result.copy_from_slice(&bytes);
        Ok(Self(result))
    }
}

impl<const N: usize> From<[u8; N]> for FixedBytes<N> {
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for FixedBytes<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// A 256-bit hash type (32 bytes).
pub type Hash256 = FixedBytes<32>;

impl Hash256 {
    /// A hash filled with zeros
    pub const ZERO: Self = Self::zero();
}

// SSZ Encode implementations
impl<const N: usize> crate::Encode for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn ssz_bytes_len(&self) -> usize {
        N
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }

    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl crate::Encode for U256 {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }

    fn ssz_bytes_len(&self) -> usize {
        32
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_le_slice());
    }
}

impl crate::Encode for U128 {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        16
    }

    fn ssz_bytes_len(&self) -> usize {
        16
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_le_slice());
    }
}

// SSZ Decode implementations
impl<const N: usize> crate::Decode for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, crate::DecodeError> {
        if bytes.len() != N {
            return Err(crate::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: N,
            });
        }

        let mut fixed_array = [0u8; N];
        fixed_array.copy_from_slice(bytes);

        Ok(Self(fixed_array))
    }
}

impl crate::Decode for U256 {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, crate::DecodeError> {
        let len = bytes.len();
        let expected = <Self as crate::Decode>::ssz_fixed_len();

        if len != expected {
            Err(crate::DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(U256::from_le_slice(bytes))
        }
    }
}

impl crate::Decode for U128 {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        16
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, crate::DecodeError> {
        let len = bytes.len();
        let expected = <Self as crate::Decode>::ssz_fixed_len();

        if len != expected {
            Err(crate::DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(U128::from_le_slice(bytes))
        }
    }
}
