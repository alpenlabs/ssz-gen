pub mod tests {
    pub mod input {
        pub mod test_nested_aliases {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
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
        }
    }
}
