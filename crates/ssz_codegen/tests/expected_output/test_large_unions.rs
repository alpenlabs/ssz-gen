pub mod tests {
    pub mod input {
        pub mod test_large_unions {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            use typenum::Unsigned;
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
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum MixedUnion {
                Selector0(u8),
                Selector1(VariableList<u8, typenum::U5>),
                Selector2(FixedVector<u16, typenum::U3>),
                Selector3(BitVector<typenum::U8>),
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
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ContainerWithBigUnions {
                pub big: BigUnion,
                pub same: SameTypeUnion,
                pub mixed: MixedUnion,
            }
        }
    }
}
