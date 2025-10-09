pub mod tests {
    pub mod input {
        pub mod test_bitfields {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            pub const SMALL_SIZE: u64 = 1u64;
            pub const MEDIUM_SIZE: u64 = 64u64;
            pub const LARGE_SIZE: u64 = 256u64;
            pub const POWER_OF_TWO: u64 = 128u64;
            pub type TinyBitlist = BitList<1usize>;
            pub type StandardBitlist = BitList<64usize>;
            pub type LargeBitlist = BitList<256usize>;
            pub type TinyBitvector = BitVector<1usize>;
            pub type StandardBitvector = BitVector<64usize>;
            pub type LargeBitvector = BitVector<128usize>;
            #[derive(Encode, Decode, TreeHash)]
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
            /**Zero-copy view over [`BitfieldContainer`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Debug, Copy, Clone)]
            pub struct BitfieldContainerRef<'a> {
                bytes: &'a [u8],
            }
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
                        0usize + 1,
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
                        1usize + 1,
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
                        2usize + 1,
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
                        hasher
                            .write(tiny_list.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let std_list = self.std_list().expect("valid view");
                        hasher
                            .write(std_list.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let large_list = self.large_list().expect("valid view");
                        hasher
                            .write(large_list.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let tiny_vec = self.tiny_vec().expect("valid view");
                        hasher
                            .write(tiny_vec.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let std_vec = self.std_vec().expect("valid view");
                        hasher
                            .write(std_vec.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let large_vec = self.large_vec().expect("valid view");
                        hasher
                            .write(large_vec.tree_hash_root().as_ref())
                            .expect("write field");
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
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> BitfieldContainerRef<'a> {
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
