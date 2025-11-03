pub mod tests {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    pub mod input {
        #![allow(unused_imports, reason = "generated code using ssz-gen")]
        pub mod test_docstrings {
            #![allow(unused_imports, reason = "generated code using ssz-gen")]
            use ssz_types::*;
            use ssz_types::view::{FixedVectorRef, VariableListRef};
            use ssz_derive::{Encode, Decode};
            use tree_hash::TreeHashDigest;
            use tree_hash_derive::TreeHash;
            use ssz::view::*;
            /// This is a foo.
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct Foo {}
            /// Zero-copy view over [`Foo`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct FooRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> FooRef<'a> {}
            impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H>
            for FooRef<'a> {
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
                    hasher.finish().expect("finish hasher")
                }
            }
            impl<'a> ssz::view::DecodeView<'a> for FooRef<'a> {
                fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
                    if bytes.len() != 0usize {
                        return Err(ssz::DecodeError::InvalidByteLength {
                            len: bytes.len(),
                            expected: 0usize,
                        });
                    }
                    Ok(Self { bytes })
                }
            }
            impl<'a> FooRef<'a> {
                pub fn to_owned(&self) -> Foo {
                    Foo {}
                }
            }
            /// This is a docstring that should come first.
            ///
            /// This is a doc comment
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct PointWithBoth {
                /// X coordinate
                pub x: u32,
                /// Y coordinate
                pub y: u32,
            }
            /// Zero-copy view over [`PointWithBoth`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct PointWithBothRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> PointWithBothRef<'a> {
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
            for PointWithBothRef<'a> {
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
            impl<'a> ssz::view::DecodeView<'a> for PointWithBothRef<'a> {
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
            impl<'a> PointWithBothRef<'a> {
                pub fn to_owned(&self) -> PointWithBoth {
                    PointWithBoth {
                        x: self.x().expect("valid view"),
                        y: self.y().expect("valid view"),
                    }
                }
            }
            /// First comes the docstring. It has multiple lines.
            ///
            /// This should come after the docstring
            #[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TreeHash)]
            #[ssz(struct_behaviour = "container")]
            #[tree_hash(struct_behaviour = "container")]
            pub struct TestMerge {
                pub field: u8,
            }
            /// Zero-copy view over [`TestMerge`].
            ///
            /// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
            /// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
            /// needed.
            #[derive(Clone, Debug, PartialEq, Eq, Copy)]
            pub struct TestMergeRef<'a> {
                bytes: &'a [u8],
            }
            impl<'a> TestMergeRef<'a> {
                pub fn field(&self) -> Result<u8, ssz::DecodeError> {
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
            for TestMergeRef<'a> {
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
            impl<'a> ssz::view::DecodeView<'a> for TestMergeRef<'a> {
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
            impl<'a> TestMergeRef<'a> {
                pub fn to_owned(&self) -> TestMerge {
                    TestMerge {
                        field: self.field().expect("valid view"),
                    }
                }
            }
        }
    }
}
