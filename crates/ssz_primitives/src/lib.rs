// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Primitive types for SSZ serialization.
//!
//! This module provides primitive types that were previously imported from alloy-primitives,
//! now implemented locally to remove the ethereum-specific dependency.

use std::str::FromStr;

#[cfg(feature = "rand")]
use rand::RngCore;
use ruint::Uint;

/// A 256-bit unsigned integer type.
pub type U256 = Uint<256, 4>;

/// A 128-bit unsigned integer type.
pub type U128 = Uint<128, 2>;

/// A fixed-size byte array type.
///
/// This replaces `alloy_primitives::FixedBytes<N>` to remove the Ethereum dependency.
/// It's a simple wrapper around [u8; N] with convenient methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> FixedBytes<N> {
    /// Create a new FixedBytes filled with zeros
    pub const fn zero() -> Self {
        Self([0u8; N])
    }

    /// Create a new FixedBytes filled with random bytes
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        let mut bytes = Self::zero();
        bytes.randomize();
        bytes
    }

    /// Fill this FixedBytes with random bytes
    #[cfg(feature = "rand")]
    pub fn randomize(&mut self) {
        rand::thread_rng().fill_bytes(&mut self.0);
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
    pub fn from_hex_str(s: &str) -> Result<Self, hex::FromHexError> {
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

impl<const N: usize> FromStr for FixedBytes<N> {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex_str(s)
    }
}

/// A 256-bit hash type (32 bytes).
pub type Hash256 = FixedBytes<32>;

impl Hash256 {
    /// A hash filled with zeros
    pub const ZERO: Self = Self::zero();
}

// Conversion from U256 to Hash256 for tests
impl From<U256> for Hash256 {
    fn from(value: U256) -> Self {
        // Convert U256 to 32-byte array in little-endian format
        Self(value.to_le_bytes::<32>())
    }
}
