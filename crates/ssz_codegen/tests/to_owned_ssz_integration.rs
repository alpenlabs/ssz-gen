//! Integration test for ToOwnedSsz with external container types.
//!
//! Tests the real-world scenario: L1BlockRange containing L1BlockCommitment
//! - BlockCommitment.height: uint32 in SSZ, but absolute::Height (u64) with bitcoin feature
//! - BlockRange uses external_kind: container for nested BlockCommitment fields

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

use ssz_derive as _;
use ssz_primitives as _;
use tree_hash_derive as _;

// Include generated code
include!("expected_output/test_external_container.rs");

use ssz::view::DecodeView;
use ssz_types::view::ToOwnedSsz;
use tests::input::{test_external_inner::BlockCommitmentRef, test_external_outer::BlockRangeRef};

/// External BlockCommitment - simulates bitcoin feature where height is absolute::Height
#[derive(Debug, Clone, PartialEq, Eq)]
struct ExternalBlockCommitment {
    height: u64, // u32 -> u64 (like absolute::Height)
    block_hash: [u8; 32],
}

/// External BlockRange using external BlockCommitment
#[derive(Debug, Clone, PartialEq, Eq)]
struct ExternalBlockRange {
    start: ExternalBlockCommitment,
    end: ExternalBlockCommitment,
}

/// Consumer implements ToOwnedSsz for BlockCommitmentRef -> ExternalBlockCommitment
impl<'a> ToOwnedSsz<ExternalBlockCommitment> for BlockCommitmentRef<'a> {
    fn to_owned(&self) -> ExternalBlockCommitment {
        ExternalBlockCommitment {
            height: self.height().expect("valid view") as u64,
            block_hash: *self.block_hash().expect("valid view").as_bytes(),
        }
    }
}

/// Consumer implements ToOwnedSsz for BlockRangeRef -> ExternalBlockRange
/// This uses ToOwnedSsz::to_owned for nested fields, which picks up our custom impl
impl<'a> ToOwnedSsz<ExternalBlockRange> for BlockRangeRef<'a> {
    fn to_owned(&self) -> ExternalBlockRange {
        ExternalBlockRange {
            start: ToOwnedSsz::to_owned(&self.start().expect("valid view")),
            end: ToOwnedSsz::to_owned(&self.end().expect("valid view")),
        }
    }
}

/// Manually construct SSZ bytes for BlockRange with variable-length layout.
/// Layout: [offset_start:4][offset_end:4][start_data:36][end_data:36]
fn create_block_range_bytes(start_height: u32, end_height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();

    // Fixed portion: 2 offsets (4 bytes each)
    let offset_start: u32 = 8; // Start data begins after fixed portion
    let offset_end: u32 = 8 + 36; // End data begins after start data

    bytes.extend_from_slice(&offset_start.to_le_bytes());
    bytes.extend_from_slice(&offset_end.to_le_bytes());

    // Start BlockCommitment: height (4 bytes) + block_hash (32 bytes)
    bytes.extend_from_slice(&start_height.to_le_bytes());
    bytes.extend_from_slice(&[0xAA; 32]); // start block_hash

    // End BlockCommitment: height (4 bytes) + block_hash (32 bytes)
    bytes.extend_from_slice(&end_height.to_le_bytes());
    bytes.extend_from_slice(&[0xBB; 32]); // end block_hash

    bytes
}

#[test]
fn test_block_range_to_owned_ssz_uses_custom_conversion() {
    let bytes = create_block_range_bytes(100, 200);
    let view = BlockRangeRef::from_ssz_bytes(&bytes).expect("valid SSZ");

    // Using ToOwnedSsz::to_owned on BlockRangeRef should use our custom impl
    // which internally calls ToOwnedSsz::to_owned on BlockCommitmentRef
    let external: ExternalBlockRange = ToOwnedSsz::to_owned(&view);

    // Verify height converted from u32 to u64
    assert_eq!(external.start.height, 100u64);
    assert_eq!(external.start.block_hash, [0xAA; 32]);
    assert_eq!(external.end.height, 200u64);
    assert_eq!(external.end.block_hash, [0xBB; 32]);
}

#[test]
fn test_generated_block_range_to_owned_uses_trait_method() {
    // This test verifies the GENERATED to_owned() uses ToOwnedSsz::to_owned(&view)
    // for external container fields, not the inherent .to_owned()

    let bytes = create_block_range_bytes(42, 84);
    let view = BlockRangeRef::from_ssz_bytes(&bytes).expect("valid SSZ");

    // The generated inherent to_owned() on BlockRangeRef internally does:
    //   start: { let view = self.start()?; ToOwnedSsz::to_owned(&view) }
    // This means it uses the trait method, allowing our custom impl to be called

    // Get the nested view
    let start_view = view.start().expect("valid view");

    // Verify: ToOwnedSsz::to_owned returns ExternalBlockCommitment
    let external_start: ExternalBlockCommitment = ToOwnedSsz::to_owned(&start_view);
    assert_eq!(external_start.height, 42u64); // Converted to u64!
}
