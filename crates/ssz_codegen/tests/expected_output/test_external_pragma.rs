pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_external_pragma {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test external_kind pragma for container vs primitive external types
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct ExternalPragmaTest {
                /// External container type - needs Ref variant
                pub state: external_ssz::ChainState,
                /// External primitive type - no Ref variant (default behavior)
                pub balance: external_ssz::Balance,
                /// External container in Vector - needs Ref variant for inner type
                pub headers: FixedVector<external_ssz::BlockHeader, 10usize>,
                /// External container in List - needs Ref variant for inner type
                pub transactions: VariableList<external_ssz::Transaction, 100usize>,
                /// External primitive in List - no annotation needed
                pub account_ids: VariableList<external_ssz::AccountId, 50usize>,
            }
            impl tree_hash::TreeHash for ExternalPragmaTest {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.state)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.balance)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.headers)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.transactions)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.account_ids)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ExternalPragmaTest`].
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
            pub struct ExternalPragmaTestRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ExternalPragmaTestRef<'a> {
                pub fn state(
                    &self,
                ) -> Result<external_ssz::ChainStateRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn balance(
                    &self,
                ) -> Result<external_ssz::Balance, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn headers(
                    &self,
                ) -> Result<
                    FixedVectorRef<'a, external_ssz::BlockHeaderRef<'a>, 10usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        2usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn transactions(
                    &self,
                ) -> Result<
                    ListRef<'a, external_ssz::TransactionRef<'a>, 100usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        3usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn account_ids(
                    &self,
                ) -> Result<
                    ListRef<'a, external_ssz::AccountId, 50usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        4usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for ExternalPragmaTestRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    {
                        let state = self.state().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&state);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let balance = self.balance().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&balance);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let headers = self.headers().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&headers);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let transactions = self.transactions().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&transactions);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let account_ids = self.account_ids().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&account_ids);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ExternalPragmaTestRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                                <external_ssz::Balance as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <FixedVector<
                                    external_ssz::BlockHeader,
                                    10usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::Transaction,
                                    100usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                                <VariableList<
                                    external_ssz::AccountId,
                                    50usize,
                                > as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for ExternalPragmaTestRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(
                        !<external_ssz::ChainState as ssz::Encode>::is_ssz_fixed_len(),
                    )
                        + usize::from(
                            !<external_ssz::Balance as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<FixedVector<
                                external_ssz::BlockHeader,
                                10usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<VariableList<
                                external_ssz::Transaction,
                                100usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<VariableList<
                                external_ssz::AccountId,
                                50usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <external_ssz::ChainState as ssz::Encode>::ssz_fixed_len()
                            + <external_ssz::Balance as ssz::Encode>::ssz_fixed_len()
                            + <FixedVector<
                                external_ssz::BlockHeader,
                                10usize,
                            > as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                external_ssz::Transaction,
                                100usize,
                            > as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                external_ssz::AccountId,
                                50usize,
                            > as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ExternalPragmaTest>
            for ExternalPragmaTestRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ExternalPragmaTest {
                    <ExternalPragmaTestRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ExternalPragmaTestRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ExternalPragmaTest {
                    ExternalPragmaTest {
                        state: {
                            let view = self.state().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        balance: {
                            let view = self.balance().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        headers: self
                            .headers()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        transactions: {
                            let view = self.transactions().expect("valid view");
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
                        account_ids: {
                            let view = self.account_ids().expect("valid view");
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
