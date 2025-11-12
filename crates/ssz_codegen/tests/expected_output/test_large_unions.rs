pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_large_unions {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum BigUnion {
                Selector0(u8),
                Selector1(u16),
                Selector2(u32),
                Selector3(u64),
                Selector4(u128),
                Selector5(u256),
            }
            #[derive(Debug, Copy, Clone)]
            pub struct BigUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> BigUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u32, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector3(&self) -> Result<u64, ssz::DecodeError> {
                    if self.selector() != 3u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 3".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector4(&self) -> Result<u128, ssz::DecodeError> {
                    if self.selector() != 4u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 4".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector5(&self) -> Result<u256, ssz::DecodeError> {
                    if self.selector() != 5u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for BigUnion: expected 5".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> BigUnion {
                    match self.selector() {
                        0u8 => {
                            BigUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            BigUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            BigUnion::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        3u8 => {
                            BigUnion::Selector3(
                                self.as_selector3().expect("valid selector"),
                            )
                        }
                        4u8 => {
                            BigUnion::Selector4(
                                self.as_selector4().expect("valid selector"),
                            )
                        }
                        5u8 => {
                            BigUnion::Selector5(
                                self.as_selector5().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BigUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for BigUnionRef<'a> {
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
                            >(&value.tree_hash_root(), 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        3u8 => {
                            let value = self.as_selector3().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 3u8)
                                .expect("valid selector")
                        }
                        4u8 => {
                            let value = self.as_selector4().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 4u8)
                                .expect("valid selector")
                        }
                        5u8 => {
                            let value = self.as_selector5().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 5u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedUnion {
                Selector0(u8),
                Selector1(VariableList<u8, 5usize>),
                Selector2(FixedVector<u16, 3usize>),
                Selector3(BitVector<8usize>),
            }
            #[derive(Debug, Copy, Clone)]
            pub struct MixedUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> MixedUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(
                    &self,
                ) -> Result<FixedVectorRef<'a, u16, 3usize>, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector3(
                    &self,
                ) -> Result<BitVectorRef<'a, 8usize>, ssz::DecodeError> {
                    if self.selector() != 3u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedUnion: expected 3".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> MixedUnion {
                    match self.selector() {
                        0u8 => {
                            MixedUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            MixedUnion::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        2u8 => {
                            MixedUnion::Selector2(
                                self.as_selector2().expect("valid selector").to_owned(),
                            )
                        }
                        3u8 => {
                            MixedUnion::Selector3(
                                self.as_selector3().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MixedUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for MixedUnionRef<'a> {
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
                            >(&value.tree_hash_root(), 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        3u8 => {
                            let value = self.as_selector3().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 3u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SameTypeUnion {
                Selector0(u8),
                Selector1(u8),
                Selector2(u8),
                Selector3(u8),
            }
            #[derive(Debug, Copy, Clone)]
            pub struct SameTypeUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> SameTypeUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SameTypeUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SameTypeUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SameTypeUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector3(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 3u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SameTypeUnion: expected 3".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> SameTypeUnion {
                    match self.selector() {
                        0u8 => {
                            SameTypeUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            SameTypeUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            SameTypeUnion::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        3u8 => {
                            SameTypeUnion::Selector3(
                                self.as_selector3().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for SameTypeUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for SameTypeUnionRef<'a> {
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
                            >(&value.tree_hash_root(), 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        3u8 => {
                            let value = self.as_selector3().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 3u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ContainerWithBigUnions {
                pub big: BigUnion,
                pub same: SameTypeUnion,
                pub mixed: MixedUnion,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithBigUnions {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.big)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.same)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.mixed)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ContainerWithBigUnions`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerWithBigUnionsRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithBigUnionsRef<'a> {
                pub fn big(&self) -> Result<BigUnionRef<'a>, ssz::DecodeError> {
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
                pub fn same(&self) -> Result<SameTypeUnionRef<'a>, ssz::DecodeError> {
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
                pub fn mixed(&self) -> Result<MixedUnionRef<'a>, ssz::DecodeError> {
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
            for ContainerWithBigUnionsRef<'a> {
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
                        let big = self.big().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&big);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let same = self.same().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&same);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let mixed = self.mixed().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&mixed);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithBigUnionsRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ContainerWithBigUnionsRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ContainerWithBigUnions>
            for ContainerWithBigUnionsRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ContainerWithBigUnions {
                    <ContainerWithBigUnionsRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithBigUnionsRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ContainerWithBigUnions {
                    ContainerWithBigUnions {
                        big: self
                            .big()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        same: self
                            .same()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        mixed: self
                            .mixed()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                    }
                }
            }
        }
    }
}
