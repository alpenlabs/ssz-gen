pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_1 {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum AliasOptionUnion {
                Selector0(u8),
                Selector1(Option<u16>),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AliasOptionUnion {
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
                        AliasOptionUnion::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        AliasOptionUnion::Selector1(inner) => {
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
            pub struct AliasOptionUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AliasOptionUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasOptionUnion: expected 0"
                                    .to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<Option<u16>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasOptionUnion: expected 1"
                                    .to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> AliasOptionUnion {
                    match self.selector() {
                        0u8 => {
                            AliasOptionUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            AliasOptionUnion::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AliasOptionUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AliasOptionUnionRef<'a> {
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
            pub enum FirstUnion {
                Selector0(u8),
                Selector1(u16),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for FirstUnion {
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
                        FirstUnion::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        FirstUnion::Selector1(inner) => {
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
            pub struct FirstUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> FirstUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for FirstUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for FirstUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> FirstUnion {
                    match self.selector() {
                        0u8 => {
                            FirstUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            FirstUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for FirstUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for FirstUnionRef<'a> {
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
            pub enum TestUnion {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for TestUnion {
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
                        TestUnion::Selector0 => {
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
                                .expect("valid selector")
                        }
                        TestUnion::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        TestUnion::Selector2(inner) => {
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
            pub struct TestUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> TestUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for TestUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for TestUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for TestUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> TestUnion {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            TestUnion::Selector0
                        }
                        1u8 => {
                            TestUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            TestUnion::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for TestUnionRef<'a> {
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
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
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
            pub enum UnionA {
                Selector0(u8),
                Selector1(u8),
                Selector2(u16),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for UnionA {
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
                        UnionA::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionA::Selector1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        UnionA::Selector2(inner) => {
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
            pub struct UnionARef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionARef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionA: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionA: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionA: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionA {
                    match self.selector() {
                        0u8 => {
                            UnionA::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            UnionA::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            UnionA::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionARef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionARef<'a> {
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
            pub enum UnionB {
                Selector0(u8),
                UnionA(UnionA),
                Selector2(u32),
                Selector3(VariableList<u8, 12usize>),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for UnionB {
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
                        UnionB::Selector0(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionB::UnionA(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                        UnionB::Selector2(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 2u8)
                                .expect("valid selector")
                        }
                        UnionB::Selector3(inner) => {
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
            pub struct UnionBRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionBRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionB: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<UnionARef<'a>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionB: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<u32, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionB: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector3(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    if self.selector() != 3u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionB: expected 3".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionB {
                    match self.selector() {
                        0u8 => {
                            UnionB::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            UnionB::UnionA(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        2u8 => {
                            UnionB::Selector2(
                                self.as_selector2().expect("valid selector"),
                            )
                        }
                        3u8 => {
                            UnionB::Selector3(
                                self.as_selector3().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionBRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionBRef<'a> {
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
            pub enum UnionC {
                AliasUintAlias(AliasUintAlias),
                AliasUintAlias(AliasUintAlias),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for UnionC {
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
                        UnionC::AliasUintAlias(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionC::AliasUintAlias(inner) => {
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
            pub struct UnionCRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionCRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionC: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionC: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionC {
                    match self.selector() {
                        0u8 => {
                            UnionC::AliasUintAlias(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            UnionC::AliasUintAlias(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionCRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionCRef<'a> {
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
            pub enum UnionD {
                AliasUintAlias(AliasUintAlias),
                AliasUintAlias(AliasUintAlias),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for UnionD {
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
                        UnionD::AliasUintAlias(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionD::AliasUintAlias(inner) => {
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
            pub struct UnionDRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionDRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionD: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<u16, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionD: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionD {
                    match self.selector() {
                        0u8 => {
                            UnionD::AliasUintAlias(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            UnionD::AliasUintAlias(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionDRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionDRef<'a> {
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
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const VAL_X: u64 = 42u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const VAL_Y: u64 = 64u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SIZE_ALIAS: u64 = 64u64;
            pub type AliasUintAlias = u16;
            pub type AliasVecA = FixedBytes<10usize>;
            pub type AliasVecB = AliasVecA;
            pub type AliasListAlias = VariableList<u8, 5usize>;
            pub type AliasNested = AliasUintAlias;
            pub type BitAlias = BitList<{ VAL_X as usize }>;
            pub type UnionE = UnionD;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Alpha {
                pub a: u8,
                pub b: u16,
                pub c: AliasVecB,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Alpha {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.a)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.b)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.c)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Alpha`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct AlphaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> AlphaRef<'a> {
                pub fn a(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 0usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<u16, ssz::DecodeError> {
                    let offset = 1usize;
                    let end = offset + 2usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn c(&self) -> Result<FixedBytesRef<'a, 10usize>, ssz::DecodeError> {
                    let offset = 3usize;
                    let end = offset + 10usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AlphaRef<'a> {
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
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 1usize;
                        let field_bytes = &self.bytes[offset..offset + 2usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let c = self.c().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&c);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AlphaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 13usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 13usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for AlphaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    13usize
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Alpha> for AlphaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Alpha {
                    <AlphaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> AlphaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Alpha {
                    Alpha {
                        a: self.a().expect("valid view"),
                        b: self.b().expect("valid view"),
                        c: ssz_types::FixedBytes(
                            self.c().expect("valid view").to_owned(),
                        ),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Beta {
                pub d: AliasListAlias,
                pub e: u8,
                pub f: AliasUintAlias,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Beta {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.d)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.e)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.f)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Beta`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct BetaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BetaRef<'a> {
                pub fn d(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        7usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        7usize,
                        1usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn e(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 4usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn f(&self) -> Result<u16, ssz::DecodeError> {
                    let offset = 5usize;
                    let end = offset + 2usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for BetaRef<'a> {
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
                        let d = self.d().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&d);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let offset = 4usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 5usize;
                        let field_bytes = &self.bytes[offset..offset + 2usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 7usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 7usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            7usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 7usize {
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
            impl<'a> ssz::view::SszTypeInfo for BetaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Beta> for BetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Beta {
                    <BetaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Beta {
                    Beta {
                        d: self.d().expect("valid view").to_owned().into(),
                        e: self.e().expect("valid view"),
                        f: self.f().expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Gamma {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Gamma {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<42u64>::new();
                    if self.g.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.h.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    if let Some(ref g) = self.g {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(g).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref h) = self.h {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(h).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Gamma`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct GammaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> GammaRef<'a> {
                pub fn g(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn h(
                    &self,
                ) -> Result<
                    Optional<VariableListRef<'a, u16, 8usize>>,
                    ssz::DecodeError,
                > {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for GammaRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    let g = self.g().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&g);
                    hasher.write(root.as_ref()).expect("write field");
                    let h = self.h().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&h);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for GammaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        42usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for GammaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Gamma> for GammaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Gamma {
                    <GammaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> GammaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Gamma {
                    Gamma {
                        g: self.g().expect("valid view").to_owned(),
                        h: self.h().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Delta {
                pub z: bool,
                pub w: u8,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Delta {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.z)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.w)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Delta`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct DeltaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> DeltaRef<'a> {
                pub fn z(&self) -> Result<bool, ssz::DecodeError> {
                    let offset = 0usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn w(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 1usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for DeltaRef<'a> {
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
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 1usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for DeltaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 2usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 2usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for DeltaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    2usize
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Delta> for DeltaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Delta {
                    <DeltaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> DeltaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Delta {
                    Delta {
                        z: self.z().expect("valid view"),
                        w: self.w().expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Epsilon {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<AliasNested>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Epsilon {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<42u64>::new();
                    if self.g.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.h.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.i.is_some() {
                        active_fields
                            .set(2usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.j.is_some() {
                        active_fields
                            .set(3usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    if let Some(ref g) = self.g {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(g).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref h) = self.h {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(h).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref i) = self.i {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(i).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref j) = self.j {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(j).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Epsilon`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct EpsilonRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> EpsilonRef<'a> {
                pub fn g(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn h(
                    &self,
                ) -> Result<
                    Optional<VariableListRef<'a, u16, 8usize>>,
                    ssz::DecodeError,
                > {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn i(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        3usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn j(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        4usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for EpsilonRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    let g = self.g().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&g);
                    hasher.write(root.as_ref()).expect("write field");
                    let h = self.h().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&h);
                    hasher.write(root.as_ref()).expect("write field");
                    let i = self.i().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&i);
                    hasher.write(root.as_ref()).expect("write field");
                    let j = self.j().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&j);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for EpsilonRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        42usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for EpsilonRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Epsilon> for EpsilonRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Epsilon {
                    <EpsilonRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> EpsilonRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Epsilon {
                    Epsilon {
                        g: self.g().expect("valid view").to_owned(),
                        h: self.h().expect("valid view").to_owned(),
                        i: self.i().expect("valid view").to_owned(),
                        j: self.j().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 128usize)]
            pub struct Zeta {
                pub u: Optional<FixedBytes<16usize>>,
                pub v: Optional<AliasListAlias>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Zeta {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<128u64>::new();
                    if self.u.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.v.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(128usize);
                    if let Some(ref u) = self.u {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(u).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref v) = self.v {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(v).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Zeta`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ZetaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ZetaRef<'a> {
                pub fn u(
                    &self,
                ) -> Result<Optional<FixedBytesRef<'a, 16usize>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn v(&self) -> Result<Optional<BytesRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ZetaRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(128usize);
                    let u = self.u().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&u);
                    hasher.write(root.as_ref()).expect("write field");
                    let v = self.v().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&v);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ZetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        128usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for ZetaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Zeta> for ZetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Zeta {
                    <ZetaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ZetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Zeta {
                    Zeta {
                        u: self.u().expect("valid view").to_owned(),
                        v: self.v().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct TestType {
                pub ccc: u8,
                pub ddd: u8,
                pub eee: VariableList<u16, 3usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for TestType {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.ccc)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.ddd)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.eee)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`TestType`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct TestTypeRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestTypeRef<'a> {
                pub fn ccc(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 0usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn ddd(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 1usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn eee(
                    &self,
                ) -> Result<VariableListRef<'a, u16, 3usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        6usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        6usize,
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
            for TestTypeRef<'a> {
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
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 1usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let eee = self.eee().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&eee);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TestTypeRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 6usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 6usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            6usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 6usize {
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
            impl<'a> ssz::view::SszTypeInfo for TestTypeRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<TestType> for TestTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> TestType {
                    <TestTypeRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TestTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> TestType {
                    TestType {
                        ccc: self.ccc().expect("valid view"),
                        ddd: self.ddd().expect("valid view"),
                        eee: self
                            .eee()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Eta {
                pub l: Zeta,
                pub m: TestType,
                pub n: FirstUnion,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Eta {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.l)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.m)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.n)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Eta`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct EtaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> EtaRef<'a> {
                pub fn l(&self) -> Result<ZetaRef<'a>, ssz::DecodeError> {
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
                pub fn m(&self) -> Result<TestTypeRef<'a>, ssz::DecodeError> {
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
                pub fn n(&self) -> Result<FirstUnionRef<'a>, ssz::DecodeError> {
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
            for EtaRef<'a> {
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
                        let l = self.l().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&l);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let m = self.m().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&m);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let n = self.n().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&n);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for EtaRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for EtaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Eta> for EtaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Eta {
                    <EtaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> EtaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Eta {
                    Eta {
                        l: self.l().expect("valid view").to_owned(),
                        m: self.m().expect("valid view").to_owned(),
                        n: self.n().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Theta {
                pub o: UnionB,
                pub p: UnionC,
                pub q: AliasVecA,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Theta {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.o)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.p)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.q)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Theta`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ThetaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ThetaRef<'a> {
                pub fn o(&self) -> Result<UnionBRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        18usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        18usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn p(&self) -> Result<UnionCRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        18usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        18usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn q(&self) -> Result<FixedBytesRef<'a, 10usize>, ssz::DecodeError> {
                    let offset = 8usize;
                    let end = offset + 10usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ThetaRef<'a> {
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
                        let o = self.o().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&o);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let p = self.p().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&p);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let q = self.q().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&q);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ThetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 18usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 18usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..2usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            18usize,
                            2usize,
                            i,
                        )?;
                        if i == 0 && offset != 18usize {
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
            impl<'a> ssz::view::SszTypeInfo for ThetaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Theta> for ThetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Theta {
                    <ThetaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ThetaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Theta {
                    Theta {
                        o: self.o().expect("valid view").to_owned(),
                        p: self.p().expect("valid view").to_owned(),
                        q: ssz_types::FixedBytes(
                            self.q().expect("valid view").to_owned(),
                        ),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Iota {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<AliasNested>,
                pub r: Optional<VariableList<AliasNested, 2usize>>,
                pub s: Optional<u8>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Iota {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<42u64>::new();
                    if self.g.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.h.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.i.is_some() {
                        active_fields
                            .set(2usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.j.is_some() {
                        active_fields
                            .set(3usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.r.is_some() {
                        active_fields
                            .set(4usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.s.is_some() {
                        active_fields
                            .set(5usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    if let Some(ref g) = self.g {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(g).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref h) = self.h {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(h).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref i) = self.i {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(i).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref j) = self.j {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(j).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref r) = self.r {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(r).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref s) = self.s {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(s).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Iota`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct IotaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> IotaRef<'a> {
                pub fn g(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn h(
                    &self,
                ) -> Result<
                    Optional<VariableListRef<'a, u16, 8usize>>,
                    ssz::DecodeError,
                > {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn i(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        3usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn j(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        4usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn r(
                    &self,
                ) -> Result<
                    Optional<VariableListRef<'a, u16, 2usize>>,
                    ssz::DecodeError,
                > {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        4usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        5usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn s(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        24usize,
                        6usize,
                        5usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        24usize,
                        6usize,
                        6usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for IotaRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(42usize);
                    let g = self.g().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&g);
                    hasher.write(root.as_ref()).expect("write field");
                    let h = self.h().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&h);
                    hasher.write(root.as_ref()).expect("write field");
                    let i = self.i().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&i);
                    hasher.write(root.as_ref()).expect("write field");
                    let j = self.j().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&j);
                    hasher.write(root.as_ref()).expect("write field");
                    let r = self.r().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&r);
                    hasher.write(root.as_ref()).expect("write field");
                    let s = self.s().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&s);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for IotaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        42usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for IotaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Iota> for IotaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Iota {
                    <IotaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> IotaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Iota {
                    Iota {
                        g: self.g().expect("valid view").to_owned(),
                        h: self.h().expect("valid view").to_owned(),
                        i: self.i().expect("valid view").to_owned(),
                        j: self.j().expect("valid view").to_owned(),
                        r: self.r().expect("valid view").to_owned(),
                        s: self.s().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Kappa {
                pub t: Alpha,
                pub u: Beta,
                pub v: BitVector<64usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Kappa {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.t)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.u)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.v)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Kappa`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct KappaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> KappaRef<'a> {
                pub fn t(&self) -> Result<AlphaRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn u(&self) -> Result<BetaRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn v(&self) -> Result<BitVectorRef<'a, 64usize>, ssz::DecodeError> {
                    let offset = 8usize;
                    let end = offset + 8usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for KappaRef<'a> {
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
                        let t = self.t().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&t);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let u = self.u().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&u);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let v = self.v().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&v);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for KappaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 16usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 16usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..2usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            16usize,
                            2usize,
                            i,
                        )?;
                        if i == 0 && offset != 16usize {
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
            impl<'a> ssz::view::SszTypeInfo for KappaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Kappa> for KappaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Kappa {
                    <KappaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> KappaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Kappa {
                    Kappa {
                        t: self.t().expect("valid view").to_owned(),
                        u: self.u().expect("valid view").to_owned(),
                        v: self.v().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 4usize)]
            pub struct Lambda {
                pub w: Optional<u16>,
                pub x: Optional<u8>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Lambda {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<4u64>::new();
                    if self.w.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.x.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    if let Some(ref w) = self.w {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(w).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref x) = self.x {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(x).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Lambda`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct LambdaRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> LambdaRef<'a> {
                pub fn w(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn x(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for LambdaRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    let w = self.w().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&w);
                    hasher.write(root.as_ref()).expect("write field");
                    let x = self.x().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&x);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for LambdaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        4usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for LambdaRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Lambda> for LambdaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Lambda {
                    <LambdaRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> LambdaRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Lambda {
                    Lambda {
                        w: self.w().expect("valid view").to_owned(),
                        x: self.x().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Mu {
                pub y: Lambda,
                pub z: UnionA,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Mu {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.y)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.z)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Mu`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct MuRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MuRef<'a> {
                pub fn y(&self) -> Result<LambdaRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(&self) -> Result<UnionARef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for MuRef<'a> {
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
                        let y = self.y().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&y);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let z = self.z().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&z);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MuRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 8usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 8usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..2usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            8usize,
                            2usize,
                            i,
                        )?;
                        if i == 0 && offset != 8usize {
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
            impl<'a> ssz::view::SszTypeInfo for MuRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Mu> for MuRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Mu {
                    <MuRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> MuRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Mu {
                    Mu {
                        y: self.y().expect("valid view").to_owned(),
                        z: self.z().expect("valid view").to_owned(),
                    }
                }
            }
            pub type AliasMu = Mu;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct Nu {
                pub zz: AliasMu,
                pub aaa: FixedVector<bool, 4usize>,
                pub bbb: BitAlias,
                pub test: Option<AliasMu>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Nu {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.zz)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.aaa)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.bbb)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.test)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`Nu`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct NuRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> NuRef<'a> {
                pub fn zz(&self) -> Result<AliasMuRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        3usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        3usize,
                        1usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn aaa(
                    &self,
                ) -> Result<FixedVectorRef<'a, bool, 4usize>, ssz::DecodeError> {
                    let offset = 4usize;
                    let end = offset + 4usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn bbb(&self) -> Result<BitListRef<'a, 42usize>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        3usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
                        3usize,
                        2usize,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn test(&self) -> Result<Option<AliasMuRef<'a>>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        16usize,
                        3usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        16usize,
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
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for NuRef<'a> {
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
                        let zz = self.zz().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&zz);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let aaa = self.aaa().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&aaa);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let bbb = self.bbb().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&bbb);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let test = self.test().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&test);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for NuRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 16usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 16usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..3usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            16usize,
                            3usize,
                            i,
                        )?;
                        if i == 0 && offset != 16usize {
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
            impl<'a> ssz::view::SszTypeInfo for NuRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Nu> for NuRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> Nu {
                    <NuRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> NuRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Nu {
                    Nu {
                        zz: self.zz().expect("valid view").to_owned(),
                        aaa: self
                            .aaa()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        bbb: self.bbb().expect("valid view").to_owned(),
                        test: self.test().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
