//! SSZ spec test vectors for tree hash and encoding.
//!
//! These vectors can be independently verified using any SSZ implementation
//! (remerkleable, Nimbus, Lodestar, etc.).

use digest as _;
use rand as _;
use sha2 as _;
use smallvec as _;
use ssz::{BitList, BitVector};
use ssz_derive as _;
use ssz_primitives as _;
use ssz_types::{FixedVector, VariableList};
use thiserror as _;
use tree_hash::{Hash256, Sha256Hasher, TreeHash, TreeHashDigest};
use tree_hash_derive::TreeHash;

// =============================================================================
// Tree Hash - Basic Types
// =============================================================================

/// uint64(1): 0x01 followed by 31 zero bytes
pub const HASH_U64_ONE: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// =============================================================================
// Tree Hash - Fixed Bytes (Bytes32)
// =============================================================================

/// Bytes32 of all 0x11: root equals the value itself
pub const HASH_BYTES32_11: [u8; 32] = [0x11; 32];

/// Bytes32 of all 0xFF: root equals the value itself
pub const HASH_BYTES32_FF: [u8; 32] = [0xFF; 32];

// =============================================================================
// Tree Hash - Union Types
// =============================================================================

/// Union[None, uint64]::Empty (selector 0)
pub const HASH_UNION_EMPTY: [u8; 32] = [
    0xf5, 0xa5, 0xfd, 0x42, 0xd1, 0x6a, 0x20, 0x30, 0x27, 0x98, 0xef, 0x6e, 0xd3, 0x09, 0x97, 0x9b,
    0x43, 0x00, 0x3d, 0x23, 0x20, 0xd9, 0xf0, 0xe8, 0xea, 0x98, 0x31, 0xa9, 0x27, 0x59, 0xfb, 0x4b,
];

/// Union[None, uint64]::Value(0) (selector 1)
pub const HASH_UNION_VALUE_0: [u8; 32] = [
    0xcb, 0x59, 0x28, 0x44, 0x12, 0x1d, 0x92, 0x6f, 0x1c, 0xa3, 0xad, 0x4e, 0x1d, 0x6f, 0xb9, 0xd8,
    0xe2, 0x60, 0xed, 0x6e, 0x32, 0x16, 0x36, 0x1f, 0x77, 0x32, 0xe9, 0x75, 0xa0, 0xe8, 0xbb, 0xf6,
];

// =============================================================================
// Tree Hash - Containers
// =============================================================================

/// Container { a: uint64, b: bool } with a=0, b=false
pub const HASH_CONTAINER_ZEROS: [u8; 32] = HASH_UNION_EMPTY;

// =============================================================================
// Tree Hash - FixedVector
// =============================================================================

/// Vector[uint64, 4](1, 2, 3, 4): packed into single 32-byte chunk (identity)
pub const HASH_FIXED_VECTOR_1234: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// =============================================================================
// Tree Hash - VariableList
// =============================================================================

/// List[uint64, 8]() empty
pub const HASH_LIST_U64_EMPTY: [u8; 32] = [
    0x7a, 0x05, 0x01, 0xf5, 0x95, 0x7b, 0xdf, 0x9c, 0xb3, 0xa8, 0xff, 0x49, 0x66, 0xf0, 0x22, 0x65,
    0xf9, 0x68, 0x65, 0x8b, 0x7a, 0x9c, 0x62, 0x64, 0x2c, 0xba, 0x11, 0x65, 0xe8, 0x66, 0x42, 0xf5,
];

/// List[uint64, 8](1, 2, 3, 4)
pub const HASH_LIST_U64_1234: [u8; 32] = [
    0x95, 0xa2, 0xf2, 0x52, 0xed, 0x26, 0x59, 0xcc, 0xf7, 0x5e, 0x88, 0x21, 0xf0, 0x57, 0x57, 0xc4,
    0x66, 0x3f, 0xce, 0x68, 0xe8, 0x9d, 0x02, 0x90, 0xab, 0xf5, 0xc3, 0x3d, 0x77, 0x29, 0x35, 0xae,
];

// =============================================================================
// Tree Hash - BitVector
// =============================================================================

/// Bitvector[8](all True): 0xFF padded to 32 bytes
pub const HASH_BITVECTOR_8_ALL_TRUE: [u8; 32] = [
    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// =============================================================================
// Tree Hash - BitList
// =============================================================================

/// Bitlist[8]() empty
pub const HASH_BITLIST_8_EMPTY: [u8; 32] = HASH_UNION_EMPTY;

/// Bitlist[8](True, False, True, False): 4 bits set
pub const HASH_BITLIST_8_TFTT: [u8; 32] = [
    0xe9, 0x07, 0x22, 0xeb, 0x4d, 0x2a, 0x89, 0x17, 0x00, 0xf1, 0xf3, 0xaa, 0x2e, 0x95, 0x66, 0x1e,
    0x70, 0x7b, 0x19, 0xe6, 0x0e, 0x14, 0x7a, 0x96, 0xf8, 0xcf, 0x08, 0x9e, 0x8c, 0xbc, 0x4b, 0xec,
];

// =============================================================================
// Tree Hash - Multi-variant Union
// =============================================================================

/// Union[Empty, u8, u16, u64]::U8(1) at selector 1
pub const HASH_MULTI_UNION_U8_1: [u8; 32] = [
    0x56, 0xd8, 0xa6, 0x6f, 0xba, 0xe0, 0x30, 0x0e, 0xfb, 0xa7, 0xec, 0x2c, 0x53, 0x19, 0x73, 0xaa,
    0xae, 0x22, 0xe7, 0xa2, 0xed, 0x6d, 0xed, 0x08, 0x1b, 0x5b, 0x32, 0xd0, 0x7a, 0x32, 0x78, 0x0a,
];

/// Union[Empty, u8, u16, u64]::U16(1000) at selector 2
pub const HASH_MULTI_UNION_U16_1000: [u8; 32] = [
    0x25, 0x30, 0xc3, 0xb7, 0xf0, 0xfd, 0xcf, 0x8f, 0x66, 0xe5, 0x15, 0x8d, 0xf4, 0xc7, 0xe9, 0xe2,
    0x13, 0x08, 0x4e, 0xc4, 0xec, 0x3f, 0xf6, 0xcc, 0x8f, 0xe2, 0x24, 0x08, 0xa0, 0xc2, 0x89, 0x5a,
];

/// Union[None, uint64]::Value(42) at selector 1
pub const HASH_UNION_VALUE_42: [u8; 32] = [
    0xbd, 0x7e, 0xb7, 0xbb, 0xb9, 0xe2, 0x2c, 0xb7, 0x84, 0xbd, 0x59, 0x24, 0x7b, 0x4b, 0x69, 0x77,
    0xce, 0xb5, 0x1d, 0xe6, 0x71, 0x3c, 0xf2, 0x99, 0x38, 0x55, 0x8b, 0x71, 0x46, 0x46, 0x78, 0xb3,
];

// =============================================================================
// Tree Hash - Container with specific values
// =============================================================================

/// Container { a: uint64, b: bool } with a=1, b=true
pub const HASH_CONTAINER_ONES: [u8; 32] = [
    0x56, 0xd8, 0xa6, 0x6f, 0xba, 0xe0, 0x30, 0x0e, 0xfb, 0xa7, 0xec, 0x2c, 0x53, 0x19, 0x73, 0xaa,
    0xae, 0x22, 0xe7, 0xa2, 0xed, 0x6d, 0xed, 0x08, 0x1b, 0x5b, 0x32, 0xd0, 0x7a, 0x32, 0x78, 0x0a,
];

// =============================================================================
// Tree Hash - Union Edge Cases
// =============================================================================

/// Union[None, u64]::Value(MAX) at selector 1
pub const HASH_UNION_MAX_U64: [u8; 32] = [
    0xf9, 0xb5, 0x75, 0x4b, 0xa4, 0x17, 0x6d, 0x69, 0x70, 0x18, 0x24, 0x21, 0x08, 0x43, 0xda, 0xde,
    0x88, 0xdf, 0x08, 0xa6, 0x09, 0xae, 0xb3, 0xf0, 0x21, 0xe9, 0x36, 0xa7, 0xcf, 0x83, 0x03, 0xef,
];

/// Container { value: uint64 } with value=42
pub const HASH_DATA_VARIANT_42: [u8; 32] = [
    0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// Union[None, DataVariant]::Data({value: 42}) at selector 1
pub const HASH_UNION_CONTAINER_42: [u8; 32] = [
    0xbd, 0x7e, 0xb7, 0xbb, 0xb9, 0xe2, 0x2c, 0xb7, 0x84, 0xbd, 0x59, 0x24, 0x7b, 0x4b, 0x69, 0x77,
    0xce, 0xb5, 0x1d, 0xe6, 0x71, 0x3c, 0xf2, 0x99, 0x38, 0x55, 0x8b, 0x71, 0x46, 0x46, 0x78, 0xb3,
];

/// Container { state: Union[None, u64] } with state=Value(100)
pub const HASH_CONTAINER_UNION_100: [u8; 32] = [
    0x2b, 0xe9, 0x3e, 0xf4, 0xea, 0x87, 0x6c, 0x2e, 0x5a, 0x6a, 0x4a, 0x89, 0xc1, 0x36, 0xe1, 0x27,
    0x3d, 0x6d, 0xef, 0xbe, 0x34, 0x0b, 0xcb, 0x21, 0x31, 0x5b, 0xb1, 0xb6, 0xad, 0xa5, 0xdc, 0x4f,
];

/// Union[u8, u16, u32, u64]::U8(255) at selector 0 (no None variant)
pub const HASH_BIG_UNION_U8_255_SEL0: [u8; 32] = [
    0x48, 0x3d, 0x5e, 0x8c, 0x70, 0xec, 0xbb, 0xd7, 0xd7, 0x6b, 0x34, 0x32, 0x98, 0xd1, 0x95, 0x8c,
    0xec, 0x12, 0x1b, 0x6d, 0x1f, 0xc2, 0x4d, 0x72, 0x80, 0x14, 0x42, 0x31, 0x7b, 0x82, 0xcf, 0xc0,
];

/// Union[u8, u16, u32, u64]::U16(65535) at selector 1
pub const HASH_BIG_UNION_U16_MAX_SEL1: [u8; 32] = [
    0xdf, 0xb2, 0xfd, 0xb7, 0xaa, 0x3e, 0xc8, 0x21, 0x2d, 0x7c, 0x3c, 0x29, 0x63, 0xa5, 0x32, 0xa0,
    0xc4, 0x30, 0x7b, 0xff, 0x7e, 0x52, 0x31, 0xbd, 0x7e, 0x37, 0x4c, 0x11, 0x8e, 0x5a, 0xf7, 0x6c,
];

/// Union[u8, u16, u32, u64]::U32(MAX) at selector 2
pub const HASH_BIG_UNION_U32_MAX_SEL2: [u8; 32] = [
    0xad, 0x6c, 0x69, 0x9f, 0x1a, 0xb6, 0xcf, 0xd5, 0xec, 0x83, 0xc8, 0x76, 0x01, 0x72, 0x2e, 0xfc,
    0xfe, 0x52, 0xa1, 0x8b, 0x6f, 0xef, 0xac, 0x47, 0x79, 0x71, 0xe2, 0xe6, 0xed, 0xbd, 0x75, 0xa8,
];

/// Union[u8, u16, u32, u64]::U64(0xDEADBEEFCAFEBABE) at selector 3
pub const HASH_BIG_UNION_U64_DEADBEEF_SEL3: [u8; 32] = [
    0x25, 0x90, 0x7e, 0x23, 0xe5, 0xc6, 0xf5, 0xa9, 0xc4, 0xd9, 0x1a, 0x7a, 0xdf, 0x85, 0xa2, 0x71,
    0x4d, 0x7d, 0x3d, 0xae, 0x98, 0x0a, 0x8a, 0x30, 0xe0, 0x97, 0x23, 0xbf, 0xd8, 0x0f, 0x9f, 0xc2,
];

/// Union[None, u8, u16, u64]::U64(1000000) at selector 3
pub const HASH_MULTI_UNION_U64_1000000: [u8; 32] = [
    0x70, 0x19, 0x26, 0x73, 0x67, 0xe1, 0xc2, 0x8b, 0xfc, 0x09, 0x00, 0x72, 0xa2, 0xf2, 0x49, 0x3c,
    0x80, 0x55, 0x85, 0x0f, 0xb0, 0xa0, 0xab, 0xa1, 0x7a, 0x8a, 0xe4, 0x27, 0x4c, 0x08, 0x3a, 0x91,
];

// =============================================================================
// Test Types
// =============================================================================

/// Union type for testing Union[None, uint64]
#[derive(Debug, TreeHash)]
#[tree_hash(enum_behaviour = "union")]
pub enum UnionNoneU64 {
    /// Empty/None variant at selector 0
    Empty,
    /// Value variant at selector 1
    Value(u64),
}

/// Simple container with two basic fields
#[derive(Debug, TreeHash)]
pub struct SimpleContainer {
    /// First field: uint64
    pub a: u64,
    /// Second field: bool
    pub b: bool,
}

/// Container with a nested container
#[derive(Debug, TreeHash)]
pub struct NestedContainer {
    /// Outer field
    pub x: u64,
    /// Nested container
    pub inner: SimpleContainer,
}

/// Container with Bytes32 field
#[derive(Debug, TreeHash)]
pub struct ContainerWithBytes32 {
    /// A 32-byte value
    pub data: [u8; 32],
}

/// Multi-variant union for testing different selectors
#[derive(Debug, TreeHash)]
#[tree_hash(enum_behaviour = "union")]
pub enum MultiUnion {
    /// Empty variant at selector 0
    Empty,
    /// u8 variant at selector 1
    U8(u8),
    /// u16 variant at selector 2
    U16(u16),
    /// u64 variant at selector 3
    U64(u64),
}

/// Union without None variant (selectors 0,1,2,3 are all data types)
#[derive(Debug, TreeHash)]
#[tree_hash(enum_behaviour = "union")]
pub enum BigUnion {
    /// u8 at selector 0
    U8(u8),
    /// u16 at selector 1
    U16(u16),
    /// u32 at selector 2
    U32(u32),
    /// u64 at selector 3
    U64(u64),
}

/// Container with a single u64 field (for testing union containing container)
#[derive(Debug, TreeHash)]
pub struct DataVariant {
    /// Single u64 value
    pub value: u64,
}

/// Union with empty and container variants
#[derive(Debug, TreeHash)]
#[tree_hash(enum_behaviour = "union")]
pub enum UnionEmptyData {
    /// Empty variant at selector 0
    Empty,
    /// Container variant at selector 1
    Data(DataVariant),
}

/// Container containing a union field
#[derive(Debug, TreeHash)]
pub struct ContainerWithUnion {
    /// Union field
    pub state: UnionNoneU64,
}

// =============================================================================
// Tests - Basic Types
// =============================================================================

#[test]
fn test_uint64_tree_hash() {
    assert_eq!(
        <u64 as TreeHash<Sha256Hasher>>::tree_hash_root(&0u64),
        Hash256::ZERO,
        "uint64(0) should hash to all zeros"
    );

    assert_eq!(
        <u64 as TreeHash<Sha256Hasher>>::tree_hash_root(&1u64),
        Hash256::from_slice(&HASH_U64_ONE),
        "tree_hash_root of uint64(1)"
    );

    // Additional uint64 tests
    assert_eq!(
        <u64 as TreeHash<Sha256Hasher>>::tree_hash_root(&42u64),
        Hash256::from_slice(&[
            0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        "tree_hash_root of uint64(42)"
    );

    assert_eq!(
        <u64 as TreeHash<Sha256Hasher>>::tree_hash_root(&u64::MAX),
        Hash256::from_slice(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        "tree_hash_root of uint64(MAX)"
    );
}

#[test]
fn test_uint32_tree_hash() {
    assert_eq!(
        <u32 as TreeHash<Sha256Hasher>>::tree_hash_root(&0u32),
        Hash256::ZERO,
        "uint32(0) should hash to all zeros"
    );

    assert_eq!(
        <u32 as TreeHash<Sha256Hasher>>::tree_hash_root(&1u32),
        Hash256::from_slice(&HASH_U64_ONE),
        "tree_hash_root of uint32(1)"
    );

    assert_eq!(
        <u32 as TreeHash<Sha256Hasher>>::tree_hash_root(&u32::MAX),
        Hash256::from_slice(&[
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        "tree_hash_root of uint32(MAX)"
    );
}

#[test]
fn test_uint16_tree_hash() {
    assert_eq!(
        <u16 as TreeHash<Sha256Hasher>>::tree_hash_root(&0u16),
        Hash256::ZERO,
        "uint16(0) should hash to all zeros"
    );

    assert_eq!(
        <u16 as TreeHash<Sha256Hasher>>::tree_hash_root(&1u16),
        Hash256::from_slice(&HASH_U64_ONE),
        "tree_hash_root of uint16(1)"
    );
}

#[test]
fn test_uint8_tree_hash() {
    assert_eq!(
        <u8 as TreeHash<Sha256Hasher>>::tree_hash_root(&0u8),
        Hash256::ZERO,
        "uint8(0) should hash to all zeros"
    );

    assert_eq!(
        <u8 as TreeHash<Sha256Hasher>>::tree_hash_root(&1u8),
        Hash256::from_slice(&HASH_U64_ONE),
        "tree_hash_root of uint8(1)"
    );

    assert_eq!(
        <u8 as TreeHash<Sha256Hasher>>::tree_hash_root(&255u8),
        Hash256::from_slice(&[
            0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        "tree_hash_root of uint8(255)"
    );
}

#[test]
fn test_bool_tree_hash() {
    assert_eq!(
        <bool as TreeHash<Sha256Hasher>>::tree_hash_root(&false),
        Hash256::ZERO,
        "bool(false) should hash to all zeros"
    );

    assert_eq!(
        <bool as TreeHash<Sha256Hasher>>::tree_hash_root(&true),
        Hash256::from_slice(&HASH_U64_ONE),
        "tree_hash_root of bool(true)"
    );
}

// =============================================================================
// Tests - Fixed Bytes
// =============================================================================

#[test]
fn test_bytes32_tree_hash() {
    // Bytes32 of all zeros - root equals itself (identity)
    let zeros: [u8; 32] = [0u8; 32];
    assert_eq!(
        <[u8; 32] as TreeHash<Sha256Hasher>>::tree_hash_root(&zeros),
        Hash256::ZERO,
        "Bytes32(zeros) should equal itself"
    );

    // Bytes32 of all 0x11 - root equals itself (identity)
    let elevens: [u8; 32] = [0x11; 32];
    assert_eq!(
        <[u8; 32] as TreeHash<Sha256Hasher>>::tree_hash_root(&elevens),
        Hash256::from_slice(&HASH_BYTES32_11),
        "Bytes32(0x11...) should equal itself"
    );

    // Bytes32 of all 0xFF - root equals itself (identity)
    let ones: [u8; 32] = [0xFF; 32];
    assert_eq!(
        <[u8; 32] as TreeHash<Sha256Hasher>>::tree_hash_root(&ones),
        Hash256::from_slice(&HASH_BYTES32_FF),
        "Bytes32(0xFF...) should equal itself"
    );
}

#[test]
fn test_fixed_vector_tree_hash() {
    // FixedVector of 4 uint64 zeros
    let vec: FixedVector<u64, 4> = FixedVector::from(vec![0u64, 0, 0, 0]);
    let hash = <FixedVector<u64, 4> as TreeHash<Sha256Hasher>>::tree_hash_root(&vec);

    // 4 uint64s pack into 1 chunk (4 * 8 = 32 bytes)
    // All zeros -> chunk is [0; 32]
    // Single chunk -> root equals chunk
    assert_eq!(hash, Hash256::ZERO, "FixedVector<u64, 4> of zeros");
}

// =============================================================================
// Tests - Union Types
// =============================================================================

#[test]
fn test_union_tree_hash() {
    // Empty variant (selector 0)
    assert_eq!(
        UnionNoneU64::Empty.tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_EMPTY),
        "Union::Empty should be SHA256([0; 64])"
    );

    // Value(0) variant (selector 1)
    assert_eq!(
        UnionNoneU64::Value(0).tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_VALUE_0),
        "Union::Value(0) should match expected hash"
    );

    // Verify different values produce different hashes
    let hash_value_1 = UnionNoneU64::Value(1).tree_hash_root();
    let hash_value_42 = UnionNoneU64::Value(42).tree_hash_root();

    assert_ne!(
        hash_value_1,
        Hash256::from_slice(&HASH_UNION_VALUE_0),
        "Value(1) should differ from Value(0)"
    );
    assert_ne!(
        hash_value_42,
        Hash256::from_slice(&HASH_UNION_VALUE_0),
        "Value(42) should differ from Value(0)"
    );
    assert_ne!(
        hash_value_1, hash_value_42,
        "Value(1) should differ from Value(42)"
    );
}

// =============================================================================
// Tests - Containers
// =============================================================================

#[test]
fn test_container_tree_hash() {
    // Container with all zeros
    let container_zeros = SimpleContainer { a: 0, b: false };
    assert_eq!(
        container_zeros.tree_hash_root(),
        Hash256::from_slice(&HASH_CONTAINER_ZEROS),
        "Container(0, false) should equal merkle root of two zero leaves"
    );

    // Container with non-zero values should differ
    let container_ones = SimpleContainer { a: 1, b: true };
    assert_ne!(
        container_ones.tree_hash_root(),
        Hash256::from_slice(&HASH_CONTAINER_ZEROS),
        "Container(1, true) should differ from Container(0, false)"
    );

    // Different field values produce different hashes
    let c1 = SimpleContainer { a: 1, b: false };
    let c2 = SimpleContainer { a: 0, b: true };
    let c3 = SimpleContainer { a: 1, b: true };

    assert_ne!(c1.tree_hash_root(), c2.tree_hash_root());
    assert_ne!(c1.tree_hash_root(), c3.tree_hash_root());
    assert_ne!(c2.tree_hash_root(), c3.tree_hash_root());
}

#[test]
fn test_nested_container_tree_hash() {
    let nested_zeros = NestedContainer {
        x: 0,
        inner: SimpleContainer { a: 0, b: false },
    };

    let nested_ones = NestedContainer {
        x: 1,
        inner: SimpleContainer { a: 1, b: true },
    };

    // Nested containers should produce valid hashes
    let hash_zeros = nested_zeros.tree_hash_root();
    let hash_ones = nested_ones.tree_hash_root();

    assert_ne!(
        hash_zeros, hash_ones,
        "Different nested containers should have different hashes"
    );

    // Changing outer field should change hash
    let nested_outer_changed = NestedContainer {
        x: 42,
        inner: SimpleContainer { a: 0, b: false },
    };
    assert_ne!(
        hash_zeros,
        nested_outer_changed.tree_hash_root(),
        "Changing outer field should change hash"
    );

    // Changing inner field should change hash
    let nested_inner_changed = NestedContainer {
        x: 0,
        inner: SimpleContainer { a: 42, b: false },
    };
    assert_ne!(
        hash_zeros,
        nested_inner_changed.tree_hash_root(),
        "Changing inner field should change hash"
    );
}

#[test]
fn test_container_with_bytes32() {
    let container_zeros = ContainerWithBytes32 { data: [0u8; 32] };
    let container_ones = ContainerWithBytes32 { data: [0xFF; 32] };

    let hash_zeros = container_zeros.tree_hash_root();
    let hash_ones = container_ones.tree_hash_root();

    assert_ne!(
        hash_zeros, hash_ones,
        "Different Bytes32 values should produce different hashes"
    );
}

// =============================================================================
// Tests - Zero Hash Verification
// =============================================================================

#[test]
fn test_zero_hash_consistency() {
    // Verify that get_zero_hash(0) is all zeros
    let zero_hash_0 = Sha256Hasher::get_zero_hash(0);
    assert_eq!(
        zero_hash_0,
        Hash256::ZERO,
        "Zero hash at depth 0 should be 32 zero bytes"
    );

    let zero_hash_1 = Sha256Hasher::get_zero_hash(1);
    assert_eq!(
        zero_hash_1,
        Hash256::from_slice(&HASH_UNION_EMPTY),
        "Zero hash at depth 1 should be SHA256([0; 64])"
    );
}

// =============================================================================
// Tests - FixedVector with Values
// =============================================================================

#[test]
fn test_fixed_vector_with_values() {
    // Vector[uint64, 4](1, 2, 3, 4) - packs into single 32-byte chunk
    let vec: FixedVector<u64, 4> = FixedVector::from(vec![1u64, 2, 3, 4]);
    assert_eq!(
        <FixedVector<u64, 4> as TreeHash<Sha256Hasher>>::tree_hash_root(&vec),
        Hash256::from_slice(&HASH_FIXED_VECTOR_1234),
        "FixedVector[1,2,3,4] should pack to identity"
    );
}

// =============================================================================
// Tests - VariableList
// =============================================================================

#[test]
fn test_variable_list_tree_hash() {
    // Empty list: List[uint64, 8]()
    let empty: VariableList<u64, 8> = VariableList::empty();
    assert_eq!(
        <VariableList<u64, 8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty),
        Hash256::from_slice(&HASH_LIST_U64_EMPTY),
        "Empty List[uint64, 8] should match reference value"
    );

    // List with values: List[uint64, 8](1, 2, 3, 4)
    let with_values: VariableList<u64, 8> = VariableList::from(vec![1u64, 2, 3, 4]);
    assert_eq!(
        <VariableList<u64, 8> as TreeHash<Sha256Hasher>>::tree_hash_root(&with_values),
        Hash256::from_slice(&HASH_LIST_U64_1234),
        "List[uint64, 8](1,2,3,4) should match reference value"
    );
}

// =============================================================================
// Tests - BitVector
// =============================================================================

#[test]
fn test_bitvector_tree_hash() {
    // Bitvector[8] all zeros
    let zeros: BitVector<8> = BitVector::new();
    assert_eq!(
        <BitVector<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&zeros),
        Hash256::ZERO,
        "Bitvector[8] zeros should be all zeros"
    );

    // Bitvector[8] all true - set all 8 bits
    let mut all_true: BitVector<8> = BitVector::new();
    for i in 0..8 {
        all_true.set(i, true).expect("valid index");
    }
    assert_eq!(
        <BitVector<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&all_true),
        Hash256::from_slice(&HASH_BITVECTOR_8_ALL_TRUE),
        "tree_hash_root of Bitvector[8] all true"
    );
}

// =============================================================================
// Tests - BitList
// =============================================================================

#[test]
fn test_bitlist_tree_hash() {
    // Bitlist[8] empty (length 0)
    let empty: BitList<8> = BitList::with_capacity(0).expect("valid capacity");
    assert_eq!(
        <BitList<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty),
        Hash256::from_slice(&HASH_BITLIST_8_EMPTY),
        "Empty Bitlist[8] should match reference value"
    );

    // Bitlist[8] with 4 bits: True, False, True, False
    let mut with_bits: BitList<8> = BitList::with_capacity(4).expect("valid capacity");
    with_bits.set(0, true).expect("valid index"); // True
    with_bits.set(1, false).expect("valid index"); // False
    with_bits.set(2, true).expect("valid index"); // True
    with_bits.set(3, false).expect("valid index"); // False
    assert_eq!(
        <BitList<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&with_bits),
        Hash256::from_slice(&HASH_BITLIST_8_TFTT),
        "Bitlist[8](T,F,T,F) should match reference value"
    );
}

// =============================================================================
// Tests - Multi-variant Union
// =============================================================================

#[test]
fn test_multi_variant_union() {
    // Empty variant at selector 0
    assert_eq!(
        MultiUnion::Empty.tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_EMPTY),
        "MultiUnion::Empty (selector 0) should match HASH_UNION_EMPTY"
    );

    // U8(1) at selector 1
    assert_eq!(
        MultiUnion::U8(1).tree_hash_root(),
        Hash256::from_slice(&HASH_MULTI_UNION_U8_1),
        "MultiUnion::U8(1) (selector 1) should match reference value"
    );

    // U16(1000) at selector 2
    assert_eq!(
        MultiUnion::U16(1000).tree_hash_root(),
        Hash256::from_slice(&HASH_MULTI_UNION_U16_1000),
        "MultiUnion::U16(1000) (selector 2) should match reference value"
    );
}

// =============================================================================
// Tests - Hash256 Identity
// =============================================================================

#[test]
fn test_hash256_identity() {
    // Hash256 / Bytes32 should be its own root (32 bytes = identity)
    let value: [u8; 32] = [0xAB; 32];
    assert_eq!(
        <[u8; 32] as TreeHash<Sha256Hasher>>::tree_hash_root(&value),
        Hash256::from_slice(&value),
        "Bytes32([0xAB; 32]) should equal itself"
    );

    // Also verify Hash256 type specifically
    let h256 = Hash256::from_slice(&value);
    assert_eq!(
        <Hash256 as TreeHash<Sha256Hasher>>::tree_hash_root(&h256),
        h256,
        "Hash256 should be its own root"
    );
}

// =============================================================================
// Tests - Union Value(42) with hardcoded hash
// =============================================================================

#[test]
fn test_union_value_42() {
    // Union[None, uint64]::Value(42) at selector 1
    assert_eq!(
        UnionNoneU64::Value(42).tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_VALUE_42),
        "Union::Value(42) should match reference value"
    );
}

// =============================================================================
// Tests - Container with hardcoded hash
// =============================================================================

#[test]
fn test_container_with_hardcoded_hash() {
    let container = SimpleContainer { a: 1, b: true };
    assert_eq!(
        container.tree_hash_root(),
        Hash256::from_slice(&HASH_CONTAINER_ONES),
        "SimpleContainer(1, true) should match reference value"
    );
}

// =============================================================================
// Tests - Union Edge Cases
// =============================================================================

#[test]
fn test_union_max_u64() {
    // Union[None, u64]::Value(MAX) at selector 1
    assert_eq!(
        UnionNoneU64::Value(u64::MAX).tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_MAX_U64),
        "Union::Value(u64::MAX) should match reference value"
    );
}

#[test]
fn test_big_union_no_none_variant() {
    // BigUnion has no None variant - selector 0 is u8
    assert_eq!(
        BigUnion::U8(255).tree_hash_root(),
        Hash256::from_slice(&HASH_BIG_UNION_U8_255_SEL0),
        "BigUnion::U8(255) (selector 0) should match reference value"
    );

    assert_eq!(
        BigUnion::U16(65535).tree_hash_root(),
        Hash256::from_slice(&HASH_BIG_UNION_U16_MAX_SEL1),
        "BigUnion::U16(MAX) (selector 1) should match reference value"
    );

    assert_eq!(
        BigUnion::U32(u32::MAX).tree_hash_root(),
        Hash256::from_slice(&HASH_BIG_UNION_U32_MAX_SEL2),
        "BigUnion::U32(MAX) (selector 2) should match reference value"
    );

    assert_eq!(
        BigUnion::U64(0xDEADBEEFCAFEBABE).tree_hash_root(),
        Hash256::from_slice(&HASH_BIG_UNION_U64_DEADBEEF_SEL3),
        "BigUnion::U64(DEADBEEF) (selector 3) should match reference value"
    );
}

#[test]
fn test_union_with_container_variant() {
    // UnionEmptyData::Empty at selector 0
    assert_eq!(
        UnionEmptyData::Empty.tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_EMPTY),
        "UnionEmptyData::Empty should equal HASH_UNION_EMPTY"
    );

    assert_eq!(
        UnionEmptyData::Data(DataVariant { value: 42 }).tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_CONTAINER_42),
        "tree_hash_root of UnionEmptyData::Data(42)"
    );
}

#[test]
fn test_container_with_union_field() {
    // Container containing union set to Empty
    let c_empty = ContainerWithUnion {
        state: UnionNoneU64::Empty,
    };
    assert_eq!(
        c_empty.tree_hash_root(),
        Hash256::from_slice(&HASH_UNION_EMPTY),
        "tree_hash_root of ContainerWithUnion(Empty)"
    );

    // Container containing union set to Value(100)
    let c_value = ContainerWithUnion {
        state: UnionNoneU64::Value(100),
    };
    assert_eq!(
        c_value.tree_hash_root(),
        Hash256::from_slice(&HASH_CONTAINER_UNION_100),
        "tree_hash_root of ContainerWithUnion(Value(100))"
    );
}

#[test]
fn test_multi_union_u64_at_selector_3() {
    // MultiUnion::U64(1000000) at selector 3
    assert_eq!(
        MultiUnion::U64(1000000).tree_hash_root(),
        Hash256::from_slice(&HASH_MULTI_UNION_U64_1000000),
        "MultiUnion::U64(1000000) (selector 3) should match reference value"
    );
}

#[test]
fn test_data_variant_container_identity() {
    // DataVariant has a single u64 field, so its tree hash should be identity
    let dv = DataVariant { value: 42 };
    assert_eq!(
        dv.tree_hash_root(),
        Hash256::from_slice(&HASH_DATA_VARIANT_42),
        "tree_hash_root of DataVariant(value=42)"
    );
}
