pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_pragmas_multiple {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// Test multiple pragmas on a class
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                Serialize,
                Deserialize,
                Encode,
                Decode,
                TreeHash
            )]
            #[repr(C)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct MultiPragmaContainer {
                pub x: u32,
                pub y: u32,
            }
            /// Zero-copy view over [`MultiPragmaContainer`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Copy)]
            pub struct MultiPragmaContainerRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> MultiPragmaContainerRef<'a> {
                pub fn x(&self) -> Result<u32, ssz::DecodeError> {
                    let offset = 0usize;
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
                pub fn y(&self) -> Result<u32, ssz::DecodeError> {
                    let offset = 4usize;
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
            for MultiPragmaContainerRef<'a> {
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
                        let field_bytes = &self.bytes[offset..offset + 4usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    {
                        let offset = 4usize;
                        let field_bytes = &self.bytes[offset..offset + 4usize];
                        hasher.write(field_bytes).expect("write field");
                    }
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for MultiPragmaContainerRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 8usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 8usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> MultiPragmaContainerRef<'a> {
                pub fn to_owned(&self) -> MultiPragmaContainer {
                    MultiPragmaContainer {
                        x: self.x().expect("valid view"),
                        y: self.y().expect("valid view"),
                    }
                }
            }
        }
    }
}
