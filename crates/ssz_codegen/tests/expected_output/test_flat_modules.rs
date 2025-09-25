pub mod test_1 {
    use ssz_types::*;
    use ssz_derive::{Encode, Decode};
    use tree_hash::TreeHashDigest;
    use tree_hash_derive::TreeHash;
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum AliasOptionUnion {
        Selector0(u8),
        Selector1(Option<u16>),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum FirstUnion {
        Selector0(u8),
        Selector1(u16),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum TestUnion {
        Selector0,
        Selector1(u8),
        Selector2(u16),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum UnionA {
        Selector0(u8),
        Selector1(u8),
        Selector2(u16),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum UnionB {
        Selector0(u8),
        Selector1(UnionA),
        Selector2(u32),
        Selector3(VariableList<u8, 12usize>),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum UnionC {
        Selector0(AliasUintAlias),
        Selector1(AliasUintAlias),
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(enum_behaviour = "union")]
    #[tree_hash(enum_behaviour = "union")]
    pub enum UnionD {
        Selector0(AliasUintAlias),
        Selector1(AliasUintAlias),
    }
    pub const VAL_X: u64 = 42u64;
    pub const VAL_Y: u64 = 64u64;
    pub const SIZE_ALIAS: u64 = 64u64;
    pub type AliasUintAlias = u16;
    pub type AliasVecA = FixedVector<u8, 10usize>;
    pub type AliasVecB = AliasVecA;
    pub type AliasListAlias = VariableList<u8, 5usize>;
    pub type AliasNested = AliasUintAlias;
    pub type BitAlias = BitList<42usize>;
    pub type UnionE = UnionD;
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Alpha {
        pub a: u8,
        pub b: u16,
        pub c: AliasVecB,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Beta {
        pub d: AliasListAlias,
        pub e: u8,
        pub f: AliasUintAlias,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
    #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
    pub struct Gamma {
        pub g: Optional<u8>,
        pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Delta {
        pub z: bool,
        pub w: u8,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
    #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
    pub struct Epsilon {
        pub g: Optional<u8>,
        pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
        pub i: Optional<u8>,
        pub j: Optional<AliasNested>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "stable_container", max_fields = 128usize)]
    #[tree_hash(struct_behaviour = "stable_container", max_fields = 128usize)]
    pub struct Zeta {
        pub u: Optional<FixedVector<u8, 16usize>>,
        pub v: Optional<AliasListAlias>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct TestType {
        pub ccc: u8,
        pub ddd: u8,
        pub eee: VariableList<u16, 3usize>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Eta {
        pub l: Zeta,
        pub m: TestType,
        pub n: FirstUnion,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Theta {
        pub o: UnionB,
        pub p: UnionC,
        pub q: AliasVecA,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
    #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
    pub struct Iota {
        pub g: Optional<u8>,
        pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
        pub i: Optional<u8>,
        pub j: Optional<AliasNested>,
        pub r: Optional<VariableList<AliasNested, 2usize>>,
        pub s: Optional<u8>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Kappa {
        pub t: Alpha,
        pub u: Beta,
        pub v: BitVector<64usize>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "stable_container", max_fields = 4usize)]
    #[tree_hash(struct_behaviour = "stable_container", max_fields = 4usize)]
    pub struct Lambda {
        pub w: Optional<u16>,
        pub x: Optional<u8>,
    }
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Mu {
        pub y: Lambda,
        pub z: UnionA,
    }
    pub type AliasMu = Mu;
    #[derive(Encode, Decode, TreeHash)]
    #[ssz(struct_behaviour = "container")]
    #[tree_hash(struct_behaviour = "container")]
    pub struct Nu {
        pub zz: AliasMu,
        pub aaa: FixedVector<bool, 4usize>,
        pub bbb: BitAlias,
        pub test: Option<AliasMu>,
    }
}
