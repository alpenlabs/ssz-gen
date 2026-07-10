pub mod existing_module {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    use ssz_types::*;
    use ssz_types::view::{FixedVectorRef, VariableListRef};
    use ssz_primitives::{U128, U256};
    use ssz_derive::{Encode, Decode};
    use tree_hash::TreeHashDigest;
    use tree_hash_derive::TreeHash;
    use ssz::view::*;
}
pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_existing_rust_module {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test container that references types from an existing Rust module
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct TestExistingModule {
                /// Field using type from existing module (no .ssz file)
                pub existing_field: crate::existing_module::ExistingType,
                /// Another field
                pub slot: u64,
            }
            impl tree_hash::TreeHash for TestExistingModule {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.existing_field)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.slot)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`TestExistingModule`].
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
            pub struct TestExistingModuleRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestExistingModuleRef<'a> {
                pub fn existing_field(
                    &self,
                ) -> Result<crate::existing_module::ExistingType, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <crate::existing_module::ExistingType as ssz::Encode>::is_ssz_fixed_len(),
                                <crate::existing_module::ExistingType as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn slot(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <crate::existing_module::ExistingType as ssz::Encode>::is_ssz_fixed_len(),
                                <crate::existing_module::ExistingType as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for TestExistingModuleRef<'a> {
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
                        let existing_field = self.existing_field().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&existing_field);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let slot = self.slot().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&slot);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestExistingModuleRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <crate::existing_module::ExistingType as ssz::Encode>::is_ssz_fixed_len(),
                                <crate::existing_module::ExistingType as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                                <u64 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for TestExistingModuleRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(
                        !<crate::existing_module::ExistingType as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <crate::existing_module::ExistingType as ssz::Encode>::ssz_fixed_len()
                            + <u64 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<TestExistingModule>
            for TestExistingModuleRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> TestExistingModule {
                    <TestExistingModuleRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestExistingModuleRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> TestExistingModule {
                    TestExistingModule {
                        existing_field: {
                            let view = self.existing_field().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        slot: self.slot().expect("valid view"),
                    }
                }
            }
        }
    }
}
