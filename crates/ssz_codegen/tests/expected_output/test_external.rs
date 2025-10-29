#![allow(unused_imports, reason = "ssz-gen generated code")]
pub mod tests {
    pub mod input {
        pub mod test_external {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionA {
                Selector0,
                Selector1(external_ssz::A),
                Selector2(external_ssz::module_a::module_b::B),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum ExternalUnionB {
                Selector0,
                Selector1(TestA),
                Selector2(TestB),
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
        }
    }
}
