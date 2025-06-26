pub mod tests {
    pub mod input {
        pub mod test_union_edge_cases {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            use typenum::Unsigned;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AnotherSimple {
                Selector0(bool),
                Selector1(u32),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ComplexUnion {
                Selector0(VariableList<u8, typenum::U10>),
                Selector1(FixedVector<u16, typenum::U5>),
                Selector2(SimpleUnion),
                Selector3(BitVector<typenum::U32>),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedOptional {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum NestedUnion {
                Selector0(SimpleUnion),
                Selector1(AnotherSimple),
                Selector2(u64),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum SimpleUnion {
                Selector0(u8),
                Selector1(u16),
            }
            pub type OptionalSimple = Option<u8>;
            pub type OptionalComplex = Option<VariableList<u16, typenum::U8>>;
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
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct AllUnions {
                pub union1: SimpleUnion,
                pub union2: NestedUnion,
                pub union3: OptionalSimple,
            }
        }
    }
}
