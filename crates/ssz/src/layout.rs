//! SSZ layout computation and validation.
//!
//! This module provides a layout-based approach to SSZ encoding/decoding where we compute
//! field positions within containers without eagerly decoding all fields. This enables
//! true zero-copy views over SSZ-encoded data.
//!
//! ## Design
//!
//! - [`FieldLayout`]: Describes the position and type of a field within a container.
//! - [`ContainerLayout`]: Describes the complete layout of an SSZ container.
//! - Validation happens at wrap time without materializing field values.
//! - Field access is lazy - positions are computed on-demand.

use crate::{BYTES_PER_LENGTH_OFFSET, DecodeError};

/// Describes how to locate a field within an SSZ container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldLayout {
    /// The name of the field (for debugging).
    pub name: String,

    /// How to compute the field's offset.
    pub offset: FieldOffset,

    /// The type characteristics of the field.
    pub ty: FieldType,
}

/// Describes where a field is located within an SSZ container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOffset {
    /// Fixed offset from the start of the container (in bytes).
    ///
    /// Used for fixed-size fields at the beginning of the container.
    Fixed(usize),

    /// Index into the offset table for variable-length fields.
    ///
    /// The offset table is located at the start of the variable portion.
    /// Each entry is 4 bytes (u32 little-endian).
    Variable(usize),
}

/// Describes the type characteristics of a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// A basic fixed-size type (primitives, fixed arrays, etc.).
    Basic {
        /// The size of the type in bytes.
        size: usize,
    },

    /// A composite or variable-length type (containers, lists, etc.).
    Composite,
}

/// Describes the complete layout of an SSZ container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerLayout {
    /// The fields in the container, in order.
    pub fields: Vec<FieldLayout>,

    /// The total size of the fixed portion (in bytes).
    ///
    /// For containers with only fixed-size fields, this is the total size.
    /// For containers with variable-length fields, this is the size of the
    /// fixed portion (which contains the offset table for variable fields).
    pub fixed_portion_size: usize,

    /// The total size of the container if all fields are fixed-size.
    ///
    /// [`None`] if the container has any variable-length fields.
    pub fixed_size: Option<usize>,
}

impl ContainerLayout {
    /// Validates that the given bytes conform to this container layout.
    ///
    /// This performs structural validation without decoding field values:
    ///
    /// - For fixed-size containers: checks byte length matches expected size
    /// - For variable-size containers: validates offset table ordering and bounds
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The byte length is incorrect for a fixed-size container
    /// - The offset table is malformed
    /// - Offsets are out of order or out of bounds
    pub fn validate(&self, bytes: &[u8]) -> Result<(), DecodeError> {
        if let Some(expected_size) = self.fixed_size {
            // Fixed-size container: just check the length
            if bytes.len() != expected_size {
                return Err(DecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: expected_size,
                });
            }
            return Ok(());
        }

        // Variable-size container: validate offset table
        let num_variable_fields = self
            .fields
            .iter()
            .filter(|f| matches!(f.offset, FieldOffset::Variable(_)))
            .count();

        if num_variable_fields == 0 {
            // No variable fields, but fixed_size is None - this shouldn't happen
            // but let's handle it gracefully
            return Ok(());
        }

        // The fixed portion contains offsets for variable fields
        if bytes.len() < self.fixed_portion_size {
            return Err(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: self.fixed_portion_size,
            });
        }

        // Validate offset table
        validate_offset_table(bytes, self.fixed_portion_size, num_variable_fields)
    }
}

/// Validates the offset table for a variable-length container.
///
/// # Arguments
///
/// * `bytes` - The complete container bytes
/// * `fixed_portion_size` - The size of the fixed portion (where offset table starts)
/// * `num_variable_fields` - The number of variable-length fields
///
/// # Returns
///
/// `Ok(())` if the offset table is valid, otherwise an error.
fn validate_offset_table(
    bytes: &[u8],
    fixed_portion_size: usize,
    num_variable_fields: usize,
) -> Result<(), DecodeError> {
    if num_variable_fields == 0 {
        return Ok(());
    }

    // Calculate where the offset table starts
    // For containers with mixed fixed/variable fields, the fixed fields come first,
    // then the offset table for variable fields
    let offset_table_start = fixed_portion_size - (num_variable_fields * BYTES_PER_LENGTH_OFFSET);

    // Read and validate all offsets
    let mut prev_offset: Option<usize> = None;

    for i in 0..num_variable_fields {
        let offset_pos = offset_table_start + (i * BYTES_PER_LENGTH_OFFSET);

        if offset_pos + BYTES_PER_LENGTH_OFFSET > bytes.len() {
            return Err(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: offset_pos + BYTES_PER_LENGTH_OFFSET,
            });
        }

        let offset_bytes = &bytes[offset_pos..offset_pos + BYTES_PER_LENGTH_OFFSET];
        let offset = u32::from_le_bytes([
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ]) as usize;

        // First offset should point to the start of the variable portion
        if i == 0 {
            if offset != fixed_portion_size {
                return Err(DecodeError::OffsetIntoFixedPortion(offset));
            }
        } else {
            // Subsequent offsets must be >= previous offset
            if let Some(prev) = prev_offset
                && offset < prev
            {
                return Err(DecodeError::OffsetsAreDecreasing(offset));
            }
        }

        // Offset must not exceed container length
        if offset > bytes.len() {
            return Err(DecodeError::OffsetOutOfBounds(offset));
        }

        prev_offset = Some(offset);
    }

    Ok(())
}

/// Helper to read an offset from the offset table.
///
/// # Arguments
///
/// * `bytes` - The complete container bytes
/// * `fixed_portion_size` - The size of the fixed portion
/// * `num_variable_fields` - Total number of variable fields
/// * `index` - The index of the offset to read (0-based)
///
/// # Returns
///
/// The offset value, or an error if out of bounds.
pub fn read_variable_offset(
    bytes: &[u8],
    fixed_portion_size: usize,
    num_variable_fields: usize,
    index: usize,
) -> Result<usize, DecodeError> {
    if index >= num_variable_fields {
        return Err(DecodeError::OutOfBoundsByte { i: index });
    }

    let offset_table_start = fixed_portion_size - (num_variable_fields * BYTES_PER_LENGTH_OFFSET);
    let offset_pos = offset_table_start + (index * BYTES_PER_LENGTH_OFFSET);

    if offset_pos + BYTES_PER_LENGTH_OFFSET > bytes.len() {
        return Err(DecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: offset_pos + BYTES_PER_LENGTH_OFFSET,
        });
    }

    let offset_bytes = &bytes[offset_pos..offset_pos + BYTES_PER_LENGTH_OFFSET];
    let offset = u32::from_le_bytes([
        offset_bytes[0],
        offset_bytes[1],
        offset_bytes[2],
        offset_bytes[3],
    ]) as usize;

    Ok(offset)
}

/// Helper to read an offset or return the end of the container.
///
/// This is useful for determining the end of a variable-length field:
///
/// - If `index` is valid, returns the offset at that index
/// - If `index == num_variable_fields`, returns the container length
/// - Otherwise, returns an error
pub fn read_variable_offset_or_end(
    bytes: &[u8],
    fixed_portion_size: usize,
    num_variable_fields: usize,
    index: usize,
) -> Result<usize, DecodeError> {
    if index == num_variable_fields {
        Ok(bytes.len())
    } else {
        read_variable_offset(bytes, fixed_portion_size, num_variable_fields, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_size_container_validation() {
        // Layout for a container with two u8 fields (2 bytes total)
        let layout = ContainerLayout {
            fields: vec![
                FieldLayout {
                    name: "a".to_string(),
                    offset: FieldOffset::Fixed(0),
                    ty: FieldType::Basic { size: 1 },
                },
                FieldLayout {
                    name: "b".to_string(),
                    offset: FieldOffset::Fixed(1),
                    ty: FieldType::Basic { size: 1 },
                },
            ],
            fixed_portion_size: 2,
            fixed_size: Some(2),
        };

        // Valid: exactly 2 bytes
        assert!(layout.validate(&[0x01, 0x02]).is_ok());

        // Invalid: too short
        assert!(layout.validate(&[0x01]).is_err());

        // Invalid: too long
        assert!(layout.validate(&[0x01, 0x02, 0x03]).is_err());
    }

    #[test]
    fn variable_size_container_validation() {
        // Layout for a container with one fixed u8 field and one variable-length field
        // Fixed portion: 1 byte (u8) + 4 bytes (offset) = 5 bytes
        let layout = ContainerLayout {
            fields: vec![
                FieldLayout {
                    name: "a".to_string(),
                    offset: FieldOffset::Fixed(0),
                    ty: FieldType::Basic { size: 1 },
                },
                FieldLayout {
                    name: "b".to_string(),
                    offset: FieldOffset::Variable(0),
                    ty: FieldType::Composite,
                },
            ],
            fixed_portion_size: 5,
            fixed_size: None,
        };

        // Valid: fixed portion + some variable data
        // Offset at byte 1: 0x05 0x00 0x00 0x00 (points to byte 5, start of variable portion)
        let valid_bytes = vec![0x01, 0x05, 0x00, 0x00, 0x00, 0xAA, 0xBB];
        assert!(layout.validate(&valid_bytes).is_ok());

        // Invalid: offset points into fixed portion
        let invalid_bytes = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0xAA, 0xBB];
        assert!(layout.validate(&invalid_bytes).is_err());

        // Invalid: offset out of bounds
        let invalid_bytes = vec![0x01, 0x10, 0x00, 0x00, 0x00, 0xAA];
        assert!(layout.validate(&invalid_bytes).is_err());

        // Invalid: too short (missing offset table)
        assert!(layout.validate(&[0x01]).is_err());
    }

    #[test]
    fn multiple_variable_fields_validation() {
        // Container with two variable-length fields
        // Fixed portion: 8 bytes (two 4-byte offsets)
        let layout = ContainerLayout {
            fields: vec![
                FieldLayout {
                    name: "a".to_string(),
                    offset: FieldOffset::Variable(0),
                    ty: FieldType::Composite,
                },
                FieldLayout {
                    name: "b".to_string(),
                    offset: FieldOffset::Variable(1),
                    ty: FieldType::Composite,
                },
            ],
            fixed_portion_size: 8,
            fixed_size: None,
        };

        // Valid: two offsets, both pointing to valid positions
        // Offset 0: 0x08 (start of variable portion)
        // Offset 1: 0x0A (two bytes into variable portion)
        // Variable data: 0xAA 0xBB (for field a), 0xCC (for field b)
        let valid_bytes = vec![
            0x08, 0x00, 0x00, 0x00, // offset 0 = 8
            0x0A, 0x00, 0x00, 0x00, // offset 1 = 10
            0xAA, 0xBB, // field a data
            0xCC, // field b data
        ];
        assert!(layout.validate(&valid_bytes).is_ok());

        // Invalid: offsets are decreasing
        let invalid_bytes = vec![
            0x0A, 0x00, 0x00, 0x00, // offset 0 = 10
            0x08, 0x00, 0x00, 0x00, // offset 1 = 8 (< offset 0)
            0xAA, 0xBB, 0xCC,
        ];
        assert!(layout.validate(&invalid_bytes).is_err());
    }

    #[test]
    fn read_variable_offset_success() {
        // Container with two variable fields
        let bytes = vec![
            0x08, 0x00, 0x00, 0x00, // offset 0 = 8
            0x0A, 0x00, 0x00, 0x00, // offset 1 = 10
            0xAA, 0xBB, 0xCC,
        ];

        assert_eq!(read_variable_offset(&bytes, 8, 2, 0).unwrap(), 8);
        assert_eq!(read_variable_offset(&bytes, 8, 2, 1).unwrap(), 10);
        assert!(read_variable_offset(&bytes, 8, 2, 2).is_err());
    }

    #[test]
    fn read_variable_offset_or_end_success() {
        let bytes = vec![
            0x08, 0x00, 0x00, 0x00, // offset 0 = 8
            0x0A, 0x00, 0x00, 0x00, // offset 1 = 10
            0xAA, 0xBB, 0xCC,
        ];

        assert_eq!(read_variable_offset_or_end(&bytes, 8, 2, 0).unwrap(), 8);
        assert_eq!(read_variable_offset_or_end(&bytes, 8, 2, 1).unwrap(), 10);
        assert_eq!(read_variable_offset_or_end(&bytes, 8, 2, 2).unwrap(), 11); // End of container
        assert!(read_variable_offset_or_end(&bytes, 8, 2, 3).is_err());
    }
}
