pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_union_in_list {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test class Name(Union): syntax in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum UnionClass {
                /// First variant
                Variant1(UnionTypeAliasVariant1),
                /// Second variant
                Variant2(UnionTypeAliasVariant2),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for UnionClass {
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
                        UnionClass::Variant1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionClass::Variant2(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            pub type Variant1Ref<'a> = UnionTypeAliasVariant1Ref<'a>;
            pub type Variant2Ref<'a> = UnionTypeAliasVariant2Ref<'a>;
            #[derive(Debug, Copy, Clone)]
            pub struct UnionClassRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionClassRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<Variant1Ref<'_>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionClass: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<Variant2Ref<'_>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionClass: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionClass {
                    match self.selector() {
                        0u8 => {
                            UnionClass::Variant1({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        1u8 => {
                            UnionClass::Variant2({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionClassRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for UnionClassRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<UnionClass> for UnionClassRef<'a> {
                fn to_owned(&self) -> UnionClass {
                    <UnionClassRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionClassRef<'a> {
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
            /// Test class Name(Union): syntax with external container in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum UnionClassWithExternal {
                /// External container variant
                Deposit(external_ssz::SubjectDepositData),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionClassWithExternal {
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
                        UnionClassWithExternal::Deposit(inner) => {
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
            pub struct UnionClassWithExternalRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionClassWithExternalRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<DepositRef<'_>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionClassWithExternal: expected 0"
                                    .to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionClassWithExternal {
                    match self.selector() {
                        0u8 => {
                            UnionClassWithExternal::Deposit({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionClassWithExternalRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for UnionClassWithExternalRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<UnionClassWithExternal>
            for UnionClassWithExternalRef<'a> {
                fn to_owned(&self) -> UnionClassWithExternal {
                    <UnionClassWithExternalRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionClassWithExternalRef<'a> {
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
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(enum_behaviour = "union")]
            pub enum UnionTypeAlias {
                TypeAliasVariant1(TypeAliasVariant1),
                TypeAliasVariant2(TypeAliasVariant2),
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionTypeAlias {
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
                        UnionTypeAlias::TypeAliasVariant1(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 0u8)
                                .expect("valid selector")
                        }
                        UnionTypeAlias::TypeAliasVariant2(inner) => {
                            let root = <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(inner);
                            tree_hash::mix_in_selector_with_hasher::<H>(&root, 1u8)
                                .expect("valid selector")
                        }
                    }
                }
            }
            pub type TypeAliasVariant1Ref<'a> = UnionTypeAliasVariant1Ref<'a>;
            pub type TypeAliasVariant2Ref<'a> = UnionTypeAliasVariant2Ref<'a>;
            #[derive(Debug, Copy, Clone)]
            pub struct UnionTypeAliasRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> UnionTypeAliasRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(
                    &self,
                ) -> Result<TypeAliasVariant1Ref<'_>, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionTypeAlias: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(
                    &self,
                ) -> Result<TypeAliasVariant2Ref<'_>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for UnionTypeAlias: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> UnionTypeAlias {
                    match self.selector() {
                        0u8 => {
                            UnionTypeAlias::TypeAliasVariant1({
                                let view = self.as_selector0().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        1u8 => {
                            UnionTypeAlias::TypeAliasVariant2({
                                let view = self.as_selector1().expect("valid selector");
                                ssz_types::view::ToOwnedSsz::to_owned(&view)
                            })
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionTypeAliasRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for UnionTypeAliasRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<UnionTypeAlias>
            for UnionTypeAliasRef<'a> {
                fn to_owned(&self) -> UnionTypeAlias {
                    <UnionTypeAliasRef<'a>>::to_owned(self)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionTypeAliasRef<'a> {
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
            pub const MAX_LIST_SIZE: u64 = 65536u64;
            /// Test Union[Type1, Type2] syntax in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct UnionTypeAliasVariant1 {
                pub value: u8,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionTypeAliasVariant1 {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.value)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`UnionTypeAliasVariant1`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct UnionTypeAliasVariant1Ref<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionTypeAliasVariant1Ref<'a> {
                pub fn value(&self) -> Result<u8, ssz::DecodeError> {
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
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionTypeAliasVariant1Ref<'a> {
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
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionTypeAliasVariant1Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 1usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 1usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for UnionTypeAliasVariant1Ref<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    1usize
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<UnionTypeAliasVariant1>
            for UnionTypeAliasVariant1Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> UnionTypeAliasVariant1 {
                    <UnionTypeAliasVariant1Ref<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionTypeAliasVariant1Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> UnionTypeAliasVariant1 {
                    UnionTypeAliasVariant1 {
                        value: self.value().expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct UnionTypeAliasVariant2 {
                pub value: u16,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionTypeAliasVariant2 {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.value)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`UnionTypeAliasVariant2`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct UnionTypeAliasVariant2Ref<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionTypeAliasVariant2Ref<'a> {
                pub fn value(&self) -> Result<u16, ssz::DecodeError> {
                    let offset = 0usize;
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
            for UnionTypeAliasVariant2Ref<'a> {
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
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 2usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionTypeAliasVariant2Ref<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for UnionTypeAliasVariant2Ref<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    2usize
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<UnionTypeAliasVariant2>
            for UnionTypeAliasVariant2Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> UnionTypeAliasVariant2 {
                    <UnionTypeAliasVariant2Ref<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> UnionTypeAliasVariant2Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> UnionTypeAliasVariant2 {
                    UnionTypeAliasVariant2 {
                        value: self.value().expect("valid view"),
                    }
                }
            }
            /// Container using class Name(Union): syntax in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ContainerWithUnionClass {
                pub items: VariableList<UnionClass, 65536usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithUnionClass {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.items)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ContainerWithUnionClass`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerWithUnionClassRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionClassRef<'a> {
                pub fn items(
                    &self,
                ) -> Result<
                    ListRef<'a, UnionClassRef<'a>, 65536usize>,
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
            for ContainerWithUnionClassRef<'a> {
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
                        let items = self.items().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&items);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithUnionClassRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ContainerWithUnionClassRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ContainerWithUnionClass>
            for ContainerWithUnionClassRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ContainerWithUnionClass {
                    <ContainerWithUnionClassRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionClassRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ContainerWithUnionClass {
                    ContainerWithUnionClass {
                        items: {
                            let view = self.items().expect("valid view");
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
            /// Container using class Name(Union): syntax with external in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ContainerWithUnionClassExternal {
                pub items: VariableList<UnionClassWithExternal, 65536usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithUnionClassExternal {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.items)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ContainerWithUnionClassExternal`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerWithUnionClassExternalRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionClassExternalRef<'a> {
                pub fn items(
                    &self,
                ) -> Result<
                    ListRef<'a, UnionClassWithExternalRef<'a>, 65536usize>,
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
            for ContainerWithUnionClassExternalRef<'a> {
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
                        let items = self.items().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&items);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a>
            for ContainerWithUnionClassExternalRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ContainerWithUnionClassExternalRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ContainerWithUnionClassExternal>
            for ContainerWithUnionClassExternalRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ContainerWithUnionClassExternal {
                    <ContainerWithUnionClassExternalRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionClassExternalRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ContainerWithUnionClassExternal {
                    ContainerWithUnionClassExternal {
                        items: {
                            let view = self.items().expect("valid view");
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
            pub type TypeAliasVariant1 = UnionTypeAliasVariant1;
            pub type TypeAliasVariant2 = UnionTypeAliasVariant2;
            /// Container using Union[Type1, Type2] syntax in a List
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "container")]
            pub struct ContainerWithUnionTypeAlias {
                pub items: VariableList<UnionTypeAlias, 65536usize>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerWithUnionTypeAlias {
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
                            <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.items)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`ContainerWithUnionTypeAlias`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerWithUnionTypeAliasRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionTypeAliasRef<'a> {
                pub fn items(
                    &self,
                ) -> Result<
                    ListRef<'a, UnionTypeAliasRef<'a>, 65536usize>,
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
            for ContainerWithUnionTypeAliasRef<'a> {
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
                        let items = self.items().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&items);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithUnionTypeAliasRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ContainerWithUnionTypeAliasRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<ContainerWithUnionTypeAlias>
            for ContainerWithUnionTypeAliasRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> ContainerWithUnionTypeAlias {
                    <ContainerWithUnionTypeAliasRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ContainerWithUnionTypeAliasRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> ContainerWithUnionTypeAlias {
                    ContainerWithUnionTypeAlias {
                        items: {
                            let view = self.items().expect("valid view");
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
