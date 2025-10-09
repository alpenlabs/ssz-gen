pub mod tests {
    pub mod input {
        pub mod test_common {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0(u8),
                Selector1(AliasUnion),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnionRef<'a> {
                Selector0(u8),
                Selector1(Option<u8>),
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self {
                        AliasUnionUnionRef::Selector0(v) => {
                            AliasUnionUnion::Selector0(*v)
                        }
                        AliasUnionUnionRef::Selector1(v) => {
                            AliasUnionUnion::Selector1(v.to_owned())
                        }
                    }
                }
            }
            pub const CONSTANT_VALUE: u64 = 5u64;
            pub type AliasUint8 = u8;
            pub type AliasAliasUint8 = AliasUint8;
            pub type AliasUnion = Option<u8>;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<u8>,
                pub b: Optional<AliasUnion>,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct StableContainerClassRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<Optional<Option<u8>>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        8usize,
                        2usize,
                        1usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for StableContainerClassRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    let a = self.a().expect("valid view");
                    hasher.write(a.tree_hash_root().as_ref()).expect("write field");
                    let b = self.b().expect("valid view");
                    hasher.write(b.tree_hash_root().as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for StableContainerClassRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    Ok(Self { bytes })
                }
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn to_owned(&self) -> StableContainerClass {
                    StableContainerClass {
                        a: self.a().expect("valid view").to_owned(),
                        b: self.b().expect("valid view").to_owned(),
                    }
                }
            }
        }
        pub mod test_import_1 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(enum_behaviour = "union")]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnion {
                Selector0,
                Selector1(crate::tests::input::test_common::AliasUint8),
                Selector2(crate::tests::input::test_common::AliasUnion),
            }
            #[derive(TreeHash)]
            #[tree_hash(enum_behaviour = "union")]
            pub enum AliasUnionUnionRef<'a> {
                Selector0,
                Selector1(u8),
                Selector2(Option<u8>),
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self {
                        AliasUnionUnionRef::Selector0 => AliasUnionUnion::Selector0,
                        AliasUnionUnionRef::Selector1(v) => {
                            AliasUnionUnion::Selector1(*v)
                        }
                        AliasUnionUnionRef::Selector2(v) => {
                            AliasUnionUnion::Selector2(v.to_owned())
                        }
                    }
                }
            }
            pub const CONSTANT_VALUE: u64 = 5u64;
            pub const CONSTANT_VALUE_2: u64 = 5u64;
            pub const CONSTANT_VALUE_IMPORTED: u64 = crate::tests::input::test_common::CONSTANT_VALUE;
            pub type AliasListImportedLength = VariableList<
                crate::tests::input::test_common::AliasUint8,
                5usize,
            >;
            pub type AliasListImportedConstant = VariableList<
                crate::tests::input::test_common::AliasUint8,
                5usize,
            >;
            pub type AliasClassStableContainer = crate::tests::input::test_common::StableContainerClass;
            pub type AliasUint8 = crate::tests::input::test_common::AliasUint8;
            pub type AliasAliasUint8 = crate::tests::input::test_common::AliasAliasUint8;
            pub type AliasUnion = crate::tests::input::test_common::AliasUnion;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<crate::tests::input::test_common::AliasUint8>,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct StableContainerClassRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        4usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        4usize,
                        1usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for StableContainerClassRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    let a = self.a().expect("valid view");
                    hasher.write(a.tree_hash_root().as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for StableContainerClassRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    Ok(Self { bytes })
                }
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn to_owned(&self) -> StableContainerClass {
                    StableContainerClass {
                        a: self.a().expect("valid view").to_owned(),
                    }
                }
            }
        }
        pub mod test_import_2 {
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct ProfileInehritance {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
            #[derive(Debug, Copy, Clone)]
            pub struct ProfileInehritanceRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ProfileInehritanceRef<'a> {
                pub fn a(&self) -> Result<u8, ssz::DecodeError> {
                    let offset = 0usize;
                    let end = offset + 1usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<Optional<Option<u8>>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        5usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        5usize,
                        1usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ProfileInehritanceRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    {
                        let a = self.a().expect("valid view");
                        for _ in 0..0usize {}
                        hasher.write(a.tree_hash_root().as_ref()).expect("write field");
                    }
                    {
                        let b = self.b().expect("valid view");
                        for _ in 0..1usize {}
                        hasher.write(b.tree_hash_root().as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ProfileInehritanceRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    Ok(Self { bytes })
                }
            }
            impl<'a> ProfileInehritanceRef<'a> {
                pub fn to_owned(&self) -> ProfileInehritance {
                    ProfileInehritance {
                        a: self.a().expect("valid view"),
                        b: self.b().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
