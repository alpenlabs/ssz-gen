pub mod tests {
    pub mod input {
        pub mod test_2 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 2usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 2usize)]
            pub struct Alpha {
                pub a: Optional<u8>,
                pub b: Optional<BitList<32usize>>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 2usize)]
            pub struct AlphaRef<'a> {
                pub a: Optional<u8>,
                pub b: Optional<BitListRef<'a, 32usize>>,
            }
            impl<'a> DecodeView<'a> for AlphaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<BitList<32usize>>>()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    Ok(Self { a, b })
                }
            }
            impl<'a> AlphaRef<'a> {
                pub fn to_owned(&self) -> Alpha {
                    Alpha {
                        a: self.a.to_owned(),
                        b: self.b.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct InnerBaseRef<'a> {
                pub x: Optional<u8>,
                pub y: Optional<BytesRef<'a>>,
                pub z: Optional<BitVectorRef<'a, 16usize>>,
                pub w: Optional<AlphaRef<'a>>,
            }
            impl<'a> DecodeView<'a> for InnerBaseRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<VariableList<u8, 4usize>>>()?;
                    builder.register_type::<Optional<BitVector<16usize>>>()?;
                    builder.register_type::<Optional<Alpha>>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { x, y, z, w })
                }
            }
            impl<'a> InnerBaseRef<'a> {
                pub fn to_owned(&self) -> InnerBase {
                    InnerBase {
                        x: self.x.to_owned(),
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                        w: self.w.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile1Ref<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub x: u8,
                #[tree_hash(stable_index = 1usize)]
                pub y: Optional<BytesRef<'a>>,
                #[tree_hash(stable_index = 2usize)]
                pub z: Optional<BitVectorRef<'a, 16usize>>,
                #[tree_hash(stable_index = 3usize)]
                pub w: Optional<AlphaRef<'a>>,
            }
            impl<'a> DecodeView<'a> for InnerProfile1Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<Optional<VariableList<u8, 4usize>>>()?;
                    builder.register_type::<Optional<BitVector<16usize>>>()?;
                    builder.register_type::<Optional<Alpha>>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { x, y, z, w })
                }
            }
            impl<'a> InnerProfile1Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile1 {
                    InnerProfile1 {
                        x: self.x,
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                        w: self.w.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile2Ref<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub x: Optional<u8>,
                #[tree_hash(stable_index = 1usize)]
                pub y: BytesRef<'a>,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVectorRef<'a, 16usize>,
            }
            impl<'a> DecodeView<'a> for InnerProfile2Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<VariableList<u8, 4usize>>()?;
                    builder.register_type::<BitVector<16usize>>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    Ok(Self { x, y, z })
                }
            }
            impl<'a> InnerProfile2Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile2 {
                    InnerProfile2 {
                        x: self.x.to_owned(),
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 2usize)]
            pub struct AlphaProfileRef<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<BitListRef<'a, 32usize>>,
            }
            impl<'a> DecodeView<'a> for AlphaProfileRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<Optional<BitList<32usize>>>()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    Ok(Self { a, b })
                }
            }
            impl<'a> AlphaProfileRef<'a> {
                pub fn to_owned(&self) -> AlphaProfile {
                    AlphaProfile {
                        a: self.a,
                        b: self.b.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile3 {
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfile,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile3Ref<'a> {
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfileRef<'a>,
            }
            impl<'a> DecodeView<'a> for InnerProfile3Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<AlphaProfile>()?;
                    let mut decoder = builder.build()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { w })
                }
            }
            impl<'a> InnerProfile3Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile3 {
                    InnerProfile3 {
                        w: self.w.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile4Ref<'a> {
                #[tree_hash(stable_index = 1usize)]
                pub y: BytesRef<'a>,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVectorRef<'a, 16usize>,
            }
            impl<'a> DecodeView<'a> for InnerProfile4Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<VariableList<u8, 4usize>>()?;
                    builder.register_type::<BitVector<16usize>>()?;
                    let mut decoder = builder.build()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    Ok(Self { y, z })
                }
            }
            impl<'a> InnerProfile4Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile4 {
                    InnerProfile4 {
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile5Ref<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub x: u8,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVectorRef<'a, 16usize>,
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaRef<'a>,
            }
            impl<'a> DecodeView<'a> for InnerProfile5Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<BitVector<16usize>>()?;
                    builder.register_type::<Alpha>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { x, z, w })
                }
            }
            impl<'a> InnerProfile5Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile5 {
                    InnerProfile5 {
                        x: self.x,
                        z: self.z.to_owned(),
                        w: self.w.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct ProfileProfileRef<'a> {
                #[tree_hash(stable_index = 0usize)]
                pub x: Optional<u8>,
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfileRef<'a>,
            }
            impl<'a> DecodeView<'a> for ProfileProfileRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<AlphaProfile>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { x, w })
                }
            }
            impl<'a> ProfileProfileRef<'a> {
                pub fn to_owned(&self) -> ProfileProfile {
                    ProfileProfile {
                        x: self.x.to_owned(),
                        w: self.w.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct ContainerContainerRef<'a> {
                pub x: Optional<u16>,
                pub y: Optional<BytesRef<'a>>,
                pub z: Optional<BitVectorRef<'a, 16usize>>,
                pub w: Optional<AlphaRef<'a>>,
                pub a: Optional<u8>,
                pub b: Optional<u8>,
                pub c: Optional<u8>,
                pub d: Optional<u8>,
            }
            impl<'a> DecodeView<'a> for ContainerContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u16>>()?;
                    builder.register_type::<Optional<VariableList<u8, 4usize>>>()?;
                    builder.register_type::<Optional<BitVector<16usize>>>()?;
                    builder.register_type::<Optional<Alpha>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    let mut decoder = builder.build()?;
                    let x = decoder.decode_next_view()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    let c = decoder.decode_next_view()?;
                    let d = decoder.decode_next_view()?;
                    Ok(Self { x, y, z, w, a, b, c, d })
                }
            }
            impl<'a> ContainerContainerRef<'a> {
                pub fn to_owned(&self) -> ContainerContainer {
                    ContainerContainer {
                        x: self.x.to_owned(),
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                        w: self.w.to_owned(),
                        a: self.a.to_owned(),
                        b: self.b.to_owned(),
                        c: self.c.to_owned(),
                        d: self.d.to_owned(),
                    }
                }
            }
        }
    }
}
