pub mod tests {
    pub mod input {
        pub mod test_1 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasOptionUnion {
                Selector0(u8),
                Selector1(Option<u16>),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasOptionUnionRef<'a> {
                Selector0(u8),
                Selector1(Option<u16>),
            }
            impl<'a> AliasOptionUnionRef<'a> {
                pub fn to_owned(&self) -> AliasOptionUnion {
                    match self {
                        AliasOptionUnionRef::Selector0(v) => {
                            AliasOptionUnion::Selector0(*v)
                        }
                        AliasOptionUnionRef::Selector1(v) => {
                            AliasOptionUnion::Selector1(v.to_owned())
                        }
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum FirstUnion {
                Selector0(u8),
                Selector1(u16),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum FirstUnionRef<'a> {
                Selector0(u8),
                Selector1(u16),
            }
            impl<'a> FirstUnionRef<'a> {
                pub fn to_owned(&self) -> FirstUnion {
                    match self {
                        FirstUnionRef::Selector0(v) => FirstUnion::Selector0(*v),
                        FirstUnionRef::Selector1(v) => FirstUnion::Selector1(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum TestUnion {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum TestUnionRef<'a> {
                Selector0,
                Selector1(u8),
                Selector2(u16),
            }
            impl<'a> TestUnionRef<'a> {
                pub fn to_owned(&self) -> TestUnion {
                    match self {
                        TestUnionRef::Selector0 => TestUnion::Selector0,
                        TestUnionRef::Selector1(v) => TestUnion::Selector1(*v),
                        TestUnionRef::Selector2(v) => TestUnion::Selector2(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionA {
                Selector0(u8),
                Selector1(u8),
                Selector2(u16),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionARef<'a> {
                Selector0(u8),
                Selector1(u8),
                Selector2(u16),
            }
            impl<'a> UnionARef<'a> {
                pub fn to_owned(&self) -> UnionA {
                    match self {
                        UnionARef::Selector0(v) => UnionA::Selector0(*v),
                        UnionARef::Selector1(v) => UnionA::Selector1(*v),
                        UnionARef::Selector2(v) => UnionA::Selector2(*v),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionBRef<'a> {
                Selector0(u8),
                Selector1(UnionARef<'a>),
                Selector2(u32),
                Selector3(BytesRef<'a>),
            }
            impl<'a> UnionBRef<'a> {
                pub fn to_owned(&self) -> UnionB {
                    match self {
                        UnionBRef::Selector0(v) => UnionB::Selector0(*v),
                        UnionBRef::Selector1(v) => UnionB::Selector1(v.to_owned()),
                        UnionBRef::Selector2(v) => UnionB::Selector2(*v),
                        UnionBRef::Selector3(v) => UnionB::Selector3(v.to_owned()),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionC {
                Selector0(AliasUintAlias),
                Selector1(AliasUintAlias),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionCRef<'a> {
                Selector0(u16),
                Selector1(u16),
            }
            impl<'a> UnionCRef<'a> {
                pub fn to_owned(&self) -> UnionC {
                    match self {
                        UnionCRef::Selector0(v) => UnionC::Selector0(*v),
                        UnionCRef::Selector1(v) => UnionC::Selector1(*v),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionD {
                Selector0(AliasUintAlias),
                Selector1(AliasUintAlias),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum UnionDRef<'a> {
                Selector0(u16),
                Selector1(u16),
            }
            impl<'a> UnionDRef<'a> {
                pub fn to_owned(&self) -> UnionD {
                    match self {
                        UnionDRef::Selector0(v) => UnionD::Selector0(*v),
                        UnionDRef::Selector1(v) => UnionD::Selector1(*v),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct AlphaRef<'a> {
                pub a: u8,
                pub b: u16,
                pub c: FixedVectorRef<'a, u8, 10usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for AlphaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<u16>()?;
                    builder.register_type::<AliasVecB>()?;
                    let mut decoder = builder.build()?;
                    let a = decoder.decode_next_view()?;
                    let b = decoder.decode_next_view()?;
                    let c = decoder.decode_next_view()?;
                    Ok(Self { a, b, c })
                }
            }
            impl<'a> AlphaRef<'a> {
                pub fn to_owned(&self) -> Alpha {
                    Alpha {
                        a: self.a,
                        b: self.b,
                        c: self.c.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Beta {
                pub d: AliasListAlias,
                pub e: u8,
                pub f: AliasUintAlias,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct BetaRef<'a> {
                pub d: BytesRef<'a>,
                pub e: u8,
                pub f: u16,
            }
            impl<'a> ssz::view::DecodeView<'a> for BetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<AliasListAlias>()?;
                    builder.register_type::<u8>()?;
                    builder.register_type::<AliasUintAlias>()?;
                    let mut decoder = builder.build()?;
                    let d = decoder.decode_next_view()?;
                    let e = decoder.decode_next_view()?;
                    let f = decoder.decode_next_view()?;
                    Ok(Self { d, e, f })
                }
            }
            impl<'a> BetaRef<'a> {
                pub fn to_owned(&self) -> Beta {
                    Beta {
                        d: self.d.to_owned(),
                        e: self.e,
                        f: self.f,
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 42usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct Gamma {
                pub g: Optional<u8>,
                pub h: Optional<VariableList<AliasUintAlias, 8usize>>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct GammaRef<'a> {
                pub g: Optional<u8>,
                pub h: Optional<VariableListRef<'a, u16, 8usize>>,
            }
            impl<'a> DecodeView<'a> for GammaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder
                        .register_type::<
                            Optional<VariableList<AliasUintAlias, 8usize>>,
                        >()?;
                    let mut decoder = builder.build()?;
                    let g = decoder.decode_next_view()?;
                    let h = decoder.decode_next_view()?;
                    Ok(Self { g, h })
                }
            }
            impl<'a> GammaRef<'a> {
                pub fn to_owned(&self) -> Gamma {
                    Gamma {
                        g: self.g.to_owned(),
                        h: self.h.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Delta {
                pub z: bool,
                pub w: u8,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct DeltaRef<'a> {
                pub z: bool,
                pub w: u8,
            }
            impl<'a> ssz::view::DecodeView<'a> for DeltaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<bool>()?;
                    builder.register_type::<u8>()?;
                    let mut decoder = builder.build()?;
                    let z = decoder.decode_next_view()?;
                    let w = decoder.decode_next_view()?;
                    Ok(Self { z, w })
                }
            }
            impl<'a> DeltaRef<'a> {
                pub fn to_owned(&self) -> Delta {
                    Delta { z: self.z, w: self.w }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct EpsilonRef<'a> {
                pub g: Optional<u8>,
                pub h: Optional<VariableListRef<'a, u16, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<u16>,
            }
            impl<'a> DecodeView<'a> for EpsilonRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder
                        .register_type::<
                            Optional<VariableList<AliasUintAlias, 8usize>>,
                        >()?;
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<AliasNested>>()?;
                    let mut decoder = builder.build()?;
                    let g = decoder.decode_next_view()?;
                    let h = decoder.decode_next_view()?;
                    let i = decoder.decode_next_view()?;
                    let j = decoder.decode_next_view()?;
                    Ok(Self { g, h, i, j })
                }
            }
            impl<'a> EpsilonRef<'a> {
                pub fn to_owned(&self) -> Epsilon {
                    Epsilon {
                        g: self.g.to_owned(),
                        h: self.h.to_owned(),
                        i: self.i.to_owned(),
                        j: self.j.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 128usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 128usize)]
            pub struct Zeta {
                pub u: Optional<FixedVector<u8, 16usize>>,
                pub v: Optional<AliasListAlias>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 128usize)]
            pub struct ZetaRef<'a> {
                pub u: Optional<FixedVectorRef<'a, u8, 16usize>>,
                pub v: Optional<BytesRef<'a>>,
            }
            impl<'a> DecodeView<'a> for ZetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<FixedVector<u8, 16usize>>>()?;
                    builder.register_type::<Optional<AliasListAlias>>()?;
                    let mut decoder = builder.build()?;
                    let u = decoder.decode_next_view()?;
                    let v = decoder.decode_next_view()?;
                    Ok(Self { u, v })
                }
            }
            impl<'a> ZetaRef<'a> {
                pub fn to_owned(&self) -> Zeta {
                    Zeta {
                        u: self.u.to_owned(),
                        v: self.v.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct TestType {
                pub ccc: u8,
                pub ddd: u8,
                pub eee: VariableList<u16, 3usize>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct TestTypeRef<'a> {
                pub ccc: u8,
                pub ddd: u8,
                pub eee: VariableListRef<'a, u16, 3usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for TestTypeRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<u8>()?;
                    builder.register_type::<u8>()?;
                    builder.register_type::<VariableList<u16, 3usize>>()?;
                    let mut decoder = builder.build()?;
                    let ccc = decoder.decode_next_view()?;
                    let ddd = decoder.decode_next_view()?;
                    let eee = decoder.decode_next_view()?;
                    Ok(Self { ccc, ddd, eee })
                }
            }
            impl<'a> TestTypeRef<'a> {
                pub fn to_owned(&self) -> TestType {
                    TestType {
                        ccc: self.ccc,
                        ddd: self.ddd,
                        eee: self.eee.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Eta {
                pub l: Zeta,
                pub m: TestType,
                pub n: FirstUnion,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct EtaRef<'a> {
                pub l: ZetaRef<'a>,
                pub m: TestTypeRef<'a>,
                pub n: FirstUnionRef<'a>,
            }
            impl<'a> ssz::view::DecodeView<'a> for EtaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<Zeta>()?;
                    builder.register_type::<TestType>()?;
                    builder.register_type::<FirstUnion>()?;
                    let mut decoder = builder.build()?;
                    let l = decoder.decode_next_view()?;
                    let m = decoder.decode_next_view()?;
                    let n = decoder.decode_next_view()?;
                    Ok(Self { l, m, n })
                }
            }
            impl<'a> EtaRef<'a> {
                pub fn to_owned(&self) -> Eta {
                    Eta {
                        l: self.l.to_owned(),
                        m: self.m.to_owned(),
                        n: self.n.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Theta {
                pub o: UnionB,
                pub p: UnionC,
                pub q: AliasVecA,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct ThetaRef<'a> {
                pub o: UnionBRef<'a>,
                pub p: UnionCRef<'a>,
                pub q: FixedVectorRef<'a, u8, 10usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for ThetaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<UnionB>()?;
                    builder.register_type::<UnionC>()?;
                    builder.register_type::<AliasVecA>()?;
                    let mut decoder = builder.build()?;
                    let o = decoder.decode_next_view()?;
                    let p = decoder.decode_next_view()?;
                    let q = decoder.decode_next_view()?;
                    Ok(Self { o, p, q })
                }
            }
            impl<'a> ThetaRef<'a> {
                pub fn to_owned(&self) -> Theta {
                    Theta {
                        o: self.o.to_owned(),
                        p: self.p.to_owned(),
                        q: self.q.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 42usize)]
            pub struct IotaRef<'a> {
                pub g: Optional<u8>,
                pub h: Optional<VariableListRef<'a, u16, 8usize>>,
                pub i: Optional<u8>,
                pub j: Optional<u16>,
                pub r: Optional<VariableListRef<'a, u16, 2usize>>,
                pub s: Optional<u8>,
            }
            impl<'a> DecodeView<'a> for IotaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u8>>()?;
                    builder
                        .register_type::<
                            Optional<VariableList<AliasUintAlias, 8usize>>,
                        >()?;
                    builder.register_type::<Optional<u8>>()?;
                    builder.register_type::<Optional<AliasNested>>()?;
                    builder
                        .register_type::<Optional<VariableList<AliasNested, 2usize>>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    let mut decoder = builder.build()?;
                    let g = decoder.decode_next_view()?;
                    let h = decoder.decode_next_view()?;
                    let i = decoder.decode_next_view()?;
                    let j = decoder.decode_next_view()?;
                    let r = decoder.decode_next_view()?;
                    let s = decoder.decode_next_view()?;
                    Ok(Self { g, h, i, j, r, s })
                }
            }
            impl<'a> IotaRef<'a> {
                pub fn to_owned(&self) -> Iota {
                    Iota {
                        g: self.g.to_owned(),
                        h: self.h.to_owned(),
                        i: self.i.to_owned(),
                        j: self.j.to_owned(),
                        r: self.r.to_owned(),
                        s: self.s.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Kappa {
                pub t: Alpha,
                pub u: Beta,
                pub v: BitVector<64usize>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct KappaRef<'a> {
                pub t: AlphaRef<'a>,
                pub u: BetaRef<'a>,
                pub v: BitVectorRef<'a, 64usize>,
            }
            impl<'a> ssz::view::DecodeView<'a> for KappaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<Alpha>()?;
                    builder.register_type::<Beta>()?;
                    builder.register_type::<BitVector<64usize>>()?;
                    let mut decoder = builder.build()?;
                    let t = decoder.decode_next_view()?;
                    let u = decoder.decode_next_view()?;
                    let v = decoder.decode_next_view()?;
                    Ok(Self { t, u, v })
                }
            }
            impl<'a> KappaRef<'a> {
                pub fn to_owned(&self) -> Kappa {
                    Kappa {
                        t: self.t.to_owned(),
                        u: self.u.to_owned(),
                        v: self.v.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 4usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 4usize)]
            pub struct Lambda {
                pub w: Optional<u16>,
                pub x: Optional<u8>,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 4usize)]
            pub struct LambdaRef<'a> {
                pub w: Optional<u16>,
                pub x: Optional<u8>,
            }
            impl<'a> DecodeView<'a> for LambdaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = SszDecoderBuilder::new(bytes);
                    builder.register_type::<Optional<u16>>()?;
                    builder.register_type::<Optional<u8>>()?;
                    let mut decoder = builder.build()?;
                    let w = decoder.decode_next_view()?;
                    let x = decoder.decode_next_view()?;
                    Ok(Self { w, x })
                }
            }
            impl<'a> LambdaRef<'a> {
                pub fn to_owned(&self) -> Lambda {
                    Lambda {
                        w: self.w.to_owned(),
                        x: self.x.to_owned(),
                    }
                }
            }
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Mu {
                pub y: Lambda,
                pub z: UnionA,
            }
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct MuRef<'a> {
                pub y: LambdaRef<'a>,
                pub z: UnionARef<'a>,
            }
            impl<'a> ssz::view::DecodeView<'a> for MuRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<Lambda>()?;
                    builder.register_type::<UnionA>()?;
                    let mut decoder = builder.build()?;
                    let y = decoder.decode_next_view()?;
                    let z = decoder.decode_next_view()?;
                    Ok(Self { y, z })
                }
            }
            impl<'a> MuRef<'a> {
                pub fn to_owned(&self) -> Mu {
                    Mu {
                        y: self.y.to_owned(),
                        z: self.z.to_owned(),
                    }
                }
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
            #[derive(TreeHash)]
            #[tree_hash(struct_behaviour = "container")]
            pub struct NuRef<'a> {
                pub zz: MuRef<'a>,
                pub aaa: FixedVectorRef<'a, bool, 4usize>,
                pub bbb: BitListRef<'a, 42usize>,
                pub test: Option<MuRef<'a>>,
            }
            impl<'a> ssz::view::DecodeView<'a> for NuRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let mut builder = ssz::SszDecoderBuilder::new(bytes);
                    builder.register_type::<AliasMu>()?;
                    builder.register_type::<FixedVector<bool, 4usize>>()?;
                    builder.register_type::<BitAlias>()?;
                    builder.register_type::<Option<AliasMu>>()?;
                    let mut decoder = builder.build()?;
                    let zz = decoder.decode_next_view()?;
                    let aaa = decoder.decode_next_view()?;
                    let bbb = decoder.decode_next_view()?;
                    let test = decoder.decode_next_view()?;
                    Ok(Self { zz, aaa, bbb, test })
                }
            }
            impl<'a> NuRef<'a> {
                pub fn to_owned(&self) -> Nu {
                    Nu {
                        zz: self.zz.to_owned(),
                        aaa: self.aaa.to_owned(),
                        bbb: self.bbb.to_owned(),
                        test: self.test.to_owned(),
                    }
                }
            }
        }
    }
}
