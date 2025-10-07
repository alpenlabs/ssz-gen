pub mod tests {
    pub mod input {
        pub mod test_external {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionA {
                Selector0,
                Selector1(external_ssz::A),
                Selector2(external_ssz::module_a::module_b::B),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionARef<'a> {
                Selector0,
                Selector1(external_ssz::A),
                Selector2(external_ssz::module_a::module_b::B),
            }
            impl<'a> ExternalUnionARef<'a> {
                pub fn to_owned(&self) -> ExternalUnionA {
                    match self {
                        ExternalUnionARef::Selector0 => ExternalUnionA::Selector0,
                        ExternalUnionARef::Selector1(v) => {
                            ExternalUnionA::Selector1(v.to_owned())
                        }
                        ExternalUnionARef::Selector2(v) => {
                            ExternalUnionA::Selector2(v.to_owned())
                        }
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionB {
                Selector0,
                Selector1(TestA),
                Selector2(TestB),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionBRef<'a> {
                Selector0,
                Selector1(TestA),
                Selector2(TestB),
            }
            impl<'a> ExternalUnionBRef<'a> {
                pub fn to_owned(&self) -> ExternalUnionB {
                    match self {
                        ExternalUnionBRef::Selector0 => ExternalUnionB::Selector0,
                        ExternalUnionBRef::Selector1(v) => {
                            ExternalUnionB::Selector1(v.to_owned())
                        }
                        ExternalUnionBRef::Selector2(v) => {
                            ExternalUnionB::Selector2(v.to_owned())
                        }
                    }
                }
            }
            pub type TestA = external_ssz::A;
            pub type TestB = external_ssz::module_a::module_b::B;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ExternalContainer {
                pub field_a: external_ssz::A,
                pub field_b: external_ssz::module_a::module_b::B,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ExternalContainerRef<'a> {
                pub field_a: external_ssz::A,
                pub field_b: external_ssz::module_a::module_b::B,
            }
            impl<'a> ssz::view::DecodeView<'a> for ExternalContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<external_ssz::A>()?;
                    builder.register_type::<external_ssz::module_a::module_b::B>()?;
                    let mut decoder = builder.build()?;
                    let field_a = decoder.decode_next_view()?;
                    let field_b = decoder.decode_next_view()?;
                    Ok(Self { field_a, field_b })
                }
            }
            impl<'a> ExternalContainerRef<'a> {
                pub fn to_owned(&self) -> ExternalContainer {
                    ExternalContainer {
                        field_a: self.field_a.to_owned(),
                        field_b: self.field_b.to_owned(),
                    }
                }
            }
        }
    }
}
