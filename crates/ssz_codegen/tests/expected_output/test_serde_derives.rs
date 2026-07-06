pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_serde_derives {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            pub type Slot = u64;
            /// Test that serde derives use fully qualified paths (mirrors identifiers use
            /// case)
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                std::marker::Copy,
                std::hash::Hash,
                serde::Serialize,
                serde::Deserialize,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct BlockCommitment {
                /// Slot number
                pub slot: Slot,
                /// Block ID
                pub blkid: U256,
            }
            impl tree_hash::TreeHash for BlockCommitment {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.slot)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.blkid)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BlockCommitment`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                std::marker::Copy,
                std::hash::Hash,
                serde::Serialize,
                serde::Deserialize
            )]
            pub struct BlockCommitmentRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BlockCommitmentRef<'a> {
                pub fn slot(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <Slot as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <Slot as ssz::Encode>::ssz_fixed_len(),
                        <Slot as ssz::Encode>::ssz_fixed_len()
                            + <U256 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<Slot as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<U256 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn blkid(&self) -> Result<U256, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <U256 as ssz::Encode>::is_ssz_fixed_len(),
                        <Slot as ssz::Encode>::ssz_fixed_len(),
                        <U256 as ssz::Encode>::ssz_fixed_len(),
                        <Slot as ssz::Encode>::ssz_fixed_len()
                            + <U256 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<Slot as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<U256 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<Slot as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for BlockCommitmentRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    {
                        let slot = self.slot().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&slot);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let blkid = self.blkid().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&blkid);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BlockCommitmentRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <Slot as ssz::Encode>::ssz_fixed_len()
                        + <U256 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<Slot as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<U256 as ssz::Encode>::is_ssz_fixed_len());
                    if num_variable_fields == 0 {
                        if bytes.len() != fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                    } else {
                        if bytes.len() < fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                        let mut prev_offset: Option<usize> = None;
                        for i in 0..num_variable_fields {
                            let offset = ssz::layout::read_variable_offset(
                                bytes,
                                fixed_portion_size,
                                num_variable_fields,
                                i,
                            )?;
                            if i == 0 && offset != fixed_portion_size {
                                return Err(
                                    ssz::DecodeError::OffsetIntoFixedPortion(offset),
                                );
                            }
                            if let Some(prev) = prev_offset && offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                            if offset > bytes.len() {
                                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                            }
                            prev_offset = Some(offset);
                        }
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for BlockCommitmentRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<Slot as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<U256 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <Slot as ssz::Encode>::ssz_fixed_len()
                            + <U256 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BlockCommitment>
            for BlockCommitmentRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BlockCommitment {
                    <BlockCommitmentRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BlockCommitmentRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BlockCommitment {
                    BlockCommitment {
                        slot: self.slot().expect("valid view"),
                        blkid: self.blkid().expect("valid view"),
                    }
                }
            }
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct OtherType {
                pub value: u64,
            }
            impl tree_hash::TreeHash for OtherType {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.value)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`OtherType`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                std::marker::Copy
            )]
            pub struct OtherTypeRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> OtherTypeRef<'a> {
                pub fn value(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u64 as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <u64 as ssz::Encode>::ssz_fixed_len(),
                        <u64 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for OtherTypeRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    {
                        let value = self.value().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&value);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for OtherTypeRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <u64 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<u64 as ssz::Encode>::is_ssz_fixed_len(),
                    );
                    if num_variable_fields == 0 {
                        if bytes.len() != fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                    } else {
                        if bytes.len() < fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                        let mut prev_offset: Option<usize> = None;
                        for i in 0..num_variable_fields {
                            let offset = ssz::layout::read_variable_offset(
                                bytes,
                                fixed_portion_size,
                                num_variable_fields,
                                i,
                            )?;
                            if i == 0 && offset != fixed_portion_size {
                                return Err(
                                    ssz::DecodeError::OffsetIntoFixedPortion(offset),
                                );
                            }
                            if let Some(prev) = prev_offset && offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                            if offset > bytes.len() {
                                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                            }
                            prev_offset = Some(offset);
                        }
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for OtherTypeRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u64 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<OtherType> for OtherTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> OtherType {
                    <OtherTypeRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> OtherTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> OtherType {
                    OtherType {
                        value: self.value().expect("valid view"),
                    }
                }
            }
        }
    }
}
