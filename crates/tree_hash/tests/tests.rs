// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Tree hash tests

use rand as _;
use smallvec as _;
use ssz_derive::Encode;
use ssz_primitives::{U128, U256};
use ssz_types::{BitVector, Optional, VariableList};
use tree_hash::{self, BYTES_PER_CHUNK, Hash256, MerkleHasher, PackedEncoding, TreeHash};
use tree_hash_derive::TreeHash;

#[derive(Encode)]
struct HashVec {
    vec: Vec<u8>,
}

impl From<Vec<u8>> for HashVec {
    fn from(vec: Vec<u8>) -> Self {
        Self { vec }
    }
}

impl tree_hash::TreeHash for HashVec {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        tree_hash::TreeHashType::List
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_root(&self) -> Hash256 {
        let mut hasher = MerkleHasher::with_leaves(self.vec.len().div_ceil(BYTES_PER_CHUNK));

        for item in &self.vec {
            hasher.write(&item.tree_hash_packed_encoding()).unwrap()
        }

        let root = hasher.finish().unwrap();

        tree_hash::mix_in_length(&root, self.vec.len())
    }
}

fn mix_in_selector(a: Hash256, selector: u8) -> Hash256 {
    let mut b = [0; 32];
    b[0] = selector;

    Hash256::from_slice(&ethereum_hashing::hash32_concat(a.as_slice(), &b))
}

fn u8_hash_concat(v1: u8, v2: u8) -> Hash256 {
    let mut a = [0; 32];
    let mut b = [0; 32];

    a[0] = v1;
    b[0] = v2;

    Hash256::from_slice(&ethereum_hashing::hash32_concat(&a, &b))
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
    let canonical = U256::from(val).tree_hash_packed_encoding();
    let encodings = [
        (0x8880_u16.tree_hash_packed_encoding(), 0),
        (0x9990_u16.tree_hash_packed_encoding(), 2),
        (0xaaa0_u16.tree_hash_packed_encoding(), 4),
        (0xbbb0_u16.tree_hash_packed_encoding(), 6),
        (0xccc0_u16.tree_hash_packed_encoding(), 8),
        (0xddd0_u16.tree_hash_packed_encoding(), 10),
        (0xeee0_u16.tree_hash_packed_encoding(), 12),
        (0xfff0_u16.tree_hash_packed_encoding(), 14),
        (U128::from(val).tree_hash_packed_encoding(), 0),
        (U128::from(0).tree_hash_packed_encoding(), 16),
        (
            Hash256::from_slice(U256::from(val).as_le_slice())
                .tree_hash_root()
                .0
                .into(),
            0,
        ),
        (U256::from(val).tree_hash_root().0.into(), 0),
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
            0xbf, 0xdb, 0x6f, 0xda, 0x9d, 0x02, 0x80, 0x5e, 0x64, 0x0c, 0x0f, 0x57, 0x67, 0xb8,
            0xd1, 0xbb, 0x9f, 0xf4, 0x21, 0x14, 0x98, 0xa5, 0xe2, 0xd7, 0xc0, 0xf3, 0x6e, 0x1b,
            0x88, 0xce, 0x57, 0xff,
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
            0xf6, 0x6d, 0x2c, 0x38, 0xc8, 0xd2, 0xaf, 0xbd, 0x40, 0x9e, 0x86, 0xc5, 0x29, 0xdf,
            0xf7, 0x28, 0xe9, 0xa4, 0x20, 0x82, 0x15, 0xca, 0x20, 0xee, 0x44, 0xe4, 0x9c, 0x3d,
            0x11, 0xe1, 0x45, 0xd8,
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
            0x07, 0x92, 0xfb, 0x50, 0x93, 0x77, 0xee, 0x2f, 0xf3, 0xb9, 0x53, 0xdd, 0x9a, 0x88,
            0xee, 0xe1, 0x1a, 0xc7, 0x56, 0x6a, 0x8d, 0xf4, 0x1c, 0x6c, 0x67, 0xa8, 0x5b, 0xc0,
            0xb5, 0x3e, 0xfa, 0x4e,
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
            0xdd, 0xc7, 0xac, 0xd3, 0x8a, 0xe9, 0xd6, 0xd6, 0x78, 0x8c, 0x14, 0xbd, 0x76, 0x35,
            0xae, 0xb1, 0xd7, 0x69, 0x47, 0x68, 0xd7, 0xe0, 0x0e, 0x17, 0x95, 0xbb, 0x6d, 0x32,
            0x8e, 0xc1, 0x4f, 0x28,
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
            0x98, 0x93, 0xec, 0xf9, 0xb6, 0x80, 0x30, 0xff, 0x23, 0xc6, 0x67, 0xa5, 0xf2, 0xe4,
            0xa7, 0x65, 0x38, 0xa8, 0xe2, 0xab, 0x48, 0xfd, 0x06, 0x0a, 0x52, 0x48, 0x88, 0xa6,
            0x6f, 0xb9, 0x38, 0xc9,
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
            0xe8, 0x23, 0x47, 0x13, 0x10, 0x31, 0x2d, 0x52, 0xaa, 0x11, 0x35, 0xd9, 0x71, 0xa3,
            0xed, 0x72, 0xba, 0x04, 0x1a, 0xde, 0x3e, 0xc5, 0xb5, 0x07, 0x7c, 0x17, 0xa3, 0x9d,
            0x73, 0xab, 0x17, 0xc5,
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
            0x10, 0x93, 0xb0, 0xf1, 0xd8, 0x8b, 0x1b, 0x2b, 0x45, 0x81, 0x96, 0xfa, 0x86, 0x0e,
            0x0d, 0xf7, 0xa7, 0xdc, 0x18, 0x37, 0xfe, 0x80, 0x4b, 0x95, 0xd6, 0x64, 0x27, 0x96,
            0x35, 0xcb, 0x30, 0x2f,
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
            0x28, 0xdf, 0x3f, 0x1c, 0x3e, 0xeb, 0xd9, 0x25, 0x04, 0x40, 0x1b, 0x15, 0x5c, 0x5c,
            0xfe, 0x2f, 0x01, 0xc0, 0x60, 0x48, 0x89, 0xe4, 0x6e, 0xd3, 0xd2, 0x2a, 0x30, 0x91,
            0xdd, 0xe1, 0x37, 0x1f,
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
            0x65, 0x96, 0x38, 0x36, 0x84, 0x67, 0xb2, 0xc0, 0x52, 0xca, 0x69, 0x8f, 0xcb, 0x65,
            0x90, 0x2e, 0x9b, 0x42, 0xce, 0x8e, 0x94, 0xe1, 0xf7, 0x94, 0xdd, 0x52, 0x96, 0xce,
            0xac, 0x2d, 0xec, 0x3e,
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
            0xd5, 0x85, 0xdd, 0x05, 0x61, 0xc7, 0x18, 0xbf, 0x4c, 0x29, 0xe4, 0xc1, 0xbd, 0x7d,
            0x4e, 0xfd, 0x4a, 0x5f, 0xe3, 0xc4, 0x59, 0x42, 0xa7, 0xf7, 0x78, 0xac, 0xb7, 0x8f,
            0xd0, 0xb2, 0xa4, 0xd2,
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
            0x00, 0xfc, 0x0c, 0xec, 0xc2, 0x00, 0xa4, 0x15, 0xa0, 0x73, 0x72, 0xd5, 0xd5, 0xb8,
            0xbc, 0x7c, 0xe4, 0x9f, 0x52, 0x50, 0x4e, 0xd3, 0xda, 0x03, 0x36, 0xf8, 0x0a, 0x26,
            0xd8, 0x11, 0xc7, 0xbf,
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
