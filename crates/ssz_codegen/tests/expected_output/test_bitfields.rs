pub mod tests {
    pub mod input {
        pub mod test_bitfields {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash_derive::TreeHash;
            use typenum::Unsigned;
            pub const SMALL_SIZE: u64 = 1u64;
            pub const MEDIUM_SIZE: u64 = 64u64;
            pub const LARGE_SIZE: u64 = 256u64;
            pub const POWER_OF_TWO: u64 = 128u64;
            pub type TinyBitlist = BitList<typenum::U1>;
            pub type StandardBitlist = BitList<typenum::U64>;
            pub type LargeBitlist = BitList<typenum::U256>;
            pub type TinyBitvector = BitVector<typenum::U1>;
            pub type StandardBitvector = BitVector<typenum::U64>;
            pub type LargeBitvector = BitVector<typenum::U128>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct BitfieldContainer {
                pub tiny_list: TinyBitlist,
                pub std_list: StandardBitlist,
                pub large_list: LargeBitlist,
                pub tiny_vec: TinyBitvector,
                pub std_vec: StandardBitvector,
                pub large_vec: LargeBitvector,
            }
        }
    }
}
