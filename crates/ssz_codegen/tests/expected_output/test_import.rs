#![allow(unused_imports, reason = "ssz-gen generated code")]
pub mod tests {
    pub mod input {
        pub mod test_common {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0(u8),
                Selector1(AliasUnion),
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
        }
        pub mod test_import_1 {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0,
                Selector1(crate::tests::input::test_common::AliasUint8),
                Selector2(crate::tests::input::test_common::AliasUnion),
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
        }
        pub mod test_import_2 {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct ProfileInehritance {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
        }
    }
}
