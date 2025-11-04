pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_empty {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test empty pragmas (edge case)
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct EmptyPragmaContainer {
                pub x: u8,
            }
            /// Zero-copy view over [`EmptyPragmaContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct EmptyPragmaContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> EmptyPragmaContainerRef<'a> {
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
            }
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for EmptyPragmaContainerRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(0);
                    {
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 1usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for EmptyPragmaContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 1usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 1usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for EmptyPragmaContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    1usize
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<EmptyPragmaContainer>
            for EmptyPragmaContainerRef<'a> {
                fn to_owned(&self) -> EmptyPragmaContainer {
                    <EmptyPragmaContainerRef<'a>>::to_owned(self)
                }
            }
            impl<'a> EmptyPragmaContainerRef<'a> {
                pub fn to_owned(&self) -> EmptyPragmaContainer {
                    EmptyPragmaContainer {
                        x: self.x().expect("valid view"),
                    }
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct EmptyValueContainer {
                pub y: u16,
            }
            /// Zero-copy view over [`EmptyValueContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct EmptyValueContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> EmptyValueContainerRef<'a> {
                pub fn y(&self) -> Result<u16, ssz::DecodeError> {
                    let offset = 0usize;
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
            for EmptyValueContainerRef<'a> {
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
                    let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(0);
                    {
                        let offset = 0usize;
                        let field_bytes = &self.bytes[offset..offset + 2usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for EmptyValueContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 2usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 2usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> ssz::view::SszTypeInfo for EmptyValueContainerRef<'a> {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    2usize
                }
            }
            impl<'a> ssz_types::view::ToOwnedSsz<EmptyValueContainer>
            for EmptyValueContainerRef<'a> {
                fn to_owned(&self) -> EmptyValueContainer {
                    <EmptyValueContainerRef<'a>>::to_owned(self)
                }
            }
            impl<'a> EmptyValueContainerRef<'a> {
                pub fn to_owned(&self) -> EmptyValueContainer {
                    EmptyValueContainer {
                        y: self.y().expect("valid view"),
                    }
                }
            }
        }
    }
}
