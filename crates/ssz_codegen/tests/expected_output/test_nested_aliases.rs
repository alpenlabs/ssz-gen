pub mod tests {
    pub mod input {
        pub mod test_nested_aliases {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            pub const SIZE_1: u64 = 10u64;
            pub const SIZE_2: u64 = 10u64;
            pub const SIZE_3: u64 = 10u64;
            pub type A = u8;
            pub type B = A;
            pub type C = B;
            pub type D = VariableList<C, 10usize>;
            pub type E = FixedVector<D, 5usize>;
            pub type F = VariableList<A, 10usize>;
            pub type G = FixedVector<F, 10usize>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct NestedAliasContainer {
                pub field1: D,
                pub field2: E,
                pub field3: F,
                pub field4: G,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct NestedAliasContainerRef<'a> {
                pub field1: BytesRef<'a>,
                pub field2: FixedVectorRef<'a, BytesRef<'a>, 5usize>,
                pub field3: BytesRef<'a>,
                pub field4: FixedVectorRef<'a, BytesRef<'a>, 10usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for NestedAliasContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<D>()?;
                    builder.register_type::<E>()?;
                    builder.register_type::<F>()?;
                    builder.register_type::<G>()?;
                    let mut decoder = builder.build()?;
                    let field1 = decoder.decode_next_view()?;
                    let field2 = decoder.decode_next_view()?;
                    let field3 = decoder.decode_next_view()?;
                    let field4 = decoder.decode_next_view()?;
                    Ok(Self {
                        field1,
                        field2,
                        field3,
                        field4,
                    })
                }
            }
            impl<'a> NestedAliasContainerRef<'a> {
                pub fn to_owned(&self) -> NestedAliasContainer {
                    NestedAliasContainer {
                        field1: self.field1.to_owned(),
                        field2: self.field2.to_owned(),
                        field3: self.field3.to_owned(),
                        field4: self.field4.to_owned(),
                    }
                }
            }
        }
    }
}
