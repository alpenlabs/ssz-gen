pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_external_ref_variants {
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
            pub struct ContainerWithExternal {
                pub payload: external_ssz::MsgPayload,
                pub account_id: external_ssz::AccountId,
                pub messages: VariableList<external_ssz::MessagePayload, 10usize>,
            }
            impl tree_hash::TreeHash for ContainerWithExternal {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.payload)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.account_id)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.messages)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ContainerWithExternal`].
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
            pub struct ContainerWithExternalRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithExternalRef<'a> {
                pub fn payload(
                    &self,
                ) -> Result<external_ssz::MsgPayloadRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::MsgPayload as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::MsgPayload as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::AccountId as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::AccountId as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn account_id(
                    &self,
                ) -> Result<external_ssz::AccountId, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::MsgPayload as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::MsgPayload as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::AccountId as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::AccountId as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn messages(
                    &self,
                ) -> Result<
                    ListRef<'a, external_ssz::MessagePayloadRef<'a>, 10usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::MsgPayload as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::MsgPayload as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::AccountId as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::AccountId as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        2usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for ContainerWithExternalRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    {
                        let payload = self.payload().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&payload);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let account_id = self.account_id().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&account_id);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let messages = self.messages().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&messages);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithExternalRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <external_ssz::MsgPayload as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::MsgPayload as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::AccountId as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::AccountId as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::MessagePayload,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for ContainerWithExternalRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(
                        !<external_ssz::MsgPayload as ssz::Encode>::is_ssz_fixed_len(),
                    )
                        + usize::from(
                            !<external_ssz::AccountId as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<VariableList<
                                external_ssz::MessagePayload,
                                10usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <external_ssz::MsgPayload as ssz::Encode>::ssz_fixed_len()
                            + <external_ssz::AccountId as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                external_ssz::MessagePayload,
                                10usize,
                            > as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ContainerWithExternal>
            for ContainerWithExternalRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ContainerWithExternal {
                    <ContainerWithExternalRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithExternalRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ContainerWithExternal {
                    ContainerWithExternal {
                        payload: {
                            let view = self.payload().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        account_id: {
                            let view = self.account_id().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        messages: {
                            let view = self.messages().expect("valid view");
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
