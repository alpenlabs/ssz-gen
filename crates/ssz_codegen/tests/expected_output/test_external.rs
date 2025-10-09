pub mod tests {
    pub mod input {
        pub mod test_external {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionA {
                Selector0,
                Selector1(external_ssz::A),
                Selector2(external_ssz::module_a::module_b::B),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionARef<'a> {
                Selector0,
                Selector1(external_ssz::A),
                Selector2(external_ssz::module_a::module_b::B),
            }
            impl<'a> ExternalUnionARef<'a> {
                pub fn to_owned(&self) -> ExternalUnionA {
                    match self {
                        ExternalUnionARef::Selector0 => ExternalUnionA::Selector0,
                        ExternalUnionARef::Selector1(v) => {
                            ExternalUnionA::Selector1(v.to_owned())
                        }
                        ExternalUnionARef::Selector2(v) => {
                            ExternalUnionA::Selector2(v.to_owned())
                        }
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionB {
                Selector0,
                Selector1(TestA),
                Selector2(TestB),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionBRef<'a> {
                Selector0,
                Selector1(TestA),
                Selector2(TestB),
            }
            impl<'a> ExternalUnionBRef<'a> {
                pub fn to_owned(&self) -> ExternalUnionB {
                    match self {
                        ExternalUnionBRef::Selector0 => ExternalUnionB::Selector0,
                        ExternalUnionBRef::Selector1(v) => {
                            ExternalUnionB::Selector1(v.to_owned())
                        }
                        ExternalUnionBRef::Selector2(v) => {
                            ExternalUnionB::Selector2(v.to_owned())
                        }
                    }
                }
            }
            pub type TestA = external_ssz::A;
            pub type TestB = external_ssz::module_a::module_b::B;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ExternalContainer {
                pub field_a: external_ssz::A,
                pub field_b: external_ssz::module_a::module_b::B,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct ExternalContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ExternalContainerRef<'a> {
                pub fn field_a(&self) -> Result<external_ssz::A, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field_b(
                    &self,
                ) -> Result<external_ssz::module_a::module_b::B, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalContainerRef<'a> {
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
                    let field_a = self.field_a().expect("valid view");
                    hasher
                        .write(field_a.tree_hash_root().as_ref())
                        .expect("write field");
                    let field_b = self.field_b().expect("valid view");
                    hasher
                        .write(field_b.tree_hash_root().as_ref())
                        .expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ExternalContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 8usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 8usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..2usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            8usize,
                            2usize,
                            i,
                        )?;
                        if i == 0 && offset != 8usize {
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
            impl<'a> ExternalContainerRef<'a> {
                pub fn to_owned(&self) -> ExternalContainer {
                    ExternalContainer {
                        field_a: self.field_a().expect("valid view").to_owned(),
                        field_b: self.field_b().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
