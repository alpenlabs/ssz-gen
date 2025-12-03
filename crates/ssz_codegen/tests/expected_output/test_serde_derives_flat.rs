pub mod test_serde_derives {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    use ssz_types::*;
    use ssz_types::view::{FixedVectorRef, VariableListRef};
    use ssz_primitives::{U128, U256};
    use ssz_derive::{Encode, Decode};
    use tree_hash::TreeHashDigest;
    use tree_hash_derive::TreeHash;
    use ssz::view::*;
    use serde::{Serialize, Deserialize};
    pub type Slot = u64;
    /// Test that serde derives add proper imports (mirrors identifiers use case)
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Copy,
        Hash,
        Serialize,
        Deserialize,
        Encode,
        Decode
    )]
    #[ssz(struct_behaviour = "container")]
    pub struct BlockCommitment {
        /// Slot number
        pub slot: Slot,
        /// Block ID
        pub blkid: U256,
    }
    impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for BlockCommitment {
        fn tree_hash_type() -> tree_hash::TreeHashType {
            tree_hash::TreeHashType::Container
        }
        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_packing_factor() -> usize {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_root(&self) -> H::Output {
            use tree_hash::TreeHash;
            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
            hasher
                .write(
                    <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.slot).as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher
                .write(
                    <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.blkid).as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher.finish().expect("tree hash derive should not have a remaining buffer")
        }
    }
    /// Zero-copy view over [`BlockCommitment`].
    ///
    /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
    /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
    /// needed.
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    #[derive(Clone, Debug, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
    pub struct BlockCommitmentRef<'a> {
        bytes: &'a [u8],
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> BlockCommitmentRef<'a> {
        pub fn slot(&self) -> Result<u64, ssz::DecodeError> {
            let offset = 0usize;
            let end = offset + 8usize;
            if end > self.bytes.len() {
                return Err(ssz::DecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: end,
                });
            }
            let bytes = &self.bytes[offset..end];
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
        pub fn blkid(&self) -> Result<U256, ssz::DecodeError> {
            let offset = 8usize;
            let end = offset + 32usize;
            if end > self.bytes.len() {
                return Err(ssz::DecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: end,
                });
            }
            let bytes = &self.bytes[offset..end];
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
    }
    impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
    for BlockCommitmentRef<'a> {
        fn tree_hash_type() -> tree_hash::TreeHashType {
            tree_hash::TreeHashType::Container
        }
        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_packing_factor() -> usize {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_root(&self) -> H::Output {
            use tree_hash::TreeHash;
            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(0);
            {
                let offset = 0usize;
                let field_bytes = &self.bytes[offset..offset + 8usize];
                hasher.write(field_bytes).expect("write field");
            }
            {
                let offset = 8usize;
                let field_bytes = &self.bytes[offset..offset + 32usize];
                hasher.write(field_bytes).expect("write field");
            }
            hasher.finish().expect("finish hasher")
        }
    }
    impl<'a> ssz::view::DecodeView<'a> for BlockCommitmentRef<'a> {
        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
            if bytes.len() != 40usize {
                return Err(ssz::DecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: 40usize,
                });
            }
            Ok(Self { bytes })
        }
    }
    impl<'a> ssz::view::SszTypeInfo for BlockCommitmentRef<'a> {
        fn is_ssz_fixed_len() -> bool {
            true
        }
        fn ssz_fixed_len() -> usize {
            40usize
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> ssz_types::view::ToOwnedSsz<BlockCommitment> for BlockCommitmentRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        fn to_owned(&self) -> BlockCommitment {
            <BlockCommitmentRef<'a>>::to_owned(self)
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> BlockCommitmentRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        pub fn to_owned(&self) -> BlockCommitment {
            BlockCommitment {
                slot: self.slot().expect("valid view"),
                blkid: self.blkid().expect("valid view"),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
    #[ssz(struct_behaviour = "container")]
    pub struct OtherType {
        pub value: u64,
    }
    impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for OtherType {
        fn tree_hash_type() -> tree_hash::TreeHashType {
            tree_hash::TreeHashType::Container
        }
        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_packing_factor() -> usize {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_root(&self) -> H::Output {
            use tree_hash::TreeHash;
            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
            hasher
                .write(
                    <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.value).as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher.finish().expect("tree hash derive should not have a remaining buffer")
        }
    }
    /// Zero-copy view over [`OtherType`].
    ///
    /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
    /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
    /// needed.
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct OtherTypeRef<'a> {
        bytes: &'a [u8],
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> OtherTypeRef<'a> {
        pub fn value(&self) -> Result<u64, ssz::DecodeError> {
            let offset = 0usize;
            let end = offset + 8usize;
            if end > self.bytes.len() {
                return Err(ssz::DecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: end,
                });
            }
            let bytes = &self.bytes[offset..end];
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
    }
    impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for OtherTypeRef<'a> {
        fn tree_hash_type() -> tree_hash::TreeHashType {
            tree_hash::TreeHashType::Container
        }
        fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_packing_factor() -> usize {
            unreachable!("Container should never be packed")
        }
        fn tree_hash_root(&self) -> H::Output {
            use tree_hash::TreeHash;
            let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(0);
            {
                let offset = 0usize;
                let field_bytes = &self.bytes[offset..offset + 8usize];
                hasher.write(field_bytes).expect("write field");
            }
            hasher.finish().expect("finish hasher")
        }
    }
    impl<'a> ssz::view::DecodeView<'a> for OtherTypeRef<'a> {
        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
            if bytes.len() != 8usize {
                return Err(ssz::DecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: 8usize,
                });
            }
            Ok(Self { bytes })
        }
    }
    impl<'a> ssz::view::SszTypeInfo for OtherTypeRef<'a> {
        fn is_ssz_fixed_len() -> bool {
            true
        }
        fn ssz_fixed_len() -> usize {
            8usize
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> ssz_types::view::ToOwnedSsz<OtherType> for OtherTypeRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        fn to_owned(&self) -> OtherType {
            <OtherTypeRef<'a>>::to_owned(self)
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> OtherTypeRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        pub fn to_owned(&self) -> OtherType {
            OtherType {
                value: self.value().expect("valid view"),
            }
        }
    }
}
