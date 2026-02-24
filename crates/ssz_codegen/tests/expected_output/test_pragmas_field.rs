pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_field {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_primitives::{U128, U256};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test field-level pragmas
            #[derive(
                std::clone::Clone,
                std::fmt::Debug,
                std::cmp::PartialEq,
                std::cmp::Eq,
                ssz_derive::Encode,
                ssz_derive::Decode
            )]
            #[ssz(struct_behaviour = "container")]
            pub struct FieldPragmaContainer {
                pub normal_field: u8,
                #[serde(rename = "custom_field_name")]
                pub pragma_field: u16,
                #[cfg(test)]
                pub multi_pragma_field: u32,
            }
            impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for FieldPragmaContainer {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::Container
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.normal_field)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.pragma_field)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .write(
                            <_ as tree_hash::TreeHash<
                                H,
                            >>::tree_hash_root(&self.multi_pragma_field)
                                .as_ref(),
                        )
                        .expect("tree hash derive should not apply too many leaves");
                    hasher
                        .finish()
                        .expect("tree hash derive should not have a remaining buffer")
                }
            }
            /// Zero-copy view over [`FieldPragmaContainer`].
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
            pub struct FieldPragmaContainerRef<'a> {
                bytes: &'a [u8],
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FieldPragmaContainerRef<'a> {
                pub fn normal_field(&self) -> Result<u8, ssz::DecodeError> {
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
                pub fn pragma_field(&self) -> Result<u16, ssz::DecodeError> {
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
                pub fn multi_pragma_field(&self) -> Result<u32, ssz::DecodeError> {
                    let offset = 3usize;
                    let end = offset + 4usize;
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
            for FieldPragmaContainerRef<'a> {
                fn tree_hash_type() -> tree_hash::TreeHashType {
                    tree_hash::TreeHashType::StableContainer
                }
                fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_packing_factor() -> usize {
                    unreachable!("Container should never be packed")
                }
                fn tree_hash_root(&self) -> H::Output {
                    use tree_hash::TreeHash;
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(3usize);
                    {
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 1usize;
                        let field_bytes = &self.bytes[offset..offset + 2usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 3usize;
                        let field_bytes = &self.bytes[offset..offset + 4usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for FieldPragmaContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 7usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 7usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for FieldPragmaContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    7usize
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> ssz_types::view::ToOwnedSsz<FieldPragmaContainer>
            for FieldPragmaContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                fn to_owned(&self) -> FieldPragmaContainer {
                    <FieldPragmaContainerRef<'a>>::to_owned(self)
                }
            }
            #[allow(dead_code, reason = "generated code using ssz-gen")]
            impl<'a> FieldPragmaContainerRef<'a> {
                #[allow(
                    clippy::wrong_self_convention,
                    reason = "API convention for view types"
                )]
                pub fn to_owned(&self) -> FieldPragmaContainer {
                    FieldPragmaContainer {
                        normal_field: self.normal_field().expect("valid view"),
                        pragma_field: self.pragma_field().expect("valid view"),
                        multi_pragma_field: self
                            .multi_pragma_field()
                            .expect("valid view"),
                    }
                }
            }
        }
    }
}
