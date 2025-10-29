pub mod tests {
    pub mod input {
        pub mod test_union_edge_cases {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AnotherSimple {
                Selector0(bool),
                Selector1(u32),
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
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ComplexUnion {
                Selector0(VariableList<u8, 10usize>),
                Selector1(FixedVector<u16, 5usize>),
                Selector2(SimpleUnion),
                Selector3(BitVector<32usize>),
            }
            #[derive(Debug, Copy, Clone)]
            pub struct ComplexUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ComplexUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
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
                            ComplexUnion::Selector0(
                                self.as_selector0().expect("valid selector").to_owned(),
                            )
                        }
                        1u8 => {
                            ComplexUnion::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        2u8 => {
                            ComplexUnion::Selector2(
                                self.as_selector2().expect("valid selector").to_owned(),
                            )
                        }
                        3u8 => {
                            ComplexUnion::Selector3(
                                self.as_selector3().expect("valid selector").to_owned(),
                            )
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
            pub enum MixedOptional {
                Selector0,
                Selector1(u8),
                Selector2(u16),
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
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
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
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum NestedUnion {
                Selector0(SimpleUnion),
                Selector1(AnotherSimple),
                Selector2(u64),
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
                            NestedUnion::Selector0(
                                self.as_selector0().expect("valid selector").to_owned(),
                            )
                        }
                        1u8 => {
                            NestedUnion::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
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
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SimpleUnion {
                Selector0(u8),
                Selector1(u16),
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
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            pub type OptionalSimple = Option<u8>;
            pub type OptionalComplex = Option<VariableList<u16, 8usize>>;
            pub type OptionalUnion = Option<SimpleUnion>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct UnionEdgeCases {
                pub simple: SimpleUnion,
                pub nested: NestedUnion,
                pub complex: ComplexUnion,
                pub opt_simple: OptionalSimple,
                pub opt_complex: OptionalComplex,
                pub opt_union: OptionalUnion,
            }
            /**Zero-copy view over [`UnionEdgeCases`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Debug, Copy, Clone)]
            pub struct UnionEdgeCasesRef<'a> {
                bytes: &'a [u8],
            }
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
                        0usize + 1,
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
                        1usize + 1,
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
                        2usize + 1,
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
                        3usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn opt_complex(
                    &self,
                ) -> Result<Option<VariableListRef<'a, u16, 8usize>>, ssz::DecodeError> {
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
                        4usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
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
                        5usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for UnionEdgeCasesRef<'a> {
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
                        let simple = self.simple().expect("valid view");
                        hasher
                            .write(simple.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let nested = self.nested().expect("valid view");
                        hasher
                            .write(nested.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let complex = self.complex().expect("valid view");
                        hasher
                            .write(complex.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let opt_simple = self.opt_simple().expect("valid view");
                        hasher
                            .write(opt_simple.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let opt_complex = self.opt_complex().expect("valid view");
                        hasher
                            .write(opt_complex.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let opt_union = self.opt_union().expect("valid view");
                        hasher
                            .write(opt_union.tree_hash_root().as_ref())
                            .expect("write field");
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
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> UnionEdgeCasesRef<'a> {
                pub fn to_owned(&self) -> UnionEdgeCases {
                    UnionEdgeCases {
                        simple: self.simple().expect("valid view").to_owned(),
                        nested: self.nested().expect("valid view").to_owned(),
                        complex: self.complex().expect("valid view").to_owned(),
                        opt_simple: self.opt_simple().expect("valid view").to_owned(),
                        opt_complex: self.opt_complex().expect("valid view").to_owned(),
                        opt_union: self.opt_union().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct AllUnions {
                pub union1: SimpleUnion,
                pub union2: NestedUnion,
                pub union3: OptionalSimple,
            }
            /**Zero-copy view over [`AllUnions`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Debug, Copy, Clone)]
            pub struct AllUnionsRef<'a> {
                bytes: &'a [u8],
            }
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
                        0usize + 1,
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
                        1usize + 1,
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
                        2usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AllUnionsRef<'a> {
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
                        let union1 = self.union1().expect("valid view");
                        hasher
                            .write(union1.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let union2 = self.union2().expect("valid view");
                        hasher
                            .write(union2.tree_hash_root().as_ref())
                            .expect("write field");
                    }
                    {
                        let union3 = self.union3().expect("valid view");
                        hasher
                            .write(union3.tree_hash_root().as_ref())
                            .expect("write field");
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
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> AllUnionsRef<'a> {
                pub fn to_owned(&self) -> AllUnions {
                    AllUnions {
                        union1: self.union1().expect("valid view").to_owned(),
                        union2: self.union2().expect("valid view").to_owned(),
                        union3: self.union3().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
