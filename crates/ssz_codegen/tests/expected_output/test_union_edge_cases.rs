pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_union_edge_cases {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum AnotherSimple {
                Selector0(bool),
                Selector1(u32),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for AnotherSimple {
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
                        AnotherSimple::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        AnotherSimple::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            #[derive(Debug, Copy, Clone)]
            pub struct AnotherSimpleRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AnotherSimpleRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<bool, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AnotherSimple: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u32, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AnotherSimple: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> AnotherSimple {
                    match self.selector() {
                        0u8 => {
                            AnotherSimple::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            AnotherSimple::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AnotherSimpleRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for AnotherSimpleRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<AnotherSimple>
            for AnotherSimpleRef<'a> {
                fn to_owned(&self) -> AnotherSimple {
                    <AnotherSimpleRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AnotherSimpleRef<'a> {
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
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum ComplexUnion {
                Selector0(VariableList<u8, 10usize>),
                Selector1(FixedVector<u16, 5usize>),
                SimpleUnion(SimpleUnion),
                Selector3(BitVector<32usize>),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for ComplexUnion {
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
                        ComplexUnion::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        ComplexUnion::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        ComplexUnion::SimpleUnion(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                        ComplexUnion::Selector3(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 3u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            #[derive(Debug, Copy, Clone)]
            pub struct ComplexUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ComplexUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(
                    &self,
                ) -> Result<BytesRef<'a, 10usize>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ComplexUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(
                    &self,
                ) -> Result<FixedVectorRef<'a, u16, 5usize>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ComplexUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(
                    &self,
                ) -> Result<SimpleUnionRef<'a>, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ComplexUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector3(
                    &self,
                ) -> Result<BitVectorRef<'a, 32usize>, ssz::DecodeError> {
                    if self.selector() != 3u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for ComplexUnion: expected 3".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> ComplexUnion {
                    match self.selector() {
                        0u8 => {
                            ComplexUnion::Selector0({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        1u8 => {
                            ComplexUnion::Selector1({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        2u8 => {
                            ComplexUnion::SimpleUnion({
                                let view = self.as_selector2().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        3u8 => {
                            ComplexUnion::Selector3({
                                let view = self.as_selector3().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ComplexUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for ComplexUnionRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<ComplexUnion> for ComplexUnionRef<'a> {
                fn to_owned(&self) -> ComplexUnion {
                    <ComplexUnionRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ComplexUnionRef<'a> {
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
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    2u8,
                                )
                                .expect("valid selector")
                        }
                        3u8 => {
                            let value = self.as_selector3().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    3u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum MixedOptional {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for MixedOptional {
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
                        MixedOptional::Selector0 => {
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        MixedOptional::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        MixedOptional::Selector2(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            #[derive(Debug, Copy, Clone)]
            pub struct MixedOptionalRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> MixedOptionalRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedOptional: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedOptional: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for MixedOptional: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> MixedOptional {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            MixedOptional::Selector0
                        }
                        1u8 => {
                            MixedOptional::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            MixedOptional::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MixedOptionalRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for MixedOptionalRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<MixedOptional>
            for MixedOptionalRef<'a> {
                fn to_owned(&self) -> MixedOptional {
                    <MixedOptionalRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for MixedOptionalRef<'a> {
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
                            let zero_root = H::get_zero_hash(0);
                            tree_hash::mix_in_selector_with_hasher::<H>(&zero_root, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    2u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum NestedUnion {
                SimpleUnion(SimpleUnion),
                AnotherSimple(AnotherSimple),
                Selector2(u64),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for NestedUnion {
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
                        NestedUnion::SimpleUnion(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        NestedUnion::AnotherSimple(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        NestedUnion::Selector2(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            #[derive(Debug, Copy, Clone)]
            pub struct NestedUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> NestedUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(
                    &self,
                ) -> Result<SimpleUnionRef<'a>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for NestedUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(
                    &self,
                ) -> Result<AnotherSimpleRef<'a>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for NestedUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u64, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for NestedUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> NestedUnion {
                    match self.selector() {
                        0u8 => {
                            NestedUnion::SimpleUnion({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        1u8 => {
                            NestedUnion::AnotherSimple({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        2u8 => {
                            NestedUnion::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for NestedUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for NestedUnionRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<NestedUnion> for NestedUnionRef<'a> {
                fn to_owned(&self) -> NestedUnion {
                    <NestedUnionRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for NestedUnionRef<'a> {
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
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    2u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum SimpleUnion {
                Selector0(u8),
                Selector1(u16),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for SimpleUnion {
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
                        SimpleUnion::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        SimpleUnion::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            #[derive(Debug, Copy, Clone)]
            pub struct SimpleUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> SimpleUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SimpleUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for SimpleUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> SimpleUnion {
                    match self.selector() {
                        0u8 => {
                            SimpleUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            SimpleUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for SimpleUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for SimpleUnionRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<SimpleUnion> for SimpleUnionRef<'a> {
                fn to_owned(&self) -> SimpleUnion {
                    <SimpleUnionRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for SimpleUnionRef<'a> {
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
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(
                                    &<_ as tree_hash::TreeHash<H>>::tree_hash_root(&value),
                                    1u8,
                                )
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            pub type OptionalSimple = Option<u8>;
            pub type OptionalComplex = Option<VariableList<u16, 8usize>>;
            pub type OptionalUnion = Option<SimpleUnion>;
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct UnionEdgeCases {
                pub simple: SimpleUnion,
                pub nested: NestedUnion,
                pub complex: ComplexUnion,
                pub opt_simple: OptionalSimple,
                pub opt_complex: OptionalComplex,
                pub opt_union: OptionalUnion,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionEdgeCases {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(6usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.simple)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.nested)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.complex)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.opt_simple)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.opt_complex)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.opt_union)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`UnionEdgeCases`].
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
            pub struct UnionEdgeCasesRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionEdgeCasesRef<'a> {
                pub fn simple(&self) -> Result<SimpleUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn nested(&self) -> Result<NestedUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn complex(&self) -> Result<ComplexUnionRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        3usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn opt_simple(&self) -> Result<Option<u8>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        4usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    if bytes.is_empty() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: 0,
                            expected: 1,
                        });
                    }
                    let selector = bytes[0];
                    match selector {
                        0 => Ok(None),
                        1 => {
                            let inner = <u8 as ssz::view::DecodeView>::from_ssz_bytes(
                                &bytes[1..],
                            )?;
                            Ok(Some(inner))
                        }
                        _ => {
                            Err(
                                ssz::DecodeError::BytesInvalid(
                                    format!("Invalid union selector for Option: {}", selector),
                                ),
                            )
                        }
                    }
                }
                pub fn opt_complex(
                    &self,
                ) -> Result<Option<ListRef<'a, u16, 8usize>>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        4usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        5usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    if bytes.is_empty() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: 0,
                            expected: 1,
                        });
                    }
                    let selector = bytes[0];
                    match selector {
                        0 => Ok(None),
                        1 => {
                            let inner = <ListRef<
                                'a,
                                u16,
                                8usize,
                            > as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                            Ok(Some(inner))
                        }
                        _ => {
                            Err(
                                ssz::DecodeError::BytesInvalid(
                                    format!("Invalid union selector for Option: {}", selector),
                                ),
                            )
                        }
                    }
                }
                pub fn opt_union(
                    &self,
                ) -> Result<Option<SimpleUnionRef<'a>>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        24usize,
                        6usize,
                        5usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        24usize,
                        6usize,
                        6usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    if bytes.is_empty() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: 0,
                            expected: 1,
                        });
                    }
                    let selector = bytes[0];
                    match selector {
                        0 => Ok(None),
                        1 => {
                            let inner = <SimpleUnionRef<
                                'a,
                            > as ssz::view::DecodeView>::from_ssz_bytes(&bytes[1..])?;
                            Ok(Some(inner))
                        }
                        _ => {
                            Err(
                                ssz::DecodeError::BytesInvalid(
                                    format!("Invalid union selector for Option: {}", selector),
                                ),
                            )
                        }
                    }
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionEdgeCasesRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(6usize);
                    {
                        let simple = self.simple().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&simple);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let nested = self.nested().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&nested);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let complex = self.complex().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&complex);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let opt_simple = self.opt_simple().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&opt_simple);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let opt_complex = self.opt_complex().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&opt_complex);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let opt_union = self.opt_union().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&opt_union);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionEdgeCasesRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 24usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 24usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..6usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            24usize,
                            6usize,
                            i,
                        )?;
                        if i == 0 && offset != 24usize {
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
            impl<'a> ssz::view::SszTypeInfo for UnionEdgeCasesRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<UnionEdgeCases>
            for UnionEdgeCasesRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> UnionEdgeCases {
                    <UnionEdgeCasesRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionEdgeCasesRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> UnionEdgeCases {
                    UnionEdgeCases {
                        simple: {
                            let view = self.simple().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        nested: {
                            let view = self.nested().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        complex: {
                            let view = self.complex().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        opt_simple: self
                            .opt_simple()
                            .expect("valid view")
                            .map(|inner| ssz_types::view::ToOwnedSsz::to_owned(&inner)),
                        opt_complex: self
                            .opt_complex()
                            .expect("valid view")
                            .map(|inner| ssz_types::view::ToOwnedSsz::to_owned(&inner)),
                        opt_union: self
                            .opt_union()
                            .expect("valid view")
                            .map(|inner| ssz_types::view::ToOwnedSsz::to_owned(&inner)),
                    }
                }
            }
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct AllUnions {
                pub union1: SimpleUnion,
                pub union2: NestedUnion,
                pub union3: OptionalSimple,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for AllUnions {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.union1)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.union2)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.union3)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`AllUnions`].
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
            pub struct AllUnionsRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> AllUnionsRef<'a> {
                pub fn union1(&self) -> Result<SimpleUnionRef<'a>, ssz::DecodeError> {
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
                pub fn union2(&self) -> Result<NestedUnionRef<'a>, ssz::DecodeError> {
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
                pub fn union3(&self) -> Result<Option<u8>, ssz::DecodeError> {
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
                    if bytes.is_empty() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: 0,
                            expected: 1,
                        });
                    }
                    let selector = bytes[0];
                    match selector {
                        0 => Ok(None),
                        1 => {
                            let inner = <u8 as ssz::view::DecodeView>::from_ssz_bytes(
                                &bytes[1..],
                            )?;
                            Ok(Some(inner))
                        }
                        _ => {
                            Err(
                                ssz::DecodeError::BytesInvalid(
                                    format!("Invalid union selector for Option: {}", selector),
                                ),
                            )
                        }
                    }
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AllUnionsRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    {
                        let union1 = self.union1().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&union1);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let union2 = self.union2().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&union2);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let union3 = self.union3().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&union3);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AllUnionsRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for AllUnionsRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<AllUnions> for AllUnionsRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> AllUnions {
                    <AllUnionsRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> AllUnionsRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> AllUnions {
                    AllUnions {
                        union1: {
                            let view = self.union1().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        union2: {
                            let view = self.union2().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        union3: self
                            .union3()
                            .expect("valid view")
                            .map(|inner| ssz_types::view::ToOwnedSsz::to_owned(&inner)),
                    }
                }
            }
        }
    }
}
