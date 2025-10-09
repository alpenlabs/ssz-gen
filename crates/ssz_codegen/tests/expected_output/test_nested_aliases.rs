pub mod tests {
    pub mod input {
        pub mod test_nested_aliases {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            pub const SIZE_1: u64 = 10u64;
            pub const SIZE_2: u64 = 10u64;
            pub const SIZE_3: u64 = 10u64;
            pub type A = u8;
            pub type B = A;
            pub type C = B;
            pub type D = VariableList<C, 10usize>;
            pub type E = FixedVector<D, 5usize>;
            pub type F = VariableList<A, 10usize>;
            pub type G = FixedVector<F, 10usize>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct NestedAliasContainer {
                pub field1: D,
                pub field2: E,
                pub field3: F,
                pub field4: G,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct NestedAliasContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> NestedAliasContainerRef<'a> {
                pub fn field1(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        4usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        4usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field2(
                    &self,
                ) -> Result<FixedVectorRef<'a, BytesRef<'a>, 5usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        4usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        4usize,
                        1usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field3(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        4usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        4usize,
                        2usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field4(
                    &self,
                ) -> Result<
                    FixedVectorRef<'a, BytesRef<'a>, 10usize>,
                    ssz::DecodeError,
                > {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        4usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        4usize,
                        3usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for NestedAliasContainerRef<'a> {
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
                        let field1 = self.field1().expect("valid view");
                        hasher
                            .write(field1.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let field2 = self.field2().expect("valid view");
                        hasher
                            .write(field2.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let field3 = self.field3().expect("valid view");
                        hasher
                            .write(field3.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let field4 = self.field4().expect("valid view");
                        hasher
                            .write(field4.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for NestedAliasContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 16usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 16usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..4usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            16usize,
                            4usize,
                            i,
                        )?;
                        if i == 0 && offset != 16usize {
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
            impl<'a> NestedAliasContainerRef<'a> {
                pub fn to_owned(&self) -> NestedAliasContainer {
                    NestedAliasContainer {
                        field1: self.field1().expect("valid view").to_owned(),
                        field2: self.field2().expect("valid view").to_owned(),
                        field3: self.field3().expect("valid view").to_owned(),
                        field4: self.field4().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
