pub mod tests {
    pub mod input {
        pub mod test_large_unions {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum BigUnion {
                Selector0(u8),
                Selector1(u16),
                Selector2(u32),
                Selector3(u64),
                Selector4(u128),
                Selector5(u256),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum BigUnionRef<'a> {
                Selector0(u8),
                Selector1(u16),
                Selector2(u32),
                Selector3(u64),
                Selector4(u128),
                Selector5(u256),
            }
            impl<'a> BigUnionRef<'a> {
                pub fn to_owned(&self) -> BigUnion {
                    match self {
                        BigUnionRef::Selector0(v) => BigUnion::Selector0(*v),
                        BigUnionRef::Selector1(v) => BigUnion::Selector1(*v),
                        BigUnionRef::Selector2(v) => BigUnion::Selector2(*v),
                        BigUnionRef::Selector3(v) => BigUnion::Selector3(*v),
                        BigUnionRef::Selector4(v) => BigUnion::Selector4(*v),
                        BigUnionRef::Selector5(v) => BigUnion::Selector5(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedUnion {
                Selector0(u8),
                Selector1(VariableList<u8, 5usize>),
                Selector2(FixedVector<u16, 3usize>),
                Selector3(BitVector<8usize>),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedUnionRef<'a> {
                Selector0(u8),
                Selector1(BytesRef<'a>),
                Selector2(FixedVectorRef<'a, u16, 3usize>),
                Selector3(BitVectorRef<'a, 8usize>),
            }
            impl<'a> MixedUnionRef<'a> {
                pub fn to_owned(&self) -> MixedUnion {
                    match self {
                        MixedUnionRef::Selector0(v) => MixedUnion::Selector0(*v),
                        MixedUnionRef::Selector1(v) => {
                            MixedUnion::Selector1(v.to_owned())
                        }
                        MixedUnionRef::Selector2(v) => {
                            MixedUnion::Selector2(v.to_owned())
                        }
                        MixedUnionRef::Selector3(v) => {
                            MixedUnion::Selector3(v.to_owned())
                        }
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SameTypeUnion {
                Selector0(u8),
                Selector1(u8),
                Selector2(u8),
                Selector3(u8),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SameTypeUnionRef<'a> {
                Selector0(u8),
                Selector1(u8),
                Selector2(u8),
                Selector3(u8),
            }
            impl<'a> SameTypeUnionRef<'a> {
                pub fn to_owned(&self) -> SameTypeUnion {
                    match self {
                        SameTypeUnionRef::Selector0(v) => SameTypeUnion::Selector0(*v),
                        SameTypeUnionRef::Selector1(v) => SameTypeUnion::Selector1(*v),
                        SameTypeUnionRef::Selector2(v) => SameTypeUnion::Selector2(*v),
                        SameTypeUnionRef::Selector3(v) => SameTypeUnion::Selector3(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ContainerWithBigUnions {
                pub big: BigUnion,
                pub same: SameTypeUnion,
                pub mixed: MixedUnion,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct ContainerWithBigUnionsRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ContainerWithBigUnionsRef<'a> {
                pub fn big(&self) -> Result<BigUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        12usize,
                        3usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        12usize,
                        3usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn same(&self) -> Result<SameTypeUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        12usize,
                        3usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        12usize,
                        3usize,
                        1usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn mixed(&self) -> Result<MixedUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        12usize,
                        3usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        12usize,
                        3usize,
                        2usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithBigUnionsRef<'a> {
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
                    let big = self.big().expect("valid view");
                    hasher.write(big.tree_hash_root().as_ref()).expect("write field");
                    let same = self.same().expect("valid view");
                    hasher.write(same.tree_hash_root().as_ref()).expect("write field");
                    let mixed = self.mixed().expect("valid view");
                    hasher.write(mixed.tree_hash_root().as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithBigUnionsRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 12usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 12usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..3usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            12usize,
                            3usize,
                            i,
                        )?;
                        if i == 0 && offset != 12usize {
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
            impl<'a> ContainerWithBigUnionsRef<'a> {
                pub fn to_owned(&self) -> ContainerWithBigUnions {
                    ContainerWithBigUnions {
                        big: self.big().expect("valid view").to_owned(),
                        same: self.same().expect("valid view").to_owned(),
                        mixed: self.mixed().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
