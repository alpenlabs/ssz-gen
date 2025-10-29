#![allow(unused_imports, reason = "ssz-gen generated code")]
pub mod tests {
    pub mod input {
        pub mod test_2 {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 2usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 2usize)]
            pub struct Alpha {
                pub a: Optional<u8>,
                pub b: Optional<BitList<32usize>>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 8usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct InnerBase {
                pub x: Optional<u8>,
                pub y: Optional<VariableList<u8, 4usize>>,
                pub z: Optional<BitVector<16usize>>,
                pub w: Optional<Alpha>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile1 {
                #[tree_hash(stable_index = 0usize)]
                pub x: u8,
                #[tree_hash(stable_index = 1usize)]
                pub y: Optional<VariableList<u8, 4usize>>,
                #[tree_hash(stable_index = 2usize)]
                pub z: Optional<BitVector<16usize>>,
                #[tree_hash(stable_index = 3usize)]
                pub w: Optional<Alpha>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile2 {
                #[tree_hash(stable_index = 0usize)]
                pub x: Optional<u8>,
                #[tree_hash(stable_index = 1usize)]
                pub y: VariableList<u8, 4usize>,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVector<16usize>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 2usize)]
            pub struct AlphaProfile {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<BitList<32usize>>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile3 {
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfile,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile4 {
                #[tree_hash(stable_index = 1usize)]
                pub y: VariableList<u8, 4usize>,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVector<16usize>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile5 {
                #[tree_hash(stable_index = 0usize)]
                pub x: u8,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVector<16usize>,
                #[tree_hash(stable_index = 3usize)]
                pub w: Alpha,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct ProfileProfile {
                #[tree_hash(stable_index = 0usize)]
                pub x: Optional<u8>,
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfile,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 8usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct ContainerContainer {
                pub x: Optional<u16>,
                pub y: Optional<VariableList<u8, 4usize>>,
                pub z: Optional<BitVector<16usize>>,
                pub w: Optional<Alpha>,
                pub a: Optional<u8>,
                pub b: Optional<u8>,
                pub c: Optional<u8>,
                pub d: Optional<u8>,
            }
        }
    }
}
