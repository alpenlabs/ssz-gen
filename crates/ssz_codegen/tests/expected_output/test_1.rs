pub mod tests {
    pub mod input {
        pub mod test_1 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum alias_option_union {
                Selector0(u8),
                Selector1(Option<u16>),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum first_union {
                Selector0(u8),
                Selector1(u16),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum test_union {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum union_a {
                Selector0(u8),
                Selector1(u8),
                Selector2(u16),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum union_b {
                Selector0(u8),
                Selector1(union_a),
                Selector2(u32),
                Selector3(VariableList<u8, 12usize>),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum union_c {
                Selector0(alias_uint_alias),
                Selector1(alias_uint_alias),
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum union_d {
                Selector0(alias_uint_alias),
                Selector1(alias_uint_alias),
            }
            pub const val_x: u64 = 42u64;
            pub const val_y: u64 = 64u64;
            pub const size_alias: u64 = 64u64;
            pub type alias_uint_alias = u16;
            pub type alias_vec_a = FixedVector<u8, 10usize>;
            pub type alias_vec_b = alias_vec_a;
            pub type alias_list_alias = VariableList<u8, 5usize>;
            pub type alias_nested = alias_uint_alias;
            pub type bit_alias = BitList<42usize>;
            pub type union_e = union_d;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Alpha {
                pub a: u8,
                pub b: u16,
                pub c: alias_vec_b,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Beta {
                pub d: alias_list_alias,
                pub e: u8,
                pub f: alias_uint_alias,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Gamma {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<alias_uint_alias, 8usize>>,
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
                pub h: Optional<VariableList<alias_uint_alias, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<alias_nested>,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 128usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 128usize)]
            pub struct Zeta {
                pub u: Optional<FixedVector<u8, 16usize>>,
                pub v: Optional<alias_list_alias>,
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
                pub n: first_union,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Theta {
                pub o: union_b,
                pub p: union_c,
                pub q: alias_vec_a,
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Iota {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<alias_uint_alias, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<alias_nested>,
                pub r: Optional<VariableList<alias_nested, 2usize>>,
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
                pub z: union_a,
            }
            pub type alias_mu = Mu;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Nu {
                pub zz: alias_mu,
                pub aaa: FixedVector<bool, 4usize>,
                pub bbb: bit_alias,
                pub test: Option<alias_mu>,
            }
        }
    }
}
