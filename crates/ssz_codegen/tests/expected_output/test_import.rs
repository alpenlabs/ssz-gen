pub mod tests {
    pub mod input {
        pub mod test_common {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0(u8),
                Selector1(AliasUnion),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnionRef<'a> {
                Selector0(u8),
                Selector1(Option<u8>),
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self {
                        AliasUnionUnionRef::Selector0(v) => {
                            AliasUnionUnion::Selector0(*v)
                        }
                        AliasUnionUnionRef::Selector1(v) => {
                            AliasUnionUnion::Selector1(v.to_owned())
                        }
                    }
                }
            }
            pub const CONSTANT_VALUE: u64 = 5u64;
            pub type AliasUint8 = u8;
            pub type AliasAliasUint8 = AliasUint8;
            pub type AliasUnion = Option<u8>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<u8>,
                pub b: Optional<AliasUnion>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClassRef<'a> {
                pub a: Optional<u8>,
                pub b: Optional<Option<u8>>,
            }
            impl<'a> DecodeView<'a> for StableContainerClassRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<AliasUnion>>()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    Ok(Self { a, b })
                }
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn to_owned(&self) -> StableContainerClass {
                    StableContainerClass {
                        a: self.a.to_owned(),
                        b: self.b.to_owned(),
                    }
                }
            }
        }
        pub mod test_import_1 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0,
                Selector1(crate::tests::input::test_common::AliasUint8),
                Selector2(crate::tests::input::test_common::AliasUnion),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnionRef<'a> {
                Selector0,
                Selector1(u8),
                Selector2(Option<u8>),
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self {
                        AliasUnionUnionRef::Selector0 => AliasUnionUnion::Selector0,
                        AliasUnionUnionRef::Selector1(v) => {
                            AliasUnionUnion::Selector1(*v)
                        }
                        AliasUnionUnionRef::Selector2(v) => {
                            AliasUnionUnion::Selector2(v.to_owned())
                        }
                    }
                }
            }
            pub const CONSTANT_VALUE: u64 = 5u64;
            pub const CONSTANT_VALUE_2: u64 = 5u64;
            pub const CONSTANT_VALUE_IMPORTED: u64 = crate::tests::input::test_common::CONSTANT_VALUE;
            pub type AliasListImportedLength = VariableList<
                crate::tests::input::test_common::AliasUint8,
                5usize,
            >;
            pub type AliasListImportedConstant = VariableList<
                crate::tests::input::test_common::AliasUint8,
                5usize,
            >;
            pub type AliasClassStableContainer = crate::tests::input::test_common::StableContainerClass;
            pub type AliasUint8 = crate::tests::input::test_common::AliasUint8;
            pub type AliasAliasUint8 = crate::tests::input::test_common::AliasAliasUint8;
            pub type AliasUnion = crate::tests::input::test_common::AliasUnion;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<crate::tests::input::test_common::AliasUint8>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClassRef<'a> {
                pub a: Optional<u8>,
            }
            impl<'a> DecodeView<'a> for StableContainerClassRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder
                        .register_type::<
                            Optional<crate::tests::input::test_common::AliasUint8>,
                        >()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    Ok(Self { a })
                }
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn to_owned(&self) -> StableContainerClass {
                    StableContainerClass {
                        a: self.a.to_owned(),
                    }
                }
            }
        }
        pub mod test_import_2 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct ProfileInehritance {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct ProfileInehritanceRef<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
            impl<'a> DecodeView<'a> for ProfileInehritanceRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<Optional<Option<u8>>>()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    Ok(Self { a, b })
                }
            }
            impl<'a> ProfileInehritanceRef<'a> {
                pub fn to_owned(&self) -> ProfileInehritance {
                    ProfileInehritance {
                        a: self.a,
                        b: self.b.to_owned(),
                    }
                }
            }
        }
    }
}
