pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_basic {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test basic pragma with derive
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                std::default::Default,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct BasicContainer {
                pub a: u8,
            }
            impl tree_hash::TreeHash for BasicContainer {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.a)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BasicContainer`].
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
                std::default::Default,
                std::marker::Copy
            )]
            pub struct BasicContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BasicContainerRef<'a> {
                pub fn a(&self) -> Result<u8, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u8 as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <u8 as ssz::Encode>::ssz_fixed_len(),
                        <u8 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for BasicContainerRef<'a> {
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
                        let a = self.a().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&a);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BasicContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <u8 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<u8 as ssz::Encode>::is_ssz_fixed_len(),
                    );
                    if num_variable_fields == 0 {
                        if bytes.len() != fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                    } else {
                        if bytes.len() < fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                        let mut prev_offset: Option<usize> = None;
                        for i in 0..num_variable_fields {
                            let offset = ssz::layout::read_variable_offset(
                                bytes,
                                fixed_portion_size,
                                num_variable_fields,
                                i,
                            )?;
                            if i == 0 && offset != fixed_portion_size {
                                return Err(
                                    ssz::DecodeError::OffsetIntoFixedPortion(offset),
                                );
                            }
                            if let Some(prev) = prev_offset && offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                            if offset > bytes.len() {
                                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                            }
                            prev_offset = Some(offset);
                        }
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for BasicContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u8 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BasicContainer>
            for BasicContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BasicContainer {
                    <BasicContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BasicContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BasicContainer {
                    BasicContainer {
                        a: self.a().expect("valid view"),
                    }
                }
            }
        }
    }
}
