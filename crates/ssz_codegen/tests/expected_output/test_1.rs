use ssz_types::*;
use ssz_derive::{Encode, Decode};
use tree_hash_derive::TreeHash;
use typenum::Unsigned;
#[derive(Encode, Decode, TreeHash)]
#[ssz(enum_behaviour = "union")]
#[tree_hash(enum_behaviour = "union")]
pub enum Union_2053421441148326692 {
    Selector0(u8),
    Selector1(Union_16280175685670805160),
    Selector2(u32),
    Selector3(VariableList<u8, typenum::U12>),
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(enum_behaviour = "union")]
#[tree_hash(enum_behaviour = "union")]
pub enum Union_2591047817855673044 {
    Selector0(u16),
    Selector1(u16),
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(enum_behaviour = "union")]
#[tree_hash(enum_behaviour = "union")]
pub enum Union_9146633999768813336 {
    Selector0(u8),
    Selector1(u16),
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(enum_behaviour = "union")]
#[tree_hash(enum_behaviour = "union")]
pub enum Union_16280175685670805160 {
    Selector0(u8),
    Selector1(u8),
    Selector2(u16),
}
pub const val_x: u64 = 42u64;
pub const val_y: u64 = 64u64;
pub type alias_uint_alias = u16;
pub type alias_vec_a = FixedVector<u8, typenum::U10>;
pub type alias_vec_b = FixedVector<u8, typenum::U10>;
pub type alias_list_alias = VariableList<u8, typenum::U5>;
pub type alias_nested = u16;
pub type bit_alias = BitList<typenum::U42>;
pub type union_a = Union_16280175685670805160;
pub type union_b = Union_2053421441148326692;
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Alpha {
    pub a: u8,
    pub b: u16,
    pub c: FixedVector<u8, typenum::U10>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Beta {
    pub d: VariableList<u8, typenum::U5>,
    pub e: u8,
    pub f: u16,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
pub struct Gamma {
    pub g: Option<u8>,
    pub h: Option<VariableList<u16, typenum::U8>>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Delta {
    pub z: bool,
    pub w: u8,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
pub struct Epsilon {
    pub g: Option<u8>,
    pub h: Option<VariableList<u16, typenum::U8>>,
    pub i: Option<u8>,
    pub j: Option<u16>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U128")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U128")]
pub struct Zeta {
    pub u: Option<FixedVector<u8, typenum::U16>>,
    pub v: Option<VariableList<u8, typenum::U5>>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct TestType {
    pub ccc: u8,
    pub ddd: u8,
    pub eee: VariableList<u16, typenum::U3>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Eta {
    pub l: Zeta,
    pub m: TestType,
    pub n: Union_9146633999768813336,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Theta {
    pub o: Union_2053421441148326692,
    pub p: Union_2591047817855673044,
    pub q: FixedVector<u8, typenum::U10>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U42")]
pub struct Iota {
    pub g: Option<u8>,
    pub h: Option<VariableList<u16, typenum::U8>>,
    pub i: Option<u8>,
    pub j: Option<u16>,
    pub r: Option<VariableList<u16, typenum::U2>>,
    pub s: Option<u8>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Kappa {
    pub t: Option<Alpha>,
    pub u: Option<Beta>,
    pub v: BitVector<typenum::U64>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "stable_container", max_fields = "typenum::U4")]
#[tree_hash(struct_behaviour = "stable_container", max_fields = "typenum::U4")]
pub struct Lambda {
    pub w: Option<u16>,
    pub x: Option<u8>,
}
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Mu {
    pub y: Lambda,
    pub z: Union_16280175685670805160,
}
pub type alias_mu = Mu;
#[derive(Encode, Decode, TreeHash)]
#[ssz(struct_behaviour = "container")]
#[tree_hash(struct_behaviour = "container")]
pub struct Nu {
    pub zz: Mu,
    pub aaa: FixedVector<bool, typenum::U4>,
    pub bbb: BitList<typenum::U42>,
}
