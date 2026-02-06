pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_external_container_union {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Pending input we expect to see in a block Using external container type for
            /// the deposit
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum PendingInputEntry {
                Deposit(external_ssz::SubjectDepositData),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for PendingInputEntry {
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
                        PendingInputEntry::Deposit(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            pub type DepositRef<'a> = external_ssz::SubjectDepositDataRef<'a>;
            #[derive(Debug, Copy, Clone)]
            pub struct PendingInputEntryRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> PendingInputEntryRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<DepositRef<'_>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for PendingInputEntry: expected 0"
                                    .to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> PendingInputEntry {
                    match self.selector() {
                        0u8 => {
                            PendingInputEntry::Deposit({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for PendingInputEntryRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for PendingInputEntryRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<PendingInputEntry>
            for PendingInputEntryRef<'a> {
                fn to_owned(&self) -> PendingInputEntry {
                    <PendingInputEntryRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for PendingInputEntryRef<'a> {
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
                            let value = self.as_selector0().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    0u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            /// Container using the union
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct TestContainer {
                /// Pending inputs
                pub pending_inputs: VariableList<PendingInputEntry, 10usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for TestContainer {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.pending_inputs)
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
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct TestContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestContainerRef<'a> {
                pub fn pending_inputs(
                    &self,
                ) -> Result<
                    ListRef<'a, PendingInputEntryRef<'a>, 10usize>,
                    ssz::DecodeError,
                > {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        4usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        4usize,
                        1usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for TestContainerRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    {
                        let pending_inputs = self.pending_inputs().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&pending_inputs);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 4usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 4usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            4usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 4usize {
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
            impl<'a> ssz::view::SszTypeInfo for TestContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
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
                        pending_inputs: {
                            let view = self.pending_inputs().expect("valid view");
                            let items: Result<Vec<_>, _> = view
                                .iter()
                                .map(|item_result| {
                                    item_result
                                        .map(|item| ssz_types::view::ToOwnedSsz::to_owned(&item))
                                })
                                .collect();
                            let items = items.expect("valid view");
                            ssz_types::VariableList::from(items)
                        },
                    }
                }
            }
        }
    }
}
