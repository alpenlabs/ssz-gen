pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_2 {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 2usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 2usize)]
            pub struct Alpha {
                pub a: Optional<u8>,
                pub b: Optional<BitList<32usize>>,
            }
            /// Zero-copy view over [`Alpha`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct AlphaRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AlphaRef<'a> {
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
                pub fn b(
                    &self,
                ) -> Result<Optional<BitListRef<'a, 32usize>>, ssz::DecodeError> {
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
            for AlphaRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    let a = self.a().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&a);
                    hasher.write(root.as_ref()).expect("write field");
                    let b = self.b().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&b);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AlphaRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        2usize,
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
            impl<'a> AlphaRef<'a> {
                pub fn to_owned(&self) -> Alpha {
                    Alpha {
                        a: self.a().expect("valid view").to_owned(),
                        b: self.b().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 8usize)]
            #[tree_hash(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct InnerBase {
                pub x: Optional<u8>,
                pub y: Optional<VariableList<u8, 4usize>>,
                pub z: Optional<BitVector<16usize>>,
                pub w: Optional<Alpha>,
            }
            /// Zero-copy view over [`InnerBase`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerBaseRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerBaseRef<'a> {
                pub fn x(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn y(&self) -> Result<Optional<BytesRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        1usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(
                    &self,
                ) -> Result<Optional<BitVectorRef<'a, 16usize>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        2usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn w(&self) -> Result<Optional<AlphaRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        16usize,
                        4usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        16usize,
                        4usize,
                        3usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for InnerBaseRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    let x = self.x().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&x);
                    hasher.write(root.as_ref()).expect("write field");
                    let y = self.y().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&y);
                    hasher.write(root.as_ref()).expect("write field");
                    let z = self.z().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&z);
                    hasher.write(root.as_ref()).expect("write field");
                    let w = self.w().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&w);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerBaseRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        8usize,
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
            impl<'a> InnerBaseRef<'a> {
                pub fn to_owned(&self) -> InnerBase {
                    InnerBase {
                        x: self.x().expect("valid view").to_owned(),
                        y: self.y().expect("valid view").to_owned(),
                        z: self.z().expect("valid view").to_owned(),
                        w: self.w().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
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
            /// Zero-copy view over [`InnerProfile1`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerProfile1Ref<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerProfile1Ref<'a> {
                pub fn x(&self) -> Result<u8, ssz::DecodeError> {
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
                pub fn y(&self) -> Result<Optional<BytesRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        13usize,
                        3usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        13usize,
                        3usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(
                    &self,
                ) -> Result<Optional<BitVectorRef<'a, 16usize>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        13usize,
                        3usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        13usize,
                        3usize,
                        1usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn w(&self) -> Result<Optional<AlphaRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        13usize,
                        3usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        13usize,
                        3usize,
                        2usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for InnerProfile1Ref<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let x = self.x().expect("valid view");
                        for _ in 0..0usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&x);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let y = self.y().expect("valid view");
                        for _ in 0..1usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&y);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let z = self.z().expect("valid view");
                        for _ in 0..2usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&z);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let w = self.w().expect("valid view");
                        for _ in 0..3usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&w);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerProfile1Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        8usize,
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
            impl<'a> InnerProfile1Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile1 {
                    InnerProfile1 {
                        x: self.x().expect("valid view"),
                        y: self.y().expect("valid view").to_owned(),
                        z: self.z().expect("valid view").to_owned(),
                        w: self.w().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
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
            /// Zero-copy view over [`InnerProfile2`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerProfile2Ref<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerProfile2Ref<'a> {
                pub fn x(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        10usize,
                        2usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        10usize,
                        2usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn y(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        10usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        10usize,
                        2usize,
                        1usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(&self) -> Result<BitVectorRef<'a, 16usize>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let offset = bitvector_offset + 8usize;
                    let end = offset + 2usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for InnerProfile2Ref<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let x = self.x().expect("valid view");
                        for _ in 0..0usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&x);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let y = self.y().expect("valid view");
                        for _ in 0..1usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&y);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let z = self.z().expect("valid view");
                        for _ in 0..2usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&z);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerProfile2Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        8usize,
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
            impl<'a> InnerProfile2Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile2 {
                    InnerProfile2 {
                        x: self.x().expect("valid view").to_owned(),
                        y: self.y().expect("valid view").to_owned().into(),
                        z: self.z().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 2usize)]
            pub struct AlphaProfile {
                #[tree_hash(stable_index = 0usize)]
                pub a: u8,
                #[tree_hash(stable_index = 1usize)]
                pub b: Optional<BitList<32usize>>,
            }
            /// Zero-copy view over [`AlphaProfile`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct AlphaProfileRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> AlphaProfileRef<'a> {
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
                pub fn b(
                    &self,
                ) -> Result<Optional<BitListRef<'a, 32usize>>, ssz::DecodeError> {
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
            for AlphaProfileRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    {
                        let a = self.a().expect("valid view");
                        for _ in 0..0usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&a);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let b = self.b().expect("valid view");
                        for _ in 0..1usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&b);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for AlphaProfileRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        2usize,
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
            impl<'a> AlphaProfileRef<'a> {
                pub fn to_owned(&self) -> AlphaProfile {
                    AlphaProfile {
                        a: self.a().expect("valid view"),
                        b: self.b().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile3 {
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfile,
            }
            /// Zero-copy view over [`InnerProfile3`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerProfile3Ref<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerProfile3Ref<'a> {
                pub fn w(&self) -> Result<AlphaProfileRef<'a>, ssz::DecodeError> {
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
            for InnerProfile3Ref<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let w = self.w().expect("valid view");
                        for _ in 0..3usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&w);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerProfile3Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 4usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 4usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            4usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 4usize {
                            return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                        }
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> InnerProfile3Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile3 {
                    InnerProfile3 {
                        w: self.w().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct InnerProfile4 {
                #[tree_hash(stable_index = 1usize)]
                pub y: VariableList<u8, 4usize>,
                #[tree_hash(stable_index = 2usize)]
                pub z: BitVector<16usize>,
            }
            /// Zero-copy view over [`InnerProfile4`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerProfile4Ref<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerProfile4Ref<'a> {
                pub fn y(&self) -> Result<BytesRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        6usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        6usize,
                        1usize,
                        0usize + 1,
                    )?;
                    if start > end || end > self.bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &self.bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(&self) -> Result<BitVectorRef<'a, 16usize>, ssz::DecodeError> {
                    let offset = 4usize;
                    let end = offset + 2usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for InnerProfile4Ref<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let y = self.y().expect("valid view");
                        for _ in 0..1usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&y);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let z = self.z().expect("valid view");
                        for _ in 0..2usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&z);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerProfile4Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 6usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 6usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            6usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 6usize {
                            return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                        }
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> InnerProfile4Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile4 {
                    InnerProfile4 {
                        y: self.y().expect("valid view").to_owned().into(),
                        z: self.z().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
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
            /// Zero-copy view over [`InnerProfile5`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct InnerProfile5Ref<'a> {
                bytes: &'a [u8],
            }
            impl<'a> InnerProfile5Ref<'a> {
                pub fn x(&self) -> Result<u8, ssz::DecodeError> {
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
                pub fn z(&self) -> Result<BitVectorRef<'a, 16usize>, ssz::DecodeError> {
                    let offset = 1usize;
                    let end = offset + 2usize;
                    if end > self.bytes.len() {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: end,
                        });
                    }
                    let bytes = &self.bytes[offset..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn w(&self) -> Result<AlphaRef<'a>, ssz::DecodeError> {
                    let start = ssz::layout::read_variable_offset(
                        self.bytes,
                        7usize,
                        1usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        self.bytes,
                        7usize,
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
            for InnerProfile5Ref<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let x = self.x().expect("valid view");
                        for _ in 0..0usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&x);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let z = self.z().expect("valid view");
                        for _ in 0..2usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&z);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let w = self.w().expect("valid view");
                        for _ in 0..3usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&w);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for InnerProfile5Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() < 7usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 7usize,
                        });
                    }
                    let mut prev_offset: Option<usize> = None;
                    for i in 0..1usize {
                        let offset = ssz::layout::read_variable_offset(
                            bytes,
                            7usize,
                            1usize,
                            i,
                        )?;
                        if i == 0 && offset != 7usize {
                            return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
                        }
                        if let Some(prev) = prev_offset {
                            if offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                        }
                        if offset > bytes.len() {
                            return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                        }
                        prev_offset = Some(offset);
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> InnerProfile5Ref<'a> {
                pub fn to_owned(&self) -> InnerProfile5 {
                    InnerProfile5 {
                        x: self.x().expect("valid view"),
                        z: self.z().expect("valid view").to_owned(),
                        w: self.w().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "profile")]
            #[tree_hash(struct_behaviour = "profile", max_fields = 8usize)]
            pub struct ProfileProfile {
                #[tree_hash(stable_index = 0usize)]
                pub x: Optional<u8>,
                #[tree_hash(stable_index = 3usize)]
                pub w: AlphaProfile,
            }
            /// Zero-copy view over [`ProfileProfile`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ProfileProfileRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ProfileProfileRef<'a> {
                pub fn x(&self) -> Result<Optional<u8>, ssz::DecodeError> {
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
                pub fn w(&self) -> Result<AlphaProfileRef<'a>, ssz::DecodeError> {
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
            for ProfileProfileRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    {
                        let x = self.x().expect("valid view");
                        for _ in 0..0usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&x);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let w = self.w().expect("valid view");
                        for _ in 0..3usize {}
                        let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                            H,
                        >::tree_hash_root(&w);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ProfileProfileRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        8usize,
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
            impl<'a> ProfileProfileRef<'a> {
                pub fn to_owned(&self) -> ProfileProfile {
                    ProfileProfile {
                        x: self.x().expect("valid view").to_owned(),
                        w: self.w().expect("valid view").to_owned(),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
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
            /// Zero-copy view over [`ContainerContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct ContainerContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> ContainerContainerRef<'a> {
                pub fn x(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        0usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn y(&self) -> Result<Optional<BytesRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        1usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn z(
                    &self,
                ) -> Result<Optional<BitVectorRef<'a, 16usize>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        2usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn w(&self) -> Result<Optional<AlphaRef<'a>>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        3usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        3usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        4usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        4usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        5usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        5usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn c(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        6usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        6usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn d(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        32usize,
                        8usize,
                        7usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        32usize,
                        8usize,
                        7usize + 1,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ContainerContainerRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(8usize);
                    let x = self.x().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&x);
                    hasher.write(root.as_ref()).expect("write field");
                    let y = self.y().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&y);
                    hasher.write(root.as_ref()).expect("write field");
                    let z = self.z().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&z);
                    hasher.write(root.as_ref()).expect("write field");
                    let w = self.w().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&w);
                    hasher.write(root.as_ref()).expect("write field");
                    let a = self.a().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&a);
                    hasher.write(root.as_ref()).expect("write field");
                    let b = self.b().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&b);
                    hasher.write(root.as_ref()).expect("write field");
                    let c = self.c().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&c);
                    hasher.write(root.as_ref()).expect("write field");
                    let d = self.d().expect("valid view");
                    let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                        H,
                    >::tree_hash_root(&d);
                    hasher.write(root.as_ref()).expect("write field");
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ContainerContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let bitvector_length = 1usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz::BitVector::<
                        8usize,
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
            impl<'a> ContainerContainerRef<'a> {
                pub fn to_owned(&self) -> ContainerContainer {
                    ContainerContainer {
                        x: self.x().expect("valid view").to_owned(),
                        y: self.y().expect("valid view").to_owned(),
                        z: self.z().expect("valid view").to_owned(),
                        w: self.w().expect("valid view").to_owned(),
                        a: self.a().expect("valid view").to_owned(),
                        b: self.b().expect("valid view").to_owned(),
                        c: self.c().expect("valid view").to_owned(),
                        d: self.d().expect("valid view").to_owned(),
                    }
                }
            }
        }
    }
}
