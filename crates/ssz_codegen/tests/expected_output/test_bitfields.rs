pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_bitfields {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SMALL_SIZE: u64 = 1u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const MEDIUM_SIZE: u64 = 64u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const LARGE_SIZE: u64 = 256u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const POWER_OF_TWO: u64 = 128u64;
            pub type TinyBitlist = BitList<{ SMALL_SIZE as usize }>;
            pub type StandardBitlist = BitList<{ MEDIUM_SIZE as usize }>;
            pub type LargeBitlist = BitList<{ LARGE_SIZE as usize }>;
            pub type TinyBitvector = BitVector<{ SMALL_SIZE as usize }>;
            pub type StandardBitvector = BitVector<{ MEDIUM_SIZE as usize }>;
            pub type LargeBitvector = BitVector<{ POWER_OF_TWO as usize }>;
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct BitfieldContainer {
                pub tiny_list: TinyBitlist,
                pub std_list: StandardBitlist,
                pub large_list: LargeBitlist,
                pub tiny_vec: TinyBitvector,
                pub std_vec: StandardBitvector,
                pub large_vec: LargeBitvector,
            }
            impl tree_hash::TreeHash for BitfieldContainer {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(6usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.tiny_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.std_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.large_list)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.tiny_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.std_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.large_vec)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BitfieldContainer`].
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
            pub struct BitfieldContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitfieldContainerRef<'a> {
                pub fn tiny_list(
                    &self,
                ) -> Result<BitListRef<'a, 1usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn std_list(
                    &self,
                ) -> Result<BitListRef<'a, 64usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn large_list(
                    &self,
                ) -> Result<BitListRef<'a, 256usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        2usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn tiny_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 1usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        3usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn std_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 64usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        4usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn large_vec(
                    &self,
                ) -> Result<BitVectorRef<'a, 128usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        5usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for BitfieldContainerRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(6usize);
                    {
                        let tiny_list = self.tiny_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tiny_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let std_list = self.std_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&std_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let large_list = self.large_list().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&large_list);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let tiny_vec = self.tiny_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tiny_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let std_vec = self.std_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&std_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let large_vec = self.large_vec().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&large_vec);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BitfieldContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <TinyBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitlist as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitlist as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <TinyBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <StandardBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                                <LargeBitvector as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for BitfieldContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<TinyBitlist as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<StandardBitlist as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(!<LargeBitlist as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<TinyBitvector as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<StandardBitvector as ssz::Encode>::is_ssz_fixed_len(),
                        )
                        + usize::from(
                            !<LargeBitvector as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <TinyBitlist as ssz::Encode>::ssz_fixed_len()
                            + <StandardBitlist as ssz::Encode>::ssz_fixed_len()
                            + <LargeBitlist as ssz::Encode>::ssz_fixed_len()
                            + <TinyBitvector as ssz::Encode>::ssz_fixed_len()
                            + <StandardBitvector as ssz::Encode>::ssz_fixed_len()
                            + <LargeBitvector as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BitfieldContainer>
            for BitfieldContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BitfieldContainer {
                    <BitfieldContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitfieldContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BitfieldContainer {
                    BitfieldContainer {
                        tiny_list: self.tiny_list().expect("valid view").to_owned(),
                        std_list: self.std_list().expect("valid view").to_owned(),
                        large_list: self.large_list().expect("valid view").to_owned(),
                        tiny_vec: self.tiny_vec().expect("valid view").to_owned(),
                        std_vec: self.std_vec().expect("valid view").to_owned(),
                        large_vec: self.large_vec().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
