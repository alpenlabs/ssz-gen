pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_nested_aliases {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SIZE_1: u64 = 10u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SIZE_2: u64 = 10u64;
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            pub const SIZE_3: u64 = 10u64;
            pub type A = u8;
            pub type B = A;
            pub type C = B;
            pub type D = VariableList<C, 10usize>;
            pub type E = FixedVector<D, 5usize>;
            pub type F = VariableList<A, { SIZE_3 as usize }>;
            pub type G = FixedVector<
                VariableList<A, { SIZE_3 as usize }>,
                { SIZE_1 as usize },
            >;
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct NestedAliasContainer {
                pub field1: D,
                pub field2: E,
                pub field3: F,
                pub field4: G,
            }
            impl tree_hash::TreeHash for NestedAliasContainer {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.field1)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.field2)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.field3)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.field4)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`NestedAliasContainer`].
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
            pub struct NestedAliasContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> NestedAliasContainerRef<'a> {
                pub fn field1(&self) -> Result<BytesRef<'a, 10usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <D as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <D as ssz::Encode>::ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len()
                            + <G as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field2(
                    &self,
                ) -> Result<
                    FixedVectorRef<'a, BytesRef<'a, 10usize>, 5usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <E as ssz::Encode>::is_ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len(),
                        <E as ssz::Encode>::ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len()
                            + <G as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field3(&self) -> Result<BytesRef<'a, 10usize>, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <F as ssz::Encode>::is_ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len(),
                        <F as ssz::Encode>::ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len()
                            + <G as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn field4(
                    &self,
                ) -> Result<
                    FixedVectorRef<'a, BytesRef<'a, 10usize>, 10usize>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <G as ssz::Encode>::is_ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len(),
                        <G as ssz::Encode>::ssz_fixed_len(),
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len()
                            + <G as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                            + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len()),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for NestedAliasContainerRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root<H: tree_hash::TreeHashDigest>(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
                    {
                        let field1 = self.field1().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&field1);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let field2 = self.field2().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&field2);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let field3 = self.field3().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&field3);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let field4 = self.field4().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&field4);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for NestedAliasContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <D as ssz::Encode>::ssz_fixed_len()
                        + <E as ssz::Encode>::ssz_fixed_len()
                        + <F as ssz::Encode>::ssz_fixed_len()
                        + <G as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<D as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len());
                    if num_variable_fields == 0 {
                        if bytes.len() != fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                    } else {
                        if bytes.len() < fixed_portion_size {
                            return Err(ssz::DecodeError::InvalidByteLength {
                                len: bytes.len(),
                                expected: fixed_portion_size,
                            });
                        }
                        let mut prev_offset: Option<usize> = None;
                        for i in 0..num_variable_fields {
                            let offset = ssz::layout::read_variable_offset(
                                bytes,
                                fixed_portion_size,
                                num_variable_fields,
                                i,
                            )?;
                            if i == 0 && offset != fixed_portion_size {
                                return Err(
                                    ssz::DecodeError::OffsetIntoFixedPortion(offset),
                                );
                            }
                            if let Some(prev) = prev_offset && offset < prev {
                                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
                            }
                            if offset > bytes.len() {
                                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
                            }
                            prev_offset = Some(offset);
                        }
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for NestedAliasContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<D as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<E as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<F as ssz::Encode>::is_ssz_fixed_len())
                        + usize::from(!<G as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <D as ssz::Encode>::ssz_fixed_len()
                            + <E as ssz::Encode>::ssz_fixed_len()
                            + <F as ssz::Encode>::ssz_fixed_len()
                            + <G as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<NestedAliasContainer>
            for NestedAliasContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> NestedAliasContainer {
                    <NestedAliasContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> NestedAliasContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> NestedAliasContainer {
                    NestedAliasContainer {
                        field1: ssz_types::VariableList::new(
                                self.field1().expect("valid view").to_owned(),
                            )
                            .expect("valid view"),
                        field2: self
                            .field2()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                        field3: ssz_types::VariableList::new(
                                self.field3().expect("valid view").to_owned(),
                            )
                            .expect("valid view"),
                        field4: self
                            .field4()
                            .expect("valid view")
                            .to_owned()
                            .expect("valid view"),
                    }
                }
            }
        }
    }
}
