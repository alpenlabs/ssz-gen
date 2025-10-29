pub mod tests {
    pub mod input {
        pub mod test_bitfields {
            #![allow(unused_imports, reason = "ssz-gen generated code")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            pub const SMALL_SIZE: u64 = 1u64;
            pub const MEDIUM_SIZE: u64 = 64u64;
            pub const LARGE_SIZE: u64 = 256u64;
            pub const POWER_OF_TWO: u64 = 128u64;
            pub type TinyBitlist = BitList<1usize>;
            pub type StandardBitlist = BitList<64usize>;
            pub type LargeBitlist = BitList<256usize>;
            pub type TinyBitvector = BitVector<1usize>;
            pub type StandardBitvector = BitVector<64usize>;
            pub type LargeBitvector = BitVector<128usize>;
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
