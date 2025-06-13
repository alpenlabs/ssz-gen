//! Primitive types for SSZ serialization.

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

/// A 32-byte hash type
pub type Hash256 = FixedBytes<32>;

/// A 20-byte Ethereum address type
pub type Address = FixedBytes<20>;

/// A 256-byte Ethereum bloom filter
pub type Bloom = FixedBytes<256>;

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

    /// Get as a byte slice
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
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
}

impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> From<[u8; N]> for FixedBytes<N> {
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl Address {
    /// An address filled with zeros
    pub const ZERO: Self = Self::zero();
}

impl Hash256 {
    /// A hash filled with zeros  
    pub const ZERO: Self = Self::zero();
}




/// A dynamic byte array  
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bytes(pub Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}