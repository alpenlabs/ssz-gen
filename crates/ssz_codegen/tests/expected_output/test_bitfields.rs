pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_bitfields {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SMALL_SIZE: u64 = 1u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const MEDIUM_SIZE: u64 = 64u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const LARGE_SIZE: u64 = 256u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const POWER_OF_TWO: u64 = 128u64;
            pub type TinyBitlist = BitList<1usize>;
            pub type StandardBitlist = BitList<64usize>;
            pub type LargeBitlist = BitList<256usize>;
            pub type TinyBitvector = BitVector<1usize>;
            pub type StandardBitvector = BitVector<64usize>;
            pub type LargeBitvector = BitVector<128usize>;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct BitfieldContainer {
                pub tiny_list: TinyBitlist,
                pub std_list: StandardBitlist,
                pub large_list: LargeBitlist,
                pub tiny_vec: TinyBitvector,
                pub std_vec: StandardBitvector,
                pub large_vec: LargeBitvector,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for BitfieldContainer {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(6usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.tiny_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.std_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.large_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.tiny_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.std_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.large_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BitfieldContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct BitfieldContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitfieldContainerRef<'a> {
                pub fn tiny_list(
                    &self,
                ) -> Result<BitListRef<'a, 1usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        37usize,
                        3usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        37usize,
                        3usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn std_list(
                    &self,
                ) -> Result<BitListRef<'a, 64usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        37usize,
                        3usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        37usize,
                        3usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn large_list(
                    &self,
                ) -> Result<BitListRef<'a, 256usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        37usize,
                        3usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        37usize,
                        3usize,
                        3usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn tiny_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 1usize>, ssz::DecodeError> {
                    let offset = 12usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn std_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 64usize>, ssz::DecodeError> {
                    let offset = 13usize;
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
                pub fn large_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 128usize>, ssz::DecodeError> {
                    let offset = 21usize;
                    let end = offset + 16usize;
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
            for BitfieldContainerRef<'a> {
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
                        let tiny_list = self.tiny_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&tiny_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let std_list = self.std_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&std_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let large_list = self.large_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&large_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let tiny_vec = self.tiny_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&tiny_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let std_vec = self.std_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&std_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let large_vec = self.large_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&large_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BitfieldContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 37usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 37usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..3usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            37usize,
                            3usize,
                            i,
                        )?;
                        if i == 0 && offset != 37usize {
                            return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                        }
                        if let Some(prev) = prev_offset && offset < prev {
                            return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for BitfieldContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BitfieldContainer>
            for BitfieldContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BitfieldContainer {
                    <BitfieldContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitfieldContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BitfieldContainer {
                    BitfieldContainer {
                        tiny_list: self.tiny_list().expect("valid view").to_owned(),
                        std_list: self.std_list().expect("valid view").to_owned(),
                        large_list: self.large_list().expect("valid view").to_owned(),
                        tiny_vec: self.tiny_vec().expect("valid view").to_owned(),
                        std_vec: self.std_vec().expect("valid view").to_owned(),
                        large_vec: self.large_vec().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
