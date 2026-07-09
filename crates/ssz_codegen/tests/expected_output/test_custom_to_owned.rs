pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_custom_to_owned {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct InnerData {
                pub value: u64,
                pub hash: FixedBytes<32usize>,
            }
            impl tree_hash::TreeHash for InnerData {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.value)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.hash)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`InnerData`].
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
            pub struct InnerDataRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> InnerDataRef<'a> {
                pub fn value(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn hash(
                    &self,
                ) -> Result<FixedBytesRef<'a, 32usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for InnerDataRef<'a> {
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
                        let value = self.value().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&value);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let hash = self.hash().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&hash);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerDataRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for InnerDataRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u64 as ssz::Encode>::ssz_fixed_len()
                            + <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<InnerData> for InnerDataRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> InnerData {
                    <InnerDataRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> InnerDataRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> InnerData {
                    InnerData {
                        value: self.value().expect("valid view"),
                        hash: ssz_types::FixedBytes(
                            self.hash().expect("valid view").to_owned(),
                        ),
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
            pub struct OuterContainer {
                pub inner: InnerData,
                pub items: VariableList<InnerData, 10usize>,
            }
            impl tree_hash::TreeHash for OuterContainer {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.inner)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.items)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`OuterContainer`].
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
            pub struct OuterContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> OuterContainerRef<'a> {
                pub fn inner(&self) -> Result<InnerDataRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <InnerData as ssz::Encode>::is_ssz_fixed_len(),
                                <InnerData as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn items(
                    &self,
                ) -> Result<ListRef<'a, InnerDataRef<'a>, 10usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <InnerData as ssz::Encode>::is_ssz_fixed_len(),
                                <InnerData as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for OuterContainerRef<'a> {
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
                        let inner = self.inner().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&inner);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let items = self.items().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&items);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for OuterContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <InnerData as ssz::Encode>::is_ssz_fixed_len(),
                                <InnerData as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    InnerData,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for OuterContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<InnerData as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<VariableList<
                                InnerData,
                                10usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <InnerData as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                InnerData,
                                10usize,
                            > as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<OuterContainer>
            for OuterContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> OuterContainer {
                    <OuterContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> OuterContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> OuterContainer {
                    OuterContainer {
                        inner: {
                            let view = self.inner().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        items: {
                            let view = self.items().expect("valid view");
                            let items: Result<Vec<_>, _> = view
                                .iter()
                                .map(|item_result| {
                                    item_result
                                        .map(|item| ssz_types::view::ToOwnedSsz::to_owned(&item))
                                })
                                .collect();
                            let items = items.expect("valid view");
                            ssz_types::VariableList::new(items).expect("valid view")
                        },
                    }
                }
            }
        }
    }
}
