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
/// Like [`read_variable_offset`], this assumes the offset entries are packed
/// at the end of the fixed portion; see [`validate_container`] for the
/// canonical interleaved layout.
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
/// Assumes the offset entries form a contiguous table at the end of the
/// fixed portion, which holds only when no fixed-size field follows a
/// variable-size one (e.g. all-`Optional` StableContainers). In canonical
/// SSZ each offset entry sits at its field's own position in the fixed
/// portion; use [`read_field_bytes`] with a field table for such layouts.
///
/// FIXME: the StableContainer/Profile codegen path still relies on this
/// end-packed assumption, which is wrong for Profiles that mix required
/// fixed-size fields after variable-size ones (pre-existing).
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

/// Owned-encoding layout facts for one container field: `(is_fixed, fixed_len)`.
///
/// `fixed_len` is the field's `ssz_fixed_len()`; for variable-size fields the
/// [`Encode`](crate::Encode) contract makes this `BYTES_PER_LENGTH_OFFSET` —
/// the size of the offset slot the field occupies in the fixed portion — so
/// summing `fixed_len` over a prefix of fields yields the next field's
/// position in the fixed portion.
pub type FieldInfo = (bool, usize);

/// Reads a 4-byte little-endian offset at `pos`.
fn read_offset_at(bytes: &[u8], pos: usize) -> Result<usize, DecodeError> {
    let end = pos
        .checked_add(BYTES_PER_LENGTH_OFFSET)
        .ok_or(DecodeError::OutOfBoundsByte { i: pos })?;
    if end > bytes.len() {
        return Err(DecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: end,
        });
    }
    let offset_bytes = &bytes[pos..end];
    Ok(u32::from_le_bytes([
        offset_bytes[0],
        offset_bytes[1],
        offset_bytes[2],
        offset_bytes[3],
    ]) as usize)
}

/// Slices the field at `index` out of `bytes` given the container's per-field
/// layout facts.
///
/// A fixed-size field sits inline in the fixed portion; a variable-size field
/// occupies a `BYTES_PER_LENGTH_OFFSET`-sized offset slot **at its own
/// position** in the fixed portion (offset entries are interleaved with fixed
/// fields in field order, not packed at the end). The field's end is the
/// offset of the next variable-size field, or the container end if there is
/// none.
///
/// `fields` entries are expected to be derived from the field types' owned
/// encoding ([`Encode`](crate::Encode) impls, or the `#[ssz(with = ...)]`
/// module when overridden), which keeps view layouts in agreement with the
/// owned encoding by construction.
///
/// # Arguments
///
/// * `bytes` - The complete container bytes
/// * `fields` - Per-field layout facts, in field order
/// * `index` - The index of the field to slice
///
/// # Returns
///
/// The field's byte slice, or an error if out of bounds.
pub fn read_field_bytes<'a>(
    bytes: &'a [u8],
    fields: &[FieldInfo],
    index: usize,
) -> Result<&'a [u8], DecodeError> {
    let &(is_fixed, fixed_len) = fields
        .get(index)
        .ok_or(DecodeError::OutOfBoundsByte { i: index })?;
    let pos: usize = fields[..index].iter().map(|&(_, len)| len).sum();

    if is_fixed {
        let end = pos
            .checked_add(fixed_len)
            .ok_or(DecodeError::OutOfBoundsByte { i: pos })?;
        if end > bytes.len() {
            return Err(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: end,
            });
        }
        Ok(&bytes[pos..end])
    } else {
        let start = read_offset_at(bytes, pos)?;

        // The field ends where the next variable-size field begins, whose
        // offset entry sits after the intervening fixed-size fields.
        let mut next_pos = pos + BYTES_PER_LENGTH_OFFSET;
        let mut end = bytes.len();
        for &(next_is_fixed, next_len) in &fields[index + 1..] {
            if next_is_fixed {
                next_pos += next_len;
            } else {
                end = read_offset_at(bytes, next_pos)?;
                break;
            }
        }

        if start > end || end > bytes.len() {
            return Err(DecodeError::OffsetsAreDecreasing(end));
        }
        Ok(&bytes[start..end])
    }
}

/// Validates container `bytes` against the per-field layout facts.
///
/// For an all-fixed container this checks the exact byte length; otherwise it
/// walks the fixed portion reading each variable-size field's offset entry at
/// its interleaved position, checking that the first offset points to the end
/// of the fixed portion, offsets do not decrease, and none exceeds the
/// container length.
///
/// # Arguments
///
/// * `bytes` - The complete container bytes
/// * `fields` - Per-field layout facts, in field order
pub fn validate_container(bytes: &[u8], fields: &[FieldInfo]) -> Result<(), DecodeError> {
    let fixed_portion_size: usize = fields.iter().map(|&(_, len)| len).sum();
    let num_variable_fields = fields.iter().filter(|&&(is_fixed, _)| !is_fixed).count();

    if num_variable_fields == 0 {
        if bytes.len() != fixed_portion_size {
            return Err(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: fixed_portion_size,
            });
        }
        return Ok(());
    }

    if bytes.len() < fixed_portion_size {
        return Err(DecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: fixed_portion_size,
        });
    }

    let mut pos = 0usize;
    let mut prev_offset: Option<usize> = None;
    for &(is_fixed, fixed_len) in fields {
        if !is_fixed {
            let offset = read_offset_at(bytes, pos)?;

            match prev_offset {
                // First offset must point to the start of the variable portion
                None if offset != fixed_portion_size => {
                    return Err(DecodeError::OffsetIntoFixedPortion(offset));
                }
                Some(prev) if offset < prev => {
                    return Err(DecodeError::OffsetsAreDecreasing(offset));
                }
                _ => {}
            }

            if offset > bytes.len() {
                return Err(DecodeError::OffsetOutOfBounds(offset));
            }

            prev_offset = Some(offset);
        }
        pos += fixed_len;
    }

    Ok(())
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

    #[test]
    fn read_field_bytes_fixed() {
        // Container: [u8][u16] -> 3 fixed bytes, no variable fields
        let fields: &[FieldInfo] = &[(true, 1), (true, 2)];
        let bytes = vec![0xAA, 0xBB, 0xCC];

        assert_eq!(read_field_bytes(&bytes, fields, 0).unwrap(), &[0xAA]);
        assert_eq!(read_field_bytes(&bytes, fields, 1).unwrap(), &[0xBB, 0xCC]);
        // Fixed slice past the end of the container
        assert!(read_field_bytes(&bytes[..2], fields, 1).is_err());
    }

    #[test]
    fn read_field_bytes_variable() {
        // Container: [u8][list] -> fixed portion = 1 + 4 (offset), list at 5..7
        let fields: &[FieldInfo] = &[(true, 1), (false, BYTES_PER_LENGTH_OFFSET)];
        let bytes = vec![
            0xAA, // u8 field
            0x05, 0x00, 0x00, 0x00, // offset 0 = 5
            0xBB, 0xCC, // list contents
        ];

        assert_eq!(read_field_bytes(&bytes, fields, 0).unwrap(), &[0xAA]);
        assert_eq!(read_field_bytes(&bytes, fields, 1).unwrap(), &[0xBB, 0xCC]);
    }

    #[test]
    fn read_field_bytes_variable_before_fixed() {
        // Container: [list][u32] -> the offset entry sits at position 0 (the
        // list field's own slot), NOT at the end of the fixed portion.
        let fields: &[FieldInfo] = &[(false, BYTES_PER_LENGTH_OFFSET), (true, 4)];
        let bytes = vec![
            0x08, 0x00, 0x00, 0x00, // offset 0 = 8 (list field's slot)
            0xDE, 0xAD, 0xBE, 0xEF, // u32 field
            0xBB, 0xCC, // list contents
        ];

        assert_eq!(read_field_bytes(&bytes, fields, 0).unwrap(), &[0xBB, 0xCC]);
        assert_eq!(
            read_field_bytes(&bytes, fields, 1).unwrap(),
            &[0xDE, 0xAD, 0xBE, 0xEF]
        );
    }

    #[test]
    fn read_field_bytes_interleaved() {
        // Container: [list][u8][list] -> slots at 0 and 5, u8 at 4.
        let fields: &[FieldInfo] = &[
            (false, BYTES_PER_LENGTH_OFFSET),
            (true, 1),
            (false, BYTES_PER_LENGTH_OFFSET),
        ];
        let bytes = vec![
            0x09, 0x00, 0x00, 0x00, // offset for field 0 = 9
            0xAA, // u8 field
            0x0B, 0x00, 0x00, 0x00, // offset for field 2 = 11
            0xBB, 0xCC, // field 0 contents
            0xDD, // field 2 contents
        ];

        assert_eq!(read_field_bytes(&bytes, fields, 0).unwrap(), &[0xBB, 0xCC]);
        assert_eq!(read_field_bytes(&bytes, fields, 1).unwrap(), &[0xAA]);
        assert_eq!(read_field_bytes(&bytes, fields, 2).unwrap(), &[0xDD]);
    }

    #[test]
    fn validate_container_fixed() {
        let fields: &[FieldInfo] = &[(true, 1), (true, 2)];

        assert!(validate_container(&[0x01, 0x02, 0x03], fields).is_ok());
        assert!(validate_container(&[0x01, 0x02], fields).is_err());
        assert!(validate_container(&[0x01, 0x02, 0x03, 0x04], fields).is_err());
    }

    #[test]
    fn validate_container_variable_before_fixed() {
        let fields: &[FieldInfo] = &[(false, BYTES_PER_LENGTH_OFFSET), (true, 4)];

        // Valid: offset entry at position 0 points to the end of the fixed
        // portion (8), u32 field at 4..8.
        let valid = vec![
            0x08, 0x00, 0x00, 0x00, // offset = 8
            0xDE, 0xAD, 0xBE, 0xEF, // u32 field
            0xBB, 0xCC, // list contents
        ];
        assert!(validate_container(&valid, fields).is_ok());

        // Invalid: the offset entry does not point to the variable portion.
        // An end-packed reading would instead interpret the u32 field's bytes
        // as the offset and reject valid encodings like the one above.
        let invalid = vec![
            0x04, 0x00, 0x00, 0x00, // offset = 4 (into fixed portion)
            0xDE, 0xAD, 0xBE, 0xEF, //
            0xBB, 0xCC,
        ];
        assert!(validate_container(&invalid, fields).is_err());
    }

    #[test]
    fn validate_container_offsets_decreasing() {
        let fields: &[FieldInfo] = &[
            (false, BYTES_PER_LENGTH_OFFSET),
            (false, BYTES_PER_LENGTH_OFFSET),
        ];

        let invalid = vec![
            0x08, 0x00, 0x00, 0x00, // offset 0 = 8
            0x07, 0x00, 0x00, 0x00, // offset 1 = 7 (< 8)
            0xAA,
        ];
        assert!(validate_container(&invalid, fields).is_err());
    }
}
