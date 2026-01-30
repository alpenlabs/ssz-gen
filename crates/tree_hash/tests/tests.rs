// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Tree hash tests

use digest as _;
use rand as _;
use sha2 as _;
use smallvec as _;
use ssz_derive::Encode;
use ssz_primitives::{U128, U256};
use ssz_types::{BitVector, Optional, VariableList};
use thiserror as _;
use tree_hash::{
    self, BYTES_PER_CHUNK, Hash256, MerkleHasher, PackedEncoding, Sha256Hasher, TreeHash,
    TreeHashDigest, hash32_concat,
};
use tree_hash_derive::TreeHash;

#[derive(Encode)]
struct HashVec {
    vec: Vec<u8>,
}

#[derive(Encode)]
struct MacroList(VariableList<u8, 64>);

tree_hash::tree_hash_ssz_encoding_as_list!(MacroList, 64);

impl From<Vec<u8>> for HashVec {
    fn from(vec: Vec<u8>) -> Self {
        Self { vec }
    }
}

impl tree_hash::TreeHash<tree_hash::Sha256Hasher> for HashVec {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        tree_hash::TreeHashType::List
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_root(&self) -> <tree_hash::Sha256Hasher as tree_hash::TreeHashDigest>::Output {
        let mut hasher = MerkleHasher::<tree_hash::Sha256Hasher>::with_leaves(
            self.vec.len().div_ceil(BYTES_PER_CHUNK),
        );

        for item in &self.vec {
            hasher
                .write(&<u8 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(item))
                .unwrap()
        }

        let root = hasher.finish().unwrap();

        tree_hash::mix_in_length_with_hasher::<tree_hash::Sha256Hasher>(&root, self.vec.len())
    }
}

#[test]
fn macro_list_tree_hash_matches_variable_list() {
    let list: VariableList<u8, 64> = vec![0x42].into();
    let macro_list = MacroList(list.clone());

    let list_root = TreeHash::<Sha256Hasher>::tree_hash_root(&list);
    let macro_root = TreeHash::<Sha256Hasher>::tree_hash_root(&macro_list);

    assert_eq!(macro_root, list_root);
}

fn mix_in_selector(a: Hash256, selector: u8) -> Hash256 {
    let mut b = [0; 32];
    b[0] = selector;

    Hash256::from_slice(&hash32_concat::<sha2::Sha256>(a.as_slice(), &b))
}

fn u8_hash_concat(v1: u8, v2: u8) -> Hash256 {
    let mut a = [0; 32];
    let mut b = [0; 32];

    a[0] = v1;
    b[0] = v2;

    Hash256::from_slice(&hash32_concat::<sha2::Sha256>(&a, &b))
}

fn u8_hash(x: u8) -> Hash256 {
    let mut a = [0; 32];
    a[0] = x;
    Hash256::from_slice(&a)
}

#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "transparent")]
enum FixedTrans {
    A(u8),
    B(u8),
}

#[test]
fn fixed_trans() {
    assert_eq!(FixedTrans::A(2).tree_hash_root(), u8_hash(2));
    assert_eq!(FixedTrans::B(2).tree_hash_root(), u8_hash(2));
}

#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "union")]
enum FixedUnion {
    A(u8),
    B(u8),
}

#[test]
fn fixed_union() {
    assert_eq!(FixedUnion::A(2).tree_hash_root(), u8_hash_concat(2, 0));
    assert_eq!(FixedUnion::B(2).tree_hash_root(), u8_hash_concat(2, 1));
}

#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "transparent")]
enum VariableTrans {
    A(HashVec),
    B(HashVec),
}

#[test]
fn variable_trans() {
    assert_eq!(
        VariableTrans::A(HashVec::from(vec![2])).tree_hash_root(),
        u8_hash_concat(2, 1)
    );
    assert_eq!(
        VariableTrans::B(HashVec::from(vec![2])).tree_hash_root(),
        u8_hash_concat(2, 1)
    );
}

#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "union")]
enum VariableUnion {
    A(HashVec),
    B(HashVec),
}

#[test]
fn variable_union() {
    assert_eq!(
        VariableUnion::A(HashVec::from(vec![2])).tree_hash_root(),
        mix_in_selector(u8_hash_concat(2, 1), 0)
    );
    assert_eq!(
        VariableUnion::B(HashVec::from(vec![2])).tree_hash_root(),
        mix_in_selector(u8_hash_concat(2, 1), 1)
    );
}

/// Test that the packed encodings for different types are equal.
#[test]
fn packed_encoding_example() {
    let val = 0xfff0eee0ddd0ccc0bbb0aaa099908880_u128;
    let canonical =
        <U256 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&U256::from(val));
    let encodings = [
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0x8880_u16),
            0,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0x9990_u16),
            2,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xaaa0_u16),
            4,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xbbb0_u16),
            6,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xccc0_u16),
            8,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xddd0_u16),
            10,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xeee0_u16),
            12,
        ),
        (
            <u16 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&0xfff0_u16),
            14,
        ),
        (
            <U128 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&U128::from(
                val,
            )),
            0,
        ),
        (
            <U128 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_packed_encoding(&U128::from(0)),
            16,
        ),
        (
            <Hash256 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_root(&Hash256::from_slice(
                U256::from(val).as_le_slice(),
            ))
            .0
            .into(),
            0,
        ),
        (
            <U256 as TreeHash<tree_hash::Sha256Hasher>>::tree_hash_root(&U256::from(val))
                .0
                .into(),
            0,
        ),
    ];
    for (i, (encoding, offset)) in encodings.into_iter().enumerate() {
        assert_eq!(
            &encoding[..],
            &canonical[offset..offset + encoding.len()],
            "encoding {i} is wrong"
        );
    }
}

#[derive(TreeHash)]
#[tree_hash(struct_behaviour = "stable_container")]
#[tree_hash(max_fields = 4)]
struct Shape1 {
    side: Optional<u16>,
    color: Optional<u8>,
    radius: Optional<u16>,
}

#[derive(TreeHash)]
#[tree_hash(struct_behaviour = "stable_container")]
#[tree_hash(max_fields = 8)]
struct Shape2 {
    side: Optional<u16>,
    color: Optional<u8>,
    radius: Optional<u16>,
}

#[derive(TreeHash)]
#[tree_hash(struct_behaviour = "stable_container")]
#[tree_hash(max_fields = 8)]
struct Shape3 {
    side: Optional<u16>,
    colors: Optional<VariableList<u8, 4>>,
    radius: Optional<u16>,
}

#[derive(TreeHash, Clone)]
#[tree_hash(struct_behaviour = "profile")]
#[tree_hash(max_fields = 4)]
struct Square {
    // We always start with a stable_index of 0.
    side: u16,
    color: u8,
}

#[derive(TreeHash, Clone)]
#[tree_hash(struct_behaviour = "profile")]
#[tree_hash(max_fields = 4)]
struct Circle {
    #[tree_hash(stable_index = 1)]
    color: u8,
    #[tree_hash(skip_hashing)]
    _phantom: u8,
    // Note that we do not need to specify `stable_index = 2` here since
    // we always increment by 1 from the previous index.
    radius: u16,
}

#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "transparent_stable")]
enum ShapeEnum {
    SquareVariant(Square),
    CircleVariant(Circle),
}

// Values for the tests below are based on:
// https://github.com/ethereum/EIPs/blob/master/assets/eip-7495/tests.py

#[test]
fn shape_1() {
    let shape_1 = Shape1 {
        side: Optional::Some(0x42),
        color: Optional::Some(1),
        radius: Optional::None,
    };

    let square = Square {
        side: 0x42,
        color: 1,
    };

    assert_eq!(shape_1.tree_hash_root(), square.tree_hash_root());
    assert_eq!(
        shape_1.tree_hash_root(),
        Hash256::from_slice(&[
            0xe3, 0x2b, 0x6b, 0x00, 0x9c, 0x15, 0x6a, 0xaa, 0x25, 0x2a, 0xff, 0x6d, 0x30, 0x2d,
            0xf2, 0xff, 0x8d, 0x99, 0xfc, 0x03, 0x48, 0x83, 0xc6, 0x5d, 0x20, 0xa9, 0x02, 0x18,
            0x71, 0x1e, 0x6a, 0x05,
        ])
    );

    let shape_1 = Shape1 {
        side: Optional::None,
        color: Optional::Some(1),
        radius: Optional::Some(0x42),
    };

    let circle = Circle {
        color: 1,
        _phantom: 6,
        radius: 0x42,
    };
    assert_eq!(shape_1.tree_hash_root(), circle.tree_hash_root());

    assert_eq!(
        shape_1.tree_hash_root(),
        Hash256::from_slice(&[
            0x3b, 0x60, 0x25, 0xa9, 0x26, 0x55, 0x52, 0x15, 0x1b, 0x65, 0x4a, 0xab, 0x0e, 0x0e,
            0xfa, 0x88, 0x53, 0x2f, 0x9e, 0xc9, 0x96, 0xfd, 0x9e, 0xd4, 0x4f, 0xd0, 0x0d, 0xc5,
            0x74, 0xd5, 0x80, 0xb2,
        ])
    );
}

#[test]
fn shape_2() {
    let shape_2 = Shape2 {
        side: Optional::Some(0x42),
        color: Optional::Some(1),
        radius: Optional::Some(0x42),
    };

    assert_eq!(
        shape_2.tree_hash_root(),
        Hash256::from_slice(&[
            0x12, 0x8a, 0x1c, 0xfa, 0xe7, 0x0b, 0x12, 0x8f, 0xa8, 0x16, 0x3b, 0x96, 0xbb, 0x8e,
            0x7a, 0xdd, 0x14, 0xb0, 0xd3, 0x23, 0xc2, 0xae, 0xc0, 0x09, 0x0d, 0x8f, 0x9b, 0x92,
            0x40, 0x65, 0xc4, 0x91,
        ])
    );

    let shape_2 = Shape2 {
        side: Optional::Some(0x42),
        color: Optional::Some(1),
        radius: Optional::None,
    };

    assert_eq!(
        shape_2.tree_hash_root(),
        Hash256::from_slice(&[
            0xcc, 0x18, 0x7a, 0xb5, 0xa9, 0xd6, 0xe5, 0x63, 0x15, 0xfc, 0xac, 0x08, 0x7b, 0x82,
            0xae, 0xf1, 0x58, 0x8d, 0x61, 0x8e, 0x30, 0x02, 0xc9, 0x71, 0x82, 0x8f, 0x85, 0x77,
            0x1f, 0xad, 0xec, 0xad,
        ])
    );

    let shape_2 = Shape2 {
        side: Optional::None,
        color: Optional::Some(1),
        radius: Optional::None,
    };

    assert_eq!(
        shape_2.tree_hash_root(),
        Hash256::from_slice(&[
            0xcc, 0xcf, 0x6a, 0x65, 0xda, 0xaa, 0x9e, 0xa0, 0xcd, 0x0d, 0x5b, 0x48, 0xd3, 0x86,
            0x30, 0x98, 0x42, 0xc3, 0xad, 0x4f, 0xce, 0xd3, 0x31, 0xb8, 0xba, 0x8a, 0xb3, 0x9a,
            0x27, 0xf9, 0xea, 0x10,
        ])
    );

    let shape_2 = Shape2 {
        side: Optional::None,
        color: Optional::Some(1),
        radius: Optional::Some(0x42),
    };

    assert_eq!(
        shape_2.tree_hash_root(),
        Hash256::from_slice(&[
            0x7d, 0xc3, 0xb7, 0x17, 0x8c, 0x39, 0xd3, 0x2a, 0x2a, 0x5d, 0xf9, 0x7c, 0x6e, 0xbb,
            0x9b, 0x1c, 0x2f, 0x87, 0x76, 0x13, 0x3f, 0xc0, 0xee, 0xf5, 0xc8, 0xe3, 0xe7, 0xaa,
            0x78, 0xf7, 0xaf, 0x61,
        ])
    );
}

#[test]
fn shape_3() {
    let shape_3 = Shape3 {
        side: Optional::Some(0x42),
        colors: Optional::Some(VariableList::from(vec![1, 2])),
        radius: Optional::Some(0x42),
    };

    assert_eq!(
        shape_3.tree_hash_root(),
        Hash256::from_slice(&[
            0xb3, 0xb9, 0xd2, 0xab, 0xed, 0x1b, 0x43, 0x91, 0xe8, 0x36, 0xc6, 0xc2, 0x8a, 0x15,
            0xff, 0x9a, 0x86, 0xb5, 0x35, 0x49, 0x5d, 0x4d, 0x6c, 0x2f, 0x09, 0xa3, 0xc1, 0x1c,
            0xd9, 0xb3, 0x94, 0x2c,
        ])
    );

    let shape_3 = Shape3 {
        side: Optional::Some(0x42),
        colors: Optional::None,
        radius: Optional::None,
    };

    assert_eq!(
        shape_3.tree_hash_root(),
        Hash256::from_slice(&[
            0x66, 0xe4, 0x22, 0x64, 0x7d, 0x98, 0xb8, 0x1d, 0xae, 0xc1, 0xa5, 0x7c, 0x50, 0x65,
            0x9b, 0x6e, 0x44, 0xfc, 0xdf, 0x46, 0x41, 0x79, 0xdd, 0x00, 0x27, 0x28, 0x23, 0x23,
            0x08, 0x6b, 0xad, 0xac,
        ])
    );

    let shape_3 = Shape3 {
        side: Optional::None,
        colors: Optional::Some(VariableList::from(vec![1, 2])),
        radius: Optional::None,
    };

    assert_eq!(
        shape_3.tree_hash_root(),
        Hash256::from_slice(&[
            0xa3, 0x13, 0x32, 0xa7, 0x00, 0x8d, 0xa6, 0x45, 0xb6, 0xa0, 0x5e, 0xff, 0xc0, 0x14,
            0xdd, 0x30, 0x62, 0xe7, 0xe7, 0x10, 0x4b, 0x42, 0xe5, 0x43, 0xc8, 0x99, 0xd4, 0x3d,
            0x43, 0x5b, 0x63, 0x10,
        ])
    );

    let shape_3 = Shape3 {
        side: Optional::None,
        colors: Optional::None,
        radius: Optional::Some(0x42),
    };

    assert_eq!(
        shape_3.tree_hash_root(),
        Hash256::from_slice(&[
            0xed, 0x0f, 0x91, 0xb8, 0x1a, 0xa7, 0x4a, 0x0a, 0x88, 0x0e, 0x7b, 0x60, 0x0d, 0x94,
            0x18, 0x6a, 0x91, 0xff, 0x2c, 0xe5, 0x1c, 0x8a, 0x77, 0xbd, 0x05, 0xa9, 0x90, 0xcb,
            0x0e, 0xf6, 0x82, 0x12,
        ])
    );

    let shape_3 = Shape3 {
        side: Optional::None,
        colors: Optional::Some(VariableList::from(vec![1, 2])),
        radius: Optional::Some(0x42),
    };

    assert_eq!(
        shape_3.tree_hash_root(),
        Hash256::from_slice(&[
            0xbf, 0xd7, 0x3d, 0xdf, 0x3f, 0x40, 0xc9, 0xc7, 0x86, 0xe5, 0xd5, 0x62, 0x42, 0x31,
            0x81, 0x75, 0x99, 0xbf, 0x2f, 0x9f, 0xe4, 0x74, 0x64, 0xd6, 0x4d, 0xe3, 0xf3, 0x45,
            0x5b, 0xed, 0xa9, 0xec,
        ])
    );
}

#[test]
fn shape_enum() {
    let square = Square { side: 16, color: 2 };

    let circle = Circle {
        color: 1,
        _phantom: 6,
        radius: 14,
    };

    let enum_square = ShapeEnum::SquareVariant(square.clone());
    let enum_circle = ShapeEnum::CircleVariant(circle.clone());

    assert_eq!(square.tree_hash_root(), enum_square.tree_hash_root());
    assert_eq!(circle.tree_hash_root(), enum_circle.tree_hash_root());
}

/// Container type for union variant data
#[derive(TreeHash, Clone)]
struct DataVariant1 {
    value: u64,
}

/// Union with empty variant at position 0 (common pattern for Option-like types)
#[derive(TreeHash)]
#[tree_hash(enum_behaviour = "union")]
enum UnionWithEmptyFirst {
    /// Empty variant at selector 0
    Empty,
    /// Data variant at selector 1
    Data(DataVariant1),
}

/// Test vector for empty union variant tree hash.
///
/// This test verifies the correct hash value for empty union variants.
/// The expected hash is: hash(zero_hash || selector_chunk)
/// where zero_hash = [0u8; 32] and selector_chunk = [selector, 0, 0, ..., 0]
#[test]
fn union_empty_variant_tree_hash_vector() {
    // Test 1: Empty variant at selector 0
    // root = zero_hash = [0u8; 32]
    // result = hash(root || [0, 0, ..., 0]) = hash([0u8; 64])
    let empty_first = UnionWithEmptyFirst::Empty;
    let empty_first_hash = empty_first.tree_hash_root();

    // The expected hash is SHA256 of 64 zero bytes
    // Computed: SHA256([0u8; 64]) =
    // f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b
    let expected_empty_selector_0 = Hash256::from_slice(&[
        0xf5, 0xa5, 0xfd, 0x42, 0xd1, 0x6a, 0x20, 0x30, 0x27, 0x98, 0xef, 0x6e, 0xd3, 0x09, 0x97,
        0x9b, 0x43, 0x00, 0x3d, 0x23, 0x20, 0xd9, 0xf0, 0xe8, 0xea, 0x98, 0x31, 0xa9, 0x27, 0x59,
        0xfb, 0x4b,
    ]);

    assert_eq!(
        empty_first_hash, expected_empty_selector_0,
        "Empty variant at selector 0 should be SHA256([0u8; 64])"
    );

    // Verify the zero hash at depth 0 is indeed [0u8; 32]
    let zero_hash = Sha256Hasher::get_zero_hash(0);
    assert_eq!(
        zero_hash,
        Hash256::ZERO,
        "Zero hash at depth 0 should be 32 zero bytes"
    );

    // Test 2: Non-empty variant to verify it differs from empty
    let data_variant = UnionWithEmptyFirst::Data(DataVariant1 { value: 42 });
    let data_hash = data_variant.tree_hash_root();

    assert_ne!(
        data_hash, empty_first_hash,
        "Data variant hash should differ from empty variant"
    );
}
