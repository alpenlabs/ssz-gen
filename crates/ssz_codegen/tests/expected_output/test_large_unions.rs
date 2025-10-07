pub mod tests {
    pub mod input {
        pub mod test_large_unions {
            use ssz_types::*;
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum BigUnionRef<'a> {
                Selector0(u8),
                Selector1(u16),
                Selector2(u32),
                Selector3(u64),
                Selector4(u128),
                Selector5(u256),
            }
            impl<'a> BigUnionRef<'a> {
                pub fn to_owned(&self) -> BigUnion {
                    match self {
                        BigUnionRef::Selector0(v) => BigUnion::Selector0(*v),
                        BigUnionRef::Selector1(v) => BigUnion::Selector1(*v),
                        BigUnionRef::Selector2(v) => BigUnion::Selector2(*v),
                        BigUnionRef::Selector3(v) => BigUnion::Selector3(*v),
                        BigUnionRef::Selector4(v) => BigUnion::Selector4(*v),
                        BigUnionRef::Selector5(v) => BigUnion::Selector5(*v),
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedUnionRef<'a> {
                Selector0(u8),
                Selector1(BytesRef<'a>),
                Selector2(FixedVectorRef<'a, u16, 3usize>),
                Selector3(BitVectorRef<'a, 8usize>),
            }
            impl<'a> MixedUnionRef<'a> {
                pub fn to_owned(&self) -> MixedUnion {
                    match self {
                        MixedUnionRef::Selector0(v) => MixedUnion::Selector0(*v),
                        MixedUnionRef::Selector1(v) => {
                            MixedUnion::Selector1(v.to_owned())
                        }
                        MixedUnionRef::Selector2(v) => {
                            MixedUnion::Selector2(v.to_owned())
                        }
                        MixedUnionRef::Selector3(v) => {
                            MixedUnion::Selector3(v.to_owned())
                        }
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SameTypeUnionRef<'a> {
                Selector0(u8),
                Selector1(u8),
                Selector2(u8),
                Selector3(u8),
            }
            impl<'a> SameTypeUnionRef<'a> {
                pub fn to_owned(&self) -> SameTypeUnion {
                    match self {
                        SameTypeUnionRef::Selector0(v) => SameTypeUnion::Selector0(*v),
                        SameTypeUnionRef::Selector1(v) => SameTypeUnion::Selector1(*v),
                        SameTypeUnionRef::Selector2(v) => SameTypeUnion::Selector2(*v),
                        SameTypeUnionRef::Selector3(v) => SameTypeUnion::Selector3(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ContainerWithBigUnions {
                pub big: BigUnion,
                pub same: SameTypeUnion,
                pub mixed: MixedUnion,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ContainerWithBigUnionsRef<'a> {
                pub big: BigUnionRef<'a>,
                pub same: SameTypeUnionRef<'a>,
                pub mixed: MixedUnionRef<'a>,
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerWithBigUnionsRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<BigUnion>()?;
                    builder.register_type::<SameTypeUnion>()?;
                    builder.register_type::<MixedUnion>()?;
                    let mut decoder = builder.build()?;
                    let big = decoder.decode_next_view()?;
                    let same = decoder.decode_next_view()?;
                    let mixed = decoder.decode_next_view()?;
                    Ok(Self { big, same, mixed })
                }
            }
            impl<'a> ContainerWithBigUnionsRef<'a> {
                pub fn to_owned(&self) -> ContainerWithBigUnions {
                    ContainerWithBigUnions {
                        big: self.big.to_owned(),
                        same: self.same.to_owned(),
                        mixed: self.mixed.to_owned(),
                    }
                }
            }
        }
    }
}
