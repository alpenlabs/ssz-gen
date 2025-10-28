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
            #[derive(Debug, Copy, Clone)]
            pub struct ExternalUnionARef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ExternalUnionARef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionA: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<external_ssz::A, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionA: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(
                    &self,
                ) -> Result<external_ssz::module_a::module_b::B, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionA: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> ExternalUnionA {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            ExternalUnionA::Selector0
                        }
                        1u8 => {
                            ExternalUnionA::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        2u8 => {
                            ExternalUnionA::Selector2(
                                self.as_selector2().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ExternalUnionARef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalUnionARef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self.selector() {
                        0u8 => {
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
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
            #[derive(Debug, Copy, Clone)]
            pub struct ExternalUnionBRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ExternalUnionBRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionB: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<TestA, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionB: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<TestB, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ExternalUnionB: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> ExternalUnionB {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            ExternalUnionB::Selector0
                        }
                        1u8 => {
                            ExternalUnionB::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        2u8 => {
                            ExternalUnionB::Selector2(
                                self.as_selector2().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ExternalUnionBRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalUnionBRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self.selector() {
                        0u8 => {
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
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
