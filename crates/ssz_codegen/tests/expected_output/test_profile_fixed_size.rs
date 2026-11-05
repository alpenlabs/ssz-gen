pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_profile_fixed_size {
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
            #[ssz(struct_behaviour = "stable_container", max_fields = 8usize)]
            pub struct StateBase {
                pub balance: Optional<u64>,
                pub nonce: Optional<u64>,
                pub extra: Optional<u64>,
            }
            impl tree_hash::TreeHash for StateBase {
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
                    let mut active_fields = BitVector::<8usize>::new();
                    if self.balance.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.nonce.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if self.extra.is_some() {
                        active_fields
                            .set(2usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        8usize,
                    );
                    if let ssz_types::Optional::Some(ref inner) = self.balance {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = self.nonce {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = self.extra {
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
            /// Zero-copy view over [`StateBase`].
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
            pub struct StateBaseRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateBaseRef<'a> {
                pub fn balance(&self) -> Result<Optional<u64>, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = self
                        .bytes
                        .get(..1usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: 1usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        8usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &self.bytes[1usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                        bitvector.get(2usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
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
                    let inner = <u64 as ssz::view::DecodeView>::from_ssz_bytes(
                        field_bytes,
                    )?;
                    Ok(ssz_types::Optional::Some(inner))
                }
                pub fn nonce(&self) -> Result<Optional<u64>, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = self
                        .bytes
                        .get(..1usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: 1usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        8usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &self.bytes[1usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                        bitvector.get(2usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
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
                    let inner = <u64 as ssz::view::DecodeView>::from_ssz_bytes(
                        field_bytes,
                    )?;
                    Ok(ssz_types::Optional::Some(inner))
                }
                pub fn extra(&self) -> Result<Optional<u64>, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = self
                        .bytes
                        .get(..1usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: self.bytes.len(),
                            expected: 1usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        8usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &self.bytes[1usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                        bitvector.get(2usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    let field_bytes = match ssz::layout::read_active_field_bytes(
                        body,
                        field_layout,
                        |i| field_active[i],
                        2usize,
                    )? {
                        Some(bytes) => bytes,
                        None => return Ok(ssz_types::Optional::None),
                    };
                    let inner = <u64 as ssz::view::DecodeView>::from_ssz_bytes(
                        field_bytes,
                    )?;
                    Ok(ssz_types::Optional::Some(inner))
                }
            }
            impl<'a> tree_hash::TreeHash for StateBaseRef<'a> {
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
                    let balance = self.balance().expect("valid view");
                    let nonce = self.nonce().expect("valid view");
                    let extra = self.extra().expect("valid view");
                    let mut active_fields = BitVector::<8usize>::new();
                    if balance.is_some() {
                        active_fields
                            .set(0usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if nonce.is_some() {
                        active_fields
                            .set(1usize, true)
                            .expect("Should not be out of bounds");
                    }
                    if extra.is_some() {
                        active_fields
                            .set(2usize, true)
                            .expect("Should not be out of bounds");
                    }
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        8usize,
                    );
                    if let ssz_types::Optional::Some(ref inner) = balance {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = nonce {
                        field_roots
                            .push(
                                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(inner),
                            );
                    }
                    if let ssz_types::Optional::Some(ref inner) = extra {
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
            impl<'a> ssz::view::DecodeView<'a> for StateBaseRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    use ssz::Decode;
                    let bitvector_bytes = bytes
                        .get(..1usize)
                        .ok_or(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 1usize,
                        })?;
                    let bitvector = ssz_types::BitVector::<
                        8usize,
                    >::from_ssz_bytes(bitvector_bytes)?;
                    let body = &bytes[1usize..];
                    let field_active: &[bool] = &[
                        bitvector.get(0usize).unwrap_or(false),
                        bitvector.get(1usize).unwrap_or(false),
                        bitvector.get(2usize).unwrap_or(false),
                    ];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <Optional<u64> as ssz::Encode>::is_ssz_fixed_len(),
                            <Optional<u64> as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    for index in 3usize..8usize {
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
            impl<'a> ssz::view::SszTypeInfo for StateBaseRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    false
                }
                fn ssz_fixed_len() -> usize {
                    0
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<StateBase> for StateBaseRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> StateBase {
                    <StateBaseRef<'a>>::to_owned(self)
                }
                fn try_to_owned(&self) -> Result<StateBase, ssz::DecodeError> {
                    <StateBaseRef<'a>>::try_to_owned(self)
                }
            }
            impl ssz_types::view::SszHasView for StateBase {
                type Ref<'a> = StateBaseRef<'a>;
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateBaseRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> StateBase {
                    <StateBaseRef<'a>>::try_to_owned(self).expect("valid view")
                }
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn try_to_owned(&self) -> Result<StateBase, ssz::DecodeError> {
                    Ok(StateBase {
                        balance: self.balance()?,
                        nonce: self.nonce()?,
                        extra: self.extra()?,
                    })
                }
            }
            /// Fixed-size Profile: encoding is exactly 16 bytes.
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "profile")]
            pub struct StateV1 {
                pub balance: u64,
                pub nonce: u64,
            }
            impl tree_hash::TreeHash for StateV1 {
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
                    let mut active_fields = BitVector::<8usize>::new();
                    active_fields
                        .set(0usize, true)
                        .expect("Should not be out of bounds");
                    active_fields
                        .set(1usize, true)
                        .expect("Should not be out of bounds");
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        8usize,
                    );
                    field_roots
                        .push(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<
                                H,
                            >(&self.balance),
                        );
                    field_roots
                        .push(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.nonce),
                        );
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash>::tree_hash_root::<
                        H,
                    >(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            /// Zero-copy view over [`StateV1`].
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
            pub struct StateV1Ref<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateV1Ref<'a> {
                pub fn balance(&self) -> Result<u64, ssz::DecodeError> {
                    let body = self.bytes;
                    let field_active: &[bool] = &[true, true];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    let field_bytes = match ssz::layout::read_active_field_bytes(
                        body,
                        field_layout,
                        |i| field_active[i],
                        0usize,
                    )? {
                        Some(bytes) => bytes,
                        None => unreachable!("required field is always active"),
                    };
                    ssz::view::DecodeView::from_ssz_bytes(field_bytes)
                }
                pub fn nonce(&self) -> Result<u64, ssz::DecodeError> {
                    let body = self.bytes;
                    let field_active: &[bool] = &[true, true];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    let field_bytes = match ssz::layout::read_active_field_bytes(
                        body,
                        field_layout,
                        |i| field_active[i],
                        1usize,
                    )? {
                        Some(bytes) => bytes,
                        None => unreachable!("required field is always active"),
                    };
                    ssz::view::DecodeView::from_ssz_bytes(field_bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for StateV1Ref<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Profile should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Profile should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    use ssz_types::BitVector;
                    let balance = self.balance().expect("valid view");
                    let nonce = self.nonce().expect("valid view");
                    let mut active_fields = BitVector::<8usize>::new();
                    active_fields
                        .set(0usize, true)
                        .expect("Should not be out of bounds");
                    active_fields
                        .set(1usize, true)
                        .expect("Should not be out of bounds");
                    let mut field_roots: Vec<<H as tree_hash::TreeHashDigest>::Output> = Vec::with_capacity(
                        8usize,
                    );
                    field_roots
                        .push(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&balance));
                    field_roots
                        .push(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&nonce));
                    let hash = tree_hash::merkleize_progressive_with_hasher::<
                        H,
                    >(&field_roots);
                    let active_fields_hash = <_ as tree_hash::TreeHash>::tree_hash_root::<
                        H,
                    >(&active_fields);
                    H::hash32_concat(hash.as_ref(), active_fields_hash.as_ref())
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for StateV1Ref<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let body = bytes;
                    let field_active: &[bool] = &[true, true];
                    let field_layout: &[ssz::layout::FieldInfo] = &[
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                        (
                            <u64 as ssz::Encode>::is_ssz_fixed_len(),
                            <u64 as ssz::Encode>::ssz_fixed_len(),
                        ),
                    ];
                    ssz::layout::validate_active_container(
                        body,
                        field_layout,
                        |i| field_active[i],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for StateV1Ref<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u64 as ssz::Encode>::ssz_fixed_len()
                            + <u64 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<StateV1> for StateV1Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> StateV1 {
                    <StateV1Ref<'a>>::to_owned(self)
                }
                fn try_to_owned(&self) -> Result<StateV1, ssz::DecodeError> {
                    <StateV1Ref<'a>>::try_to_owned(self)
                }
            }
            impl ssz_types::view::SszHasView for StateV1 {
                type Ref<'a> = StateV1Ref<'a>;
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateV1Ref<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> StateV1 {
                    <StateV1Ref<'a>>::try_to_owned(self).expect("valid view")
                }
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn try_to_owned(&self) -> Result<StateV1, ssz::DecodeError> {
                    Ok(StateV1 {
                        balance: self.balance()?,
                        nonce: self.nonce()?,
                    })
                }
            }
            /// Fixed-size container embedding a fixed-size Profile.
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct StateWrapper {
                pub state: StateV1,
                pub tag: u8,
            }
            impl tree_hash::TreeHash for StateWrapper {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.state)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.tag)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`StateWrapper`].
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
            pub struct StateWrapperRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateWrapperRef<'a> {
                pub fn state(&self) -> Result<StateV1Ref<'a>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <StateV1 as ssz::Encode>::is_ssz_fixed_len(),
                                <StateV1 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u8 as ssz::Encode>::is_ssz_fixed_len(),
                                <u8 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn tag(&self) -> Result<u8, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        &[
                            (
                                <StateV1 as ssz::Encode>::is_ssz_fixed_len(),
                                <StateV1 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u8 as ssz::Encode>::is_ssz_fixed_len(),
                                <u8 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                        1usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for StateWrapperRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    {
                        let state = self.state().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&state);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let tag = self.tag().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&tag);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for StateWrapperRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    ssz::layout::validate_container(
                        bytes,
                        &[
                            (
                                <StateV1 as ssz::Encode>::is_ssz_fixed_len(),
                                <StateV1 as ssz::Encode>::ssz_fixed_len(),
                            ),
                            (
                                <u8 as ssz::Encode>::is_ssz_fixed_len(),
                                <u8 as ssz::Encode>::ssz_fixed_len(),
                            ),
                        ],
                    )?;
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for StateWrapperRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<StateV1 as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<u8 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <StateV1 as ssz::Encode>::ssz_fixed_len()
                            + <u8 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<StateWrapper> for StateWrapperRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> StateWrapper {
                    <StateWrapperRef<'a>>::to_owned(self)
                }
                fn try_to_owned(&self) -> Result<StateWrapper, ssz::DecodeError> {
                    <StateWrapperRef<'a>>::try_to_owned(self)
                }
            }
            impl ssz_types::view::SszHasView for StateWrapper {
                type Ref<'a> = StateWrapperRef<'a>;
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> StateWrapperRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> StateWrapper {
                    <StateWrapperRef<'a>>::try_to_owned(self).expect("valid view")
                }
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn try_to_owned(&self) -> Result<StateWrapper, ssz::DecodeError> {
                    Ok(StateWrapper {
                        state: {
                            let view = self.state()?;
                            ssz_types::view::ToOwnedSsz::try_to_owned(&view)?
                        },
                        tag: self.tag()?,
                    })
                }
            }
        }
    }
}
