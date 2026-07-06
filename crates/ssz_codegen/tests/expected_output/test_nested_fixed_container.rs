pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_nested_fixed_container {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const MAX_TAIL: u64 = 16u64;
            /// A fixed-size inner container (1 byte).
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct FixedInner {
                pub tag: u8,
            }
            impl tree_hash::TreeHash for FixedInner {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.tag)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`FixedInner`].
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
            pub struct FixedInnerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedInnerRef<'a> {
                pub fn tag(&self) -> Result<u8, ssz::DecodeError> {
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
            impl<'a> tree_hash::TreeHash for FixedInnerRef<'a> {
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
                        let tag = self.tag().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tag);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for FixedInnerRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for FixedInnerRef<'a> {
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
            impl<'a> ssz_types::view::ToOwnedSsz<FixedInner> for FixedInnerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> FixedInner {
                    <FixedInnerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedInnerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> FixedInner {
                    FixedInner {
                        tag: self.tag().expect("valid view"),
                    }
                }
            }
            /// A larger fixed-size inner container (8 bytes).
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct FixedPair {
                pub x: u32,
                pub y: u32,
            }
            impl tree_hash::TreeHash for FixedPair {
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
            /// Zero-copy view over [`FixedPair`].
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
            pub struct FixedPairRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedPairRef<'a> {
                pub fn x(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u32 as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn y(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u32 as ssz::Encode>::is_ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for FixedPairRef<'a> {
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
            impl<'a> ssz::view::DecodeView<'a> for FixedPairRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <u32 as ssz::Encode>::ssz_fixed_len()
                        + <u32 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<u32 as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len());
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
            impl<'a> ssz::view::SszTypeInfo for FixedPairRef<'a> {
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
            impl<'a> ssz_types::view::ToOwnedSsz<FixedPair> for FixedPairRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> FixedPair {
                    <FixedPairRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedPairRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> FixedPair {
                    FixedPair {
                        x: self.x().expect("valid view"),
                        y: self.y().expect("valid view"),
                    }
                }
            }
            /// Mixed container: fixed containers inline, one variable tail.
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct MixedOuter {
                pub inner: FixedInner,
                pub count: u32,
                pub pair: FixedPair,
                pub tail: VariableList<u8, 16usize>,
            }
            impl tree_hash::TreeHash for MixedOuter {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.inner)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.count)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.pair)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.tail)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`MixedOuter`].
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
            pub struct MixedOuterRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MixedOuterRef<'a> {
                pub fn inner(&self) -> Result<FixedInnerRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <FixedInner as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <FixedInner as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            )
                            + usize::from(
                                !<VariableList<
                                    u8,
                                    16usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn count(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u32 as ssz::Encode>::is_ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            )
                            + usize::from(
                                !<VariableList<
                                    u8,
                                    16usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn pair(&self) -> Result<FixedPairRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        <FixedPair as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            )
                            + usize::from(
                                !<VariableList<
                                    u8,
                                    16usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn tail(&self) -> Result<BytesRef<'a, 16usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <VariableList<u8, 16usize> as ssz::Encode>::is_ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len(),
                        <VariableList<u8, 16usize> as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            )
                            + usize::from(
                                !<VariableList<
                                    u8,
                                    16usize,
                                > as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for MixedOuterRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    {
                        let inner = self.inner().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&inner);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let count = self.count().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&count);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let pair = self.pair().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&pair);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let tail = self.tail().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tail);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MixedOuterRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <FixedInner as ssz::Encode>::ssz_fixed_len()
                        + <u32 as ssz::Encode>::ssz_fixed_len()
                        + <FixedPair as ssz::Encode>::ssz_fixed_len()
                        + <VariableList<u8, 16usize> as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<FixedInner as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<FixedPair as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
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
            impl<'a> ssz::view::SszTypeInfo for MixedOuterRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<FixedPair as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(
                            !<VariableList<
                                u8,
                                16usize,
                            > as ssz::Encode>::is_ssz_fixed_len(),
                        ) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                            + <VariableList<u8, 16usize> as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<MixedOuter> for MixedOuterRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> MixedOuter {
                    <MixedOuterRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MixedOuterRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> MixedOuter {
                    MixedOuter {
                        inner: {
                            let view = self.inner().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        count: self.count().expect("valid view"),
                        pair: {
                            let view = self.pair().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        tail: ssz_types::VariableList::new(
                                self.tail().expect("valid view").to_owned(),
                            )
                            .expect("valid view"),
                    }
                }
            }
            /// Fully fixed container nesting fixed containers.
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct FixedOuter {
                pub inner: FixedInner,
                pub pair: FixedPair,
            }
            impl tree_hash::TreeHash for FixedOuter {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.pair)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`FixedOuter`].
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
            pub struct FixedOuterRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedOuterRef<'a> {
                pub fn inner(&self) -> Result<FixedInnerRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <FixedInner as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <FixedInner as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn pair(&self) -> Result<FixedPairRef<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len(),
                        <FixedPair as ssz::Encode>::ssz_fixed_len(),
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(
                                !<FixedPair as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                        usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for FixedOuterRef<'a> {
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
                        let pair = self.pair().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&pair);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for FixedOuterRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <FixedInner as ssz::Encode>::ssz_fixed_len()
                        + <FixedPair as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<FixedInner as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<FixedPair as ssz::Encode>::is_ssz_fixed_len());
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
            impl<'a> ssz::view::SszTypeInfo for FixedOuterRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<FixedInner as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<FixedPair as ssz::Encode>::is_ssz_fixed_len())
                        == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <FixedInner as ssz::Encode>::ssz_fixed_len()
                            + <FixedPair as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<FixedOuter> for FixedOuterRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> FixedOuter {
                    <FixedOuterRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FixedOuterRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> FixedOuter {
                    FixedOuter {
                        inner: {
                            let view = self.inner().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        pair: {
                            let view = self.pair().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                    }
                }
            }
            /// Basic-fields-only container: decodes fine either way, but exercises the view
            /// TreeHash leaf packing.
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct BasicPair {
                pub tag: u8,
                pub b: u32,
            }
            impl tree_hash::TreeHash for BasicPair {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.tag)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.b)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BasicPair`].
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
            pub struct BasicPairRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BasicPairRef<'a> {
                pub fn tag(&self) -> Result<u8, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u8 as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <u8 as ssz::Encode>::ssz_fixed_len(),
                        <u8 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u32 as ssz::Encode>::is_ssz_fixed_len(),
                        <u8 as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <u8 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for BasicPairRef<'a> {
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
                        let tag = self.tag().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tag);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let b = self.b().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&b);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BasicPairRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <u8 as ssz::Encode>::ssz_fixed_len()
                        + <u32 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<u8 as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len());
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
            impl<'a> ssz::view::SszTypeInfo for BasicPairRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u8 as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BasicPair> for BasicPairRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BasicPair {
                    <BasicPairRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BasicPairRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BasicPair {
                    BasicPair {
                        tag: self.tag().expect("valid view"),
                        b: self.b().expect("valid view"),
                    }
                }
            }
        }
    }
}
