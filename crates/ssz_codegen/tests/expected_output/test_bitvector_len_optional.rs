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
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "stable_container", max_fields = 9usize)]
            pub struct BitvectorLenTest {
                pub a: Optional<u8>,
                pub b: Optional<u16>,
            }
            impl tree_hash::TreeHash for BitvectorLenTest {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer/Profile should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
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
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = self.b {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash>::tree_hash_root::<
                        H,
                    >(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`BitvectorLenTest`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                std::marker::Copy
            )]
            pub struct BitvectorLenTestRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BitvectorLenTestRef<'a> {
                pub fn a(&self) -> Result<Optional<u8>, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = self
                        .bytes
                        .get(..2usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: 2usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        9usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &self.bytes[2usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u8> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u8> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u16> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u16> as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    let field_bytes = match ssz::layout::read_active_field_bytes(
                        body,
                        field_layout,
                        |i| field_active[i],
                        0usize,
                    )? {
                        Some(bytes) => bytes,
                        None => return Ok(ssz_types::Optional::None),
                    };
                    let inner = <u8 as ssz::view::DecodeView>::from_ssz_bytes(
                        field_bytes,
                    )?;
                    Ok(ssz_types::Optional::Some(inner))
                }
                pub fn b(&self) -> Result<Optional<u16>, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = self
                        .bytes
                        .get(..2usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: 2usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        9usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &self.bytes[2usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u8> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u8> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u16> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u16> as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    let field_bytes = match ssz::layout::read_active_field_bytes(
                        body,
                        field_layout,
                        |i| field_active[i],
                        1usize,
                    )? {
                        Some(bytes) => bytes,
                        None => return Ok(ssz_types::Optional::None),
                    };
                    let inner = <u16 as ssz::view::DecodeView>::from_ssz_bytes(
                        field_bytes,
                    )?;
                    Ok(ssz_types::Optional::Some(inner))
                }
            }
            impl<'a> tree_hash::TreeHash for BitvectorLenTestRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("StableContainer should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
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
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = b {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash>::tree_hash_root::<
                        H,
                    >(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BitvectorLenTestRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = bytes
                        .get(..2usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 2usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        9usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &bytes[2usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u8> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u8> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u16> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u16> as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    for index in 2usize..9usize {
                        if bitvector.get(index).unwrap_or(false) {
                            return Err(
                                ssz::DecodeError::BytesInvalid(
                                    "StableContainer has active_fields bits set beyond field count"
                                        .to_string(),
                                ),
                            );
                        }
                    }
                    ssz::layout::validate_active_container(
                        body,
                        field_layout,
                        |i| field_active[i],
                    )?;
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
