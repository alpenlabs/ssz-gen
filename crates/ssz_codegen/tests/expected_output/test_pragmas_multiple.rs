pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_multiple {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test multiple pragmas on a class
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                serde::Serialize,
                serde::Deserialize,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[repr(C)]
            #[ssz(struct_behaviour = "container")]
            pub struct MultiPragmaContainer {
                pub x: u32,
                pub y: u32,
            }
            impl tree_hash::TreeHash for MultiPragmaContainer {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.x)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.y)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`MultiPragmaContainer`].
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
                serde::Serialize,
                serde::Deserialize,
                std::marker::Copy
            )]
            pub struct MultiPragmaContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MultiPragmaContainerRef<'a> {
                pub fn x(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn y(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for MultiPragmaContainerRef<'a> {
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
                        let x = self.x().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&x);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let y = self.y().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&y);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MultiPragmaContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u32 as ssz::Encode>::is_ssz_fixed_len(),
                                <u32 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for MultiPragmaContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u32 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<MultiPragmaContainer>
            for MultiPragmaContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> MultiPragmaContainer {
                    <MultiPragmaContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MultiPragmaContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> MultiPragmaContainer {
                    MultiPragmaContainer {
                        x: self.x().expect("valid view"),
                        y: self.y().expect("valid view"),
                    }
                }
            }
        }
    }
}
