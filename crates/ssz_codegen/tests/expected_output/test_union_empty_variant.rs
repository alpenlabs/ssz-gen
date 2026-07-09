pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_union_empty_variant {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Union with both empty and data variants
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum TestUnion {
                /// Empty variant (no data)
                Empty,
                /// Variant with data
                Data(DataVariant),
            }
            impl tree_hash::TreeHash for TestUnion {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    match self {
                        TestUnion::Empty => {
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        TestUnion::Data(inner) => {
                            let root = <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            pub type DataRef<'a> = DataVariantRef<'a>;
            #[derive(Debug, Copy, Clone)]
            pub struct TestUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> TestUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for TestUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<DataRef<'_>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for TestUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> TestUnion {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            TestUnion::Empty
                        }
                        1u8 => {
                            TestUnion::Data({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for TestUnionRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<TestUnion> for TestUnionRef<'a> {
                fn to_owned(&self) -> TestUnion {
                    <TestUnionRef<'a>>::to_owned(self)
                }
            }
            impl<'a> tree_hash::TreeHash for TestUnionRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
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
                                    &<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            /// Test union with empty/unit variant
            ///
            /// Some container type for the union variant
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct DataVariant {
                pub value: u64,
            }
            impl tree_hash::TreeHash for DataVariant {
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
            /// Zero-copy view over [`DataVariant`].
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
            pub struct DataVariantRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> DataVariantRef<'a> {
                pub fn value(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for DataVariantRef<'a> {
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
            impl<'a> ssz::view::DecodeView<'a> for DataVariantRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for DataVariantRef<'a> {
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
            impl<'a> ssz_types::view::ToOwnedSsz<DataVariant> for DataVariantRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> DataVariant {
                    <DataVariantRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> DataVariantRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> DataVariant {
                    DataVariant {
                        value: self.value().expect("valid view"),
                    }
                }
            }
            /// Container using the union
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct TestContainer {
                pub state: TestUnion,
            }
            impl tree_hash::TreeHash for TestContainer {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.state)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`TestContainer`].
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
            pub struct TestContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestContainerRef<'a> {
                pub fn state(&self) -> Result<TestUnionRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TestUnion as ssz::Encode>::is_ssz_fixed_len(),
                                <TestUnion as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for TestContainerRef<'a> {
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
                        let state = self.state().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&state);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <TestUnion as ssz::Encode>::is_ssz_fixed_len(),
                                <TestUnion as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for TestContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<TestUnion as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <TestUnion as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<TestContainer>
            for TestContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> TestContainer {
                    <TestContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> TestContainer {
                    TestContainer {
                        state: {
                            let view = self.state().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                    }
                }
            }
        }
    }
}
