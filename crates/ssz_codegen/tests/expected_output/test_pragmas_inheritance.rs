pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_inheritance {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test pragmas with inheritance
            #[derive(Clone, Debug, PartialEq, Eq, Default, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct Parent {
                pub a: Optional<u8>,
                pub b: Optional<u8>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Parent {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<5u64>::new();
                    if self.a.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.b.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    if let Some(ref a) = self.a {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(a).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref b) = self.b {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(b).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Parent`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
            pub struct ParentRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ParentRef<'a> {
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
                        1usize,
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
                        8usize,
                        2usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        8usize,
                        2usize,
                        2usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ParentRef<'a> {
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
            impl<'a> ssz::view::DecodeView<'a> for ParentRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ParentRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Parent> for ParentRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                #[allow(
                    unconditional_recursion,
                    reason = "false positive - delegates to inherent method"
                )]
                fn to_owned(&self) -> Parent {
                    <ParentRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ParentRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Parent {
                    Parent {
                        a: self.a().expect("valid view").to_owned().expect("valid view"),
                        b: self.b().expect("valid view").to_owned().expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Encode, Decode)]
            #[repr(C)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 5usize)]
            pub struct Child {
                pub a: Optional<u8>,
                pub b: Optional<u16>,
                pub c: Optional<u8>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Child {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let mut active_fields = BitVector::<5u64>::new();
                    if self.a.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.b.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.c.is_some() {
                        active_fields
                            .set(2usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(5usize);
                    if let Some(ref a) = self.a {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(a).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref b) = self.b {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(b).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    if let Some(ref c) = self.c {
                        hasher
                            .write(
                                <_ as tree_hash::TreeHash<H>>::tree_hash_root(c).as_ref(),
                            )
                            .expect("tree hash derive should not apply too many leaves");
                    } else {
                        hasher
                            .write(H::get_zero_hash_slice(0))
                            .expect("tree hash derive should not apply too many leaves");
                    }
                    let hash = hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer");
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`Child`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Copy)]
            pub struct ChildRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ChildRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        12usize,
                        3usize,
                        0usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        12usize,
                        3usize,
                        1usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn b(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 1usize;
                    let container_bytes = &self.bytes[bitvector_offset..];
                    let start = ssz::layout::read_variable_offset(
                        container_bytes,
                        12usize,
                        3usize,
                        1usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        12usize,
                        3usize,
                        2usize,
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
                        12usize,
                        3usize,
                        2usize,
                    )?;
                    let end = ssz::layout::read_variable_offset_or_end(
                        container_bytes,
                        12usize,
                        3usize,
                        3usize,
                    )?;
                    if start > end || end > container_bytes.len() {
                        return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
                    }
                    let bytes = &container_bytes[start..end];
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for ChildRef<'a> {
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
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for ChildRef<'a> {
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
            impl<'a> ssz::view::SszTypeInfo for ChildRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<Child> for ChildRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                #[allow(
                    unconditional_recursion,
                    reason = "false positive - delegates to inherent method"
                )]
                fn to_owned(&self) -> Child {
                    <ChildRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ChildRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> Child {
                    Child {
                        a: self.a().expect("valid view").to_owned().expect("valid view"),
                        b: self.b().expect("valid view").to_owned().expect("valid view"),
                        c: self.c().expect("valid view").to_owned().expect("valid view"),
                    }
                }
            }
        }
    }
}
