pub mod tests {
    pub mod input {
        pub mod test_common {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum alias_union_union {
                Selector0(u8),
                Selector1(alias_union),
            }
            pub const constant_value: u64 = 5u64;
            pub type alias_uint8 = u8;
            pub type alias_alias_uint8 = alias_uint8;
            pub type alias_union = Option<u8>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct stable_container {
                pub a: Optional<u8>,
                pub b: Optional<alias_union>,
            }
        }
        pub mod test_import_1 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum alias_union_union {
                Selector0,
                Selector1(crate::tests::input::test_common::alias_uint8),
                Selector2(crate::tests::input::test_common::alias_union),
            }
            pub const constant_value: u64 = 5u64;
            pub const constant_value_2: u64 = 5u64;
            pub const constant_value_imported: u64 = crate::tests::input::test_common::constant_value;
            pub type alias_list_imported_length = VariableList<
                crate::tests::input::test_common::alias_uint8,
                5usize,
            >;
            pub type alias_list_imported_constant = VariableList<
                crate::tests::input::test_common::alias_uint8,
                5usize,
            >;
            pub type alias_class_stable_container = crate::tests::input::test_common::stable_container;
            pub type alias_uint8 = crate::tests::input::test_common::alias_uint8;
            pub type alias_alias_uint8 = crate::tests::input::test_common::alias_alias_uint8;
            pub type alias_union = crate::tests::input::test_common::alias_union;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct stable_container {
                pub a: Optional<crate::tests::input::test_common::alias_uint8>,
            }
        }
        pub mod test_import_2 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct profile_inehritance {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
        }
    }
}
