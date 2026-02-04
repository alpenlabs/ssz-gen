pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_bitvector_len {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
            #[ssz(struct_behaviour = "stable_container", max_fields = 9usize)]
            pub struct BitvectorLenTest {
                pub a: Optional<u8>,
                pub b: Optional<u16>,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for BitvectorLenTest {
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
                    let mut active_fields = BitVector::<9usize>::new();
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
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        9usize,
                    );
                    if let ssz_types::Optional::Some(ref inner) = self.a {
                        field_roots
                            .push(<_ as tree_hash::TreeHash<H>>::tree_hash_root(inner));
                    }
                    if let ssz_types::Optional::Some(ref inner) = self.b {
                        field_roots
                            .push(<_ as tree_hash::TreeHash<H>>::tree_hash_root(inner));
                    }
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`BitvectorLenTest`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct BitvectorLenTestRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitvectorLenTestRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    let bitvector_offset = 2usize;
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
                pub fn b(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    let bitvector_offset = 2usize;
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
            for BitvectorLenTestRef<'a> {
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
                    use ssz_types::BitVector;
                    let a = self.a().expect("valid view");
                    let b = self.b().expect("valid view");
                    let mut active_fields = BitVector::<9usize>::new();
                    if a.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if b.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        9usize,
                    );
                    if let ssz_types::Optional::Some(ref inner) = a {
                        field_roots
                            .push(<_ as tree_hash::TreeHash<H>>::tree_hash_root(inner));
                    }
                    if let ssz_types::Optional::Some(ref inner) = b {
                        field_roots
                            .push(<_ as tree_hash::TreeHash<H>>::tree_hash_root(inner));
                    }
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash<
                        H,
                    >>::tree_hash_root(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BitvectorLenTestRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_length = 2usize;
                    if bytes.len() < bitvector_length {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: bitvector_length,
                        });
                    }
                    let _bitvector = ssz_types::BitVector::<
                        9usize,
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
            impl<'a> ssz::view::SszTypeInfo for BitvectorLenTestRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BitvectorLenTest>
            for BitvectorLenTestRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BitvectorLenTest {
                    <BitvectorLenTestRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitvectorLenTestRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BitvectorLenTest {
                    BitvectorLenTest {
                        a: self.a().expect("valid view"),
                        b: self.b().expect("valid view"),
                    }
                }
            }
        }
    }
}
