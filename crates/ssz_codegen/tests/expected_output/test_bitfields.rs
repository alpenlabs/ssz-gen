pub mod tests {
    pub mod input {
        pub mod test_bitfields {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct BitfieldContainerRef<'a> {
                pub tiny_list: BitListRef<'a, 1usize>,
                pub std_list: BitListRef<'a, 64usize>,
                pub large_list: BitListRef<'a, 256usize>,
                pub tiny_vec: BitVectorRef<'a, 1usize>,
                pub std_vec: BitVectorRef<'a, 64usize>,
                pub large_vec: BitVectorRef<'a, 128usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for BitfieldContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<TinyBitlist>()?;
                    builder.register_type::<StandardBitlist>()?;
                    builder.register_type::<LargeBitlist>()?;
                    builder.register_type::<TinyBitvector>()?;
                    builder.register_type::<StandardBitvector>()?;
                    builder.register_type::<LargeBitvector>()?;
                    let mut decoder = builder.build()?;
                    let tiny_list = decoder.decode_next_view()?;
                    let std_list = decoder.decode_next_view()?;
                    let large_list = decoder.decode_next_view()?;
                    let tiny_vec = decoder.decode_next_view()?;
                    let std_vec = decoder.decode_next_view()?;
                    let large_vec = decoder.decode_next_view()?;
                    Ok(Self {
                        tiny_list,
                        std_list,
                        large_list,
                        tiny_vec,
                        std_vec,
                        large_vec,
                    })
                }
            }
            impl<'a> BitfieldContainerRef<'a> {
                pub fn to_owned(&self) -> BitfieldContainer {
                    BitfieldContainer {
                        tiny_list: self.tiny_list.to_owned(),
                        std_list: self.std_list.to_owned(),
                        large_list: self.large_list.to_owned(),
                        tiny_vec: self.tiny_vec.to_owned(),
                        std_vec: self.std_vec.to_owned(),
                        large_vec: self.large_vec.to_owned(),
                    }
                }
            }
        }
    }
}
