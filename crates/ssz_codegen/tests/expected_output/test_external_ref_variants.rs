pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_external_ref_variants {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ContainerWithExternal {
                pub payload: external_ssz::MsgPayload,
                pub account_id: external_ssz::AccountId,
                pub messages: VariableList<external_ssz::MessagePayload, 10usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithExternal {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.payload)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.account_id)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.messages)
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
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerWithExternalRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithExternalRef<'a> {
                pub fn payload(
                    &self,
                ) -> Result<external_ssz::MsgPayloadRef<'a>, ssz::DecodeError> {
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
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn account_id(
                    &self,
                ) -> Result<external_ssz::AccountId, ssz::DecodeError> {
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
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn messages(
                    &self,
                ) -> Result<
                    VariableListRef<'a, external_ssz::MessagePayloadRef<'a>, 10usize>,
                    ssz::DecodeError,
                > {
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
                        3usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithExternalRef<'a> {
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
                        let payload = self.payload().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&payload);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let account_id = self.account_id().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&account_id);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let messages = self.messages().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&messages);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithExternalRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ContainerWithExternalRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
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
                        payload: self
                            .payload()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        account_id: self
                            .account_id()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        messages: self
                            .messages()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                    }
                }
            }
        }
    }
}
