use ssz_types::*;
use ssz_derive::{Encode, Decode};
use tree_hash_derive::TreeHash;
use typenum::Unsigned;
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U2")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U2")]
pub struct Alpha {
    pub a: Optional<u8>,
    pub b: Optional<BitList<typenum::U32>>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U8")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U8")]
pub struct InnerBase {
    pub x: Optional<u8>,
    pub y: Optional<VariableList<u8, typenum::U4>>,
    pub z: Optional<BitVector<typenum::U16>>,
    pub w: Optional<Alpha>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct InnerProfile1 {
    #[tree_hash(stable_index = 0usize)]
    pub x: u8,
    #[tree_hash(stable_index = 1usize)]
    pub y: Optional<VariableList<u8, typenum::U4>>,
    #[tree_hash(stable_index = 2usize)]
    pub z: Optional<BitVector<typenum::U16>>,
    #[tree_hash(stable_index = 3usize)]
    pub w: Optional<Alpha>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct InnerProfile2 {
    #[tree_hash(stable_index = 0usize)]
    pub x: Optional<u8>,
    #[tree_hash(stable_index = 1usize)]
    pub y: VariableList<u8, typenum::U4>,
    #[tree_hash(stable_index = 2usize)]
    pub z: BitVector<typenum::U16>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U2")]
pub struct AlphaProfile {
    #[tree_hash(stable_index = 0usize)]
    pub a: u8,
    #[tree_hash(stable_index = 1usize)]
    pub b: Optional<BitList<typenum::U32>>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct InnerProfile3 {
    #[tree_hash(stable_index = 3usize)]
    pub w: AlphaProfile,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct InnerProfile4 {
    #[tree_hash(stable_index = 1usize)]
    pub y: VariableList<u8, typenum::U4>,
    #[tree_hash(stable_index = 2usize)]
    pub z: BitVector<typenum::U16>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct InnerProfile5 {
    #[tree_hash(stable_index = 0usize)]
    pub x: u8,
    #[tree_hash(stable_index = 2usize)]
    pub z: BitVector<typenum::U16>,
    #[tree_hash(stable_index = 3usize)]
    pub w: Alpha,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "profile")]
#[tree_hash(struct_behaviour = "profile", max_fields = "typenum::U8")]
pub struct ProfileProfile {
    #[tree_hash(stable_index = 0usize)]
    pub x: Optional<u8>,
    #[tree_hash(stable_index = 3usize)]
    pub w: AlphaProfile,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U8")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U8")]
pub struct ContainerContainer {
    pub x: Optional<u16>,
    pub y: Optional<VariableList<u8, typenum::U4>>,
    pub z: Optional<BitVector<typenum::U16>>,
    pub w: Optional<Alpha>,
    pub a: Optional<u8>,
    pub b: Optional<u8>,
    pub c: Optional<u8>,
    pub d: Optional<u8>,
}
