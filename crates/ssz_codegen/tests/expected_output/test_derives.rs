#![allow(unused_imports, reason = "ssz-gen generated code")]
pub mod tests {
    pub mod input {
        pub mod test_derives {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum TestUnion {
                Selector0(u8),
                Selector1(u16),
                Selector2(AliasUintAlias),
            }
            pub const VAL_X: u64 = 42u64;
            pub const VAL_Y: u64 = 64u64;
            pub type AliasUintAlias = u16;
            pub type AliasVecA = FixedVector<u8, 10usize>;
            #[derive(Encode, Decode, TreeHash, Clone, Debug)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Alpha {
                pub a: u8,
                pub b: u16,
                pub c: AliasVecA,
            }
            #[derive(
                Encode,
                Decode,
                TreeHash,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord
            )]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Beta {
                pub d: AliasUintAlias,
                pub e: u8,
                pub f: AliasVecA,
            }
            #[derive(Encode, Decode, TreeHash, Clone, Debug, PartialEq, Eq)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Gamma {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
            }
            #[derive(Encode, Decode, TreeHash, Clone, Debug)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Delta {
                pub z: bool,
                pub w: u8,
            }
        }
    }
}
