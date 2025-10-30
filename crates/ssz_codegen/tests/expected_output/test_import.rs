pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_common {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
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
            #[derive(Debug, Copy, Clone)]
            pub struct AliasUnionUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasUnionUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector1(&self) -> Result<Option<u8>, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasUnionUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self.selector() {
                        0u8 => {
                            AliasUnionUnion::Selector0(
                                self.as_selector0().expect("valid selector"),
                            )
                        }
                        1u8 => {
                            AliasUnionUnion::Selector1(
                                self.as_selector1().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AliasUnionUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AliasUnionUnionRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self.selector() {
                        0u8 => {
                            let value = self.as_selector0().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            pub const CONSTANT_VALUE: u64 = 5u64;
            pub type AliasUint8 = u8;
            pub type AliasAliasUint8 = AliasUint8;
            pub type AliasUnion = Option<u8>;
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Encode,
                Decode,
                TreeHash
            )]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<u8>,
                pub b: Optional<AliasUnion>,
            }
            /**Zero-copy view over [`StableContainerClass`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
            pub struct StableContainerClassRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<Optional<Option<u8>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        1usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
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
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        5usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
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
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
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
            #[derive(Debug, Copy, Clone)]
            pub struct AliasUnionUnionRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AliasUnionUnionRef<'a> {
                pub fn selector(&self) -> u8 {
                    self.bytes[0]
                }
                pub fn as_selector0(&self) -> Result<(), ssz::DecodeError> {
                    if self.selector() != 0u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasUnionUnion: expected 0".to_string(),
                            ),
                        );
                    }
                    Ok(())
                }
                pub fn as_selector1(&self) -> Result<u8, ssz::DecodeError> {
                    if self.selector() != 1u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasUnionUnion: expected 1".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn as_selector2(&self) -> Result<Option<u8>, ssz::DecodeError> {
                    if self.selector() != 2u8 {
                        return Err(
                            ssz::DecodeError::BytesInvalid(
                                "Wrong selector for AliasUnionUnion: expected 2".to_string(),
                            ),
                        );
                    }
                    ssz::view::DecodeView::from_ssz_bytes(&self.bytes[1..])
                }
                pub fn to_owned(&self) -> AliasUnionUnion {
                    match self.selector() {
                        0u8 => {
                            self.as_selector0().expect("valid selector");
                            AliasUnionUnion::Selector0
                        }
                        1u8 => {
                            AliasUnionUnion::Selector1(
                                self.as_selector1().expect("valid selector"),
                            )
                        }
                        2u8 => {
                            AliasUnionUnion::Selector2(
                                self.as_selector2().expect("valid selector").to_owned(),
                            )
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
                    }
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AliasUnionUnionRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let (_, _) = ssz::split_union_bytes(bytes)?;
                    Ok(Self { bytes })
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for AliasUnionUnionRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Vector
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Union should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    match self.selector() {
                        0u8 => {
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&tree_hash::Hash256::ZERO, 0u8)
                                .expect("valid selector")
                        }
                        1u8 => {
                            let value = self.as_selector1().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 1u8)
                                .expect("valid selector")
                        }
                        2u8 => {
                            let value = self.as_selector2().expect("valid selector");
                            tree_hash::mix_in_selector_with_hasher::<
                                H,
                            >(&value.tree_hash_root(), 2u8)
                                .expect("valid selector")
                        }
                        _ => panic!("Invalid union selector: {}", self.selector()),
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
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Encode,
                Decode,
                TreeHash
            )]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct StableContainerClass {
                pub a: Optional<crate::tests::input::test_common::AliasUint8>,
            }
            /**Zero-copy view over [`StableContainerClass`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
            pub struct StableContainerClassRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> StableContainerClassRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        4usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        4usize,
                        1usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
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
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        5usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
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
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Encode,
                Decode,
                TreeHash
            )]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 5usize)]
            pub struct ProfileInehritance {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<Option<u8>>,
            }
            /**Zero-copy view over [`ProfileInehritance`].

This type wraps SSZ-encoded bytes without allocating. Fields are accessed via lazy getter methods. Use `.to_owned()` to convert to the owned type when needed.*/
            #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
            pub struct ProfileInehritanceRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ProfileInehritanceRef<'a> {
                pub fn a(&self) -> Result<u8, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let offset = bitvector_offset + 0usize;
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
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        5usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        5usize,
                        1usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
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
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        5usize,
                    >::from_ssz_bytes(&bytes[0..bitvector_length])?;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
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
