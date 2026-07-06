pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_multi_import_a {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Type A that imports base
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct TypeA {
                pub base: crate::tests::input::test_multi_import_base::BaseType,
                pub data: u32,
            }
            impl tree_hash::TreeHash for TypeA {
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
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.base)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.data)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`TypeA`].
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
            pub struct TypeARef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TypeARef<'a> {
                pub fn base(
                    &self,
                ) -> Result<
                    crate::tests::input::test_multi_import_base::BaseTypeRef<'a>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn data(&self) -> Result<u32, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u32 as ssz::Encode>::is_ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len(),
                        <u32 as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        ),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for TypeARef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
                    {
                        let base = self.base().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&base);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let data = self.data().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&data);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TypeARef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                        + <u32 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len());
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
            impl<'a> ssz::view::SszTypeInfo for TypeARef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(
                        !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                    ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <u32 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<TypeA> for TypeARef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> TypeA {
                    <TypeARef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TypeARef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> TypeA {
                    TypeA {
                        base: {
                            let view = self.base().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        data: self.data().expect("valid view"),
                    }
                }
            }
        }
        pub mod test_multi_import_b {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Type B that imports both base and a
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct TypeB {
                pub base: crate::tests::input::test_multi_import_base::BaseType,
                pub type_a: crate::tests::input::test_multi_import_a::TypeA,
                pub extra: u16,
            }
            impl tree_hash::TreeHash for TypeB {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.base)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.type_a)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.extra)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`TypeB`].
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
            pub struct TypeBRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TypeBRef<'a> {
                pub fn base(
                    &self,
                ) -> Result<
                    crate::tests::input::test_multi_import_base::BaseTypeRef<'a>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len()
                            + <u16 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        )
                            + usize::from(
                                !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                            ) + usize::from(!<u16 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn type_a(
                    &self,
                ) -> Result<
                    crate::tests::input::test_multi_import_a::TypeARef<'a>,
                    ssz::DecodeError,
                > {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len()
                            + <u16 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        )
                            + usize::from(
                                !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                            ) + usize::from(!<u16 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        ),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
                pub fn extra(&self) -> Result<u16, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u16 as ssz::Encode>::is_ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len(),
                        <u16 as ssz::Encode>::ssz_fixed_len(),
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len()
                            + <u16 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        )
                            + usize::from(
                                !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                            ) + usize::from(!<u16 as ssz::Encode>::is_ssz_fixed_len()),
                        usize::from(
                            !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                        )
                            + usize::from(
                                !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                            ),
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for TypeBRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    {
                        let base = self.base().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&base);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let type_a = self.type_a().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&type_a);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    {
                        let extra = self.extra().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&extra);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for TypeBRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                        + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len()
                        + <u16 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                    )
                        + usize::from(
                            !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                        ) + usize::from(!<u16 as ssz::Encode>::is_ssz_fixed_len());
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
            impl<'a> ssz::view::SszTypeInfo for TypeBRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(
                        !<crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::is_ssz_fixed_len(),
                    )
                        + usize::from(
                            !<crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::is_ssz_fixed_len(),
                        ) + usize::from(!<u16 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <crate::tests::input::test_multi_import_base::BaseType as ssz::Encode>::ssz_fixed_len()
                            + <crate::tests::input::test_multi_import_a::TypeA as ssz::Encode>::ssz_fixed_len()
                            + <u16 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<TypeB> for TypeBRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> TypeB {
                    <TypeBRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> TypeBRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> TypeB {
                    TypeB {
                        base: {
                            let view = self.base().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        type_a: {
                            let view = self.type_a().expect("valid view");
                            ssz_types::view::ToOwnedSsz::to_owned(&view)
                        },
                        extra: self.extra().expect("valid view"),
                    }
                }
            }
        }
        pub mod test_multi_import_base {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Base type shared by multiple importers
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct BaseType {
                pub value: u64,
            }
            impl tree_hash::TreeHash for BaseType {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.value)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`BaseType`].
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
            pub struct BaseTypeRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BaseTypeRef<'a> {
                pub fn value(&self) -> Result<u64, ssz::DecodeError> {
                    let bytes = ssz::layout::read_field_bytes(
                        self.bytes,
                        <u64 as ssz::Encode>::is_ssz_fixed_len(),
                        0usize,
                        <u64 as ssz::Encode>::ssz_fixed_len(),
                        <u64 as ssz::Encode>::ssz_fixed_len(),
                        usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
                        0usize,
                    )?;
                    ssz::view::DecodeView::from_ssz_bytes(bytes)
                }
            }
            impl<'a> tree_hash::TreeHash for BaseTypeRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
                    {
                        let value = self.value().expect("valid view");
                        let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                            H,
                        >(&value);
                        hasher.write(root.as_ref()).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for BaseTypeRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    let fixed_portion_size = <u64 as ssz::Encode>::ssz_fixed_len();
                    let num_variable_fields = usize::from(
                        !<u64 as ssz::Encode>::is_ssz_fixed_len(),
                    );
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
            impl<'a> ssz::view::SszTypeInfo for BaseTypeRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
                }
                fn ssz_fixed_len() -> usize {
                    if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                        <u64 as ssz::Encode>::ssz_fixed_len()
                    } else {
                        0
                    }
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<BaseType> for BaseTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> BaseType {
                    <BaseTypeRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> BaseTypeRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> BaseType {
                    BaseType {
                        value: self.value().expect("valid view"),
                    }
                }
            }
        }
    }
}
