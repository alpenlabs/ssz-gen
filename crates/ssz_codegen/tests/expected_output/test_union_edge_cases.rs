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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AnotherSimpleRef<'a> {
                Selector0(bool),
                Selector1(u32),
            }
            impl<'a> AnotherSimpleRef<'a> {
                pub fn to_owned(&self) -> AnotherSimple {
                    match self {
                        AnotherSimpleRef::Selector0(v) => AnotherSimple::Selector0(*v),
                        AnotherSimpleRef::Selector1(v) => AnotherSimple::Selector1(*v),
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ComplexUnionRef<'a> {
                Selector0(BytesRef<'a>),
                Selector1(FixedVectorRef<'a, u16, 5usize>),
                Selector2(SimpleUnionRef<'a>),
                Selector3(BitVectorRef<'a, 32usize>),
            }
            impl<'a> ComplexUnionRef<'a> {
                pub fn to_owned(&self) -> ComplexUnion {
                    match self {
                        ComplexUnionRef::Selector0(v) => {
                            ComplexUnion::Selector0(v.to_owned())
                        }
                        ComplexUnionRef::Selector1(v) => {
                            ComplexUnion::Selector1(v.to_owned())
                        }
                        ComplexUnionRef::Selector2(v) => {
                            ComplexUnion::Selector2(v.to_owned())
                        }
                        ComplexUnionRef::Selector3(v) => {
                            ComplexUnion::Selector3(v.to_owned())
                        }
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedOptionalRef<'a> {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            impl<'a> MixedOptionalRef<'a> {
                pub fn to_owned(&self) -> MixedOptional {
                    match self {
                        MixedOptionalRef::Selector0 => MixedOptional::Selector0,
                        MixedOptionalRef::Selector1(v) => MixedOptional::Selector1(*v),
                        MixedOptionalRef::Selector2(v) => MixedOptional::Selector2(*v),
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum NestedUnionRef<'a> {
                Selector0(SimpleUnionRef<'a>),
                Selector1(AnotherSimpleRef<'a>),
                Selector2(u64),
            }
            impl<'a> NestedUnionRef<'a> {
                pub fn to_owned(&self) -> NestedUnion {
                    match self {
                        NestedUnionRef::Selector0(v) => {
                            NestedUnion::Selector0(v.to_owned())
                        }
                        NestedUnionRef::Selector1(v) => {
                            NestedUnion::Selector1(v.to_owned())
                        }
                        NestedUnionRef::Selector2(v) => NestedUnion::Selector2(*v),
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SimpleUnionRef<'a> {
                Selector0(u8),
                Selector1(u16),
            }
            impl<'a> SimpleUnionRef<'a> {
                pub fn to_owned(&self) -> SimpleUnion {
                    match self {
                        SimpleUnionRef::Selector0(v) => SimpleUnion::Selector0(*v),
                        SimpleUnionRef::Selector1(v) => SimpleUnion::Selector1(*v),
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
                    let simple = self.simple().expect("valid view");
                    hasher.write(simple.tree_hash_root().as_ref()).expect("write field");
                    let nested = self.nested().expect("valid view");
                    hasher.write(nested.tree_hash_root().as_ref()).expect("write field");
                    let complex = self.complex().expect("valid view");
                    hasher
                        .write(complex.tree_hash_root().as_ref())
                        .expect("write field");
                    let opt_simple = self.opt_simple().expect("valid view");
                    hasher
                        .write(opt_simple.tree_hash_root().as_ref())
                        .expect("write field");
                    let opt_complex = self.opt_complex().expect("valid view");
                    hasher
                        .write(opt_complex.tree_hash_root().as_ref())
                        .expect("write field");
                    let opt_union = self.opt_union().expect("valid view");
                    hasher
                        .write(opt_union.tree_hash_root().as_ref())
                        .expect("write field");
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
                    let union1 = self.union1().expect("valid view");
                    hasher.write(union1.tree_hash_root().as_ref()).expect("write field");
                    let union2 = self.union2().expect("valid view");
                    hasher.write(union2.tree_hash_root().as_ref()).expect("write field");
                    let union3 = self.union3().expect("valid view");
                    hasher.write(union3.tree_hash_root().as_ref()).expect("write field");
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
