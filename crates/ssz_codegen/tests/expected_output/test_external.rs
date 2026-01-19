pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_external {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum ExternalUnionA {
                Selector0,
                A(external_ssz::A),
                B(external_ssz::module_a::module_b::B),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalUnionA {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self {
                        ExternalUnionA::Selector0 => {
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        ExternalUnionA::A(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        ExternalUnionA::B(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                    }
                }
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
                            ExternalUnionA::A({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        2u8 => {
                            ExternalUnionA::B({
                                let view = self.as_selector2().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
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
            impl<'a> ssz::view::SszTypeInfo for ExternalUnionARef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<ExternalUnionA>
            for ExternalUnionARef<'a> {
                fn to_owned(&self) -> ExternalUnionA {
                    <ExternalUnionARef<'a>>::to_owned(self)
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
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    2u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum ExternalUnionB {
                Selector0,
                TestA(TestA),
                TestB(TestB),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalUnionB {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self {
                        ExternalUnionB::Selector0 => {
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        ExternalUnionB::TestA(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        ExternalUnionB::TestB(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                    }
                }
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
                            ExternalUnionB::TestA({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        2u8 => {
                            ExternalUnionB::TestB({
                                let view = self.as_selector2().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
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
            impl<'a> ssz::view::SszTypeInfo for ExternalUnionBRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<ExternalUnionB>
            for ExternalUnionBRef<'a> {
                fn to_owned(&self) -> ExternalUnionB {
                    <ExternalUnionBRef<'a>>::to_owned(self)
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
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    2u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            pub type TestA = external_ssz::A;
            pub type TestB = external_ssz::module_a::module_b::B;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ExternalContainer {
                pub field_a: external_ssz::A,
                pub field_b: external_ssz::module_a::module_b::B,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ExternalContainer {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.field_a)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.field_b)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ExternalContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ExternalContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
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
                        1usize,
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
                        2usize,
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    {
                        let field_a = self.field_a().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&field_a);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let field_b = self.field_b().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&field_b);
                        hasher.write(root.as_ref()).expect("write field");
                    }
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
            impl<'a> ssz::view::SszTypeInfo for ExternalContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ExternalContainer>
            for ExternalContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ExternalContainer {
                    <ExternalContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ExternalContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ExternalContainer {
                    ExternalContainer {
                        field_a: {
                            let view = self.field_a().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        field_b: {
                            let view = self.field_b().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                    }
                }
            }
        }
    }
}
