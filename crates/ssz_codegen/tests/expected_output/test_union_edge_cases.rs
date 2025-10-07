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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct UnionEdgeCasesRef<'a> {
                pub simple: SimpleUnionRef<'a>,
                pub nested: NestedUnionRef<'a>,
                pub complex: ComplexUnionRef<'a>,
                pub opt_simple: Option<u8>,
                pub opt_complex: Option<VariableListRef<'a, u16, 8usize>>,
                pub opt_union: Option<SimpleUnionRef<'a>>,
            }
            impl<'a> ssz::view::DecodeView<'a> for UnionEdgeCasesRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<SimpleUnion>()?;
                    builder.register_type::<NestedUnion>()?;
                    builder.register_type::<ComplexUnion>()?;
                    builder.register_type::<OptionalSimple>()?;
                    builder.register_type::<OptionalComplex>()?;
                    builder.register_type::<OptionalUnion>()?;
                    let mut decoder = builder.build()?;
                    let simple = decoder.decode_next_view()?;
                    let nested = decoder.decode_next_view()?;
                    let complex = decoder.decode_next_view()?;
                    let opt_simple = decoder.decode_next_view()?;
                    let opt_complex = decoder.decode_next_view()?;
                    let opt_union = decoder.decode_next_view()?;
                    Ok(Self {
                        simple,
                        nested,
                        complex,
                        opt_simple,
                        opt_complex,
                        opt_union,
                    })
                }
            }
            impl<'a> UnionEdgeCasesRef<'a> {
                pub fn to_owned(&self) -> UnionEdgeCases {
                    UnionEdgeCases {
                        simple: self.simple.to_owned(),
                        nested: self.nested.to_owned(),
                        complex: self.complex.to_owned(),
                        opt_simple: self.opt_simple.to_owned(),
                        opt_complex: self.opt_complex.to_owned(),
                        opt_union: self.opt_union.to_owned(),
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct AllUnionsRef<'a> {
                pub union1: SimpleUnionRef<'a>,
                pub union2: NestedUnionRef<'a>,
                pub union3: Option<u8>,
            }
            impl<'a> ssz::view::DecodeView<'a> for AllUnionsRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<SimpleUnion>()?;
                    builder.register_type::<NestedUnion>()?;
                    builder.register_type::<OptionalSimple>()?;
                    let mut decoder = builder.build()?;
                    let union1 = decoder.decode_next_view()?;
                    let union2 = decoder.decode_next_view()?;
                    let union3 = decoder.decode_next_view()?;
                    Ok(Self { union1, union2, union3 })
                }
            }
            impl<'a> AllUnionsRef<'a> {
                pub fn to_owned(&self) -> AllUnions {
                    AllUnions {
                        union1: self.union1.to_owned(),
                        union2: self.union2.to_owned(),
                        union3: self.union3.to_owned(),
                    }
                }
            }
        }
    }
}
