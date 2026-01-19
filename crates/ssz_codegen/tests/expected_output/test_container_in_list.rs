#![allow(unused_imports, reason = "generated code using ssz-gen")]
use ssz_types::*;
use ssz_types::view::{FixedVectorRef, VariableListRef};
use ssz_primitives::{U128, U256};
use ssz_derive::{Encode, Decode};
use tree_hash::TreeHashDigest;
use tree_hash_derive::TreeHash;
use ssz::view::*;
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct ExportEntry {
    pub value: u64,
    pub data: u32,
}
impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for ExportEntry {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.value).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.data).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`ExportEntry`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct ExportEntryRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportEntryRef<'a> {
    pub fn value(&self) -> Result<u64, ssz::DecodeError> {
        let offset = 0usize;
        let end = offset + 8usize;
        if end > self.bytes.len() {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: self.bytes.len(),
                expected: end,
            });
        }
        let bytes = &self.bytes[offset..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn data(&self) -> Result<u32, ssz::DecodeError> {
        let offset = 8usize;
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
impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for ExportEntryRef<'a> {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
        {
            let offset = 0usize;
            let field_bytes = &self.bytes[offset..offset + 8usize];
            hasher.write(field_bytes).expect("write field");
        }
        {
            let offset = 8usize;
            let field_bytes = &self.bytes[offset..offset + 4usize];
            hasher.write(field_bytes).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<'a> ssz::view::DecodeView<'a> for ExportEntryRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() != 12usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 12usize,
            });
        }
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ExportEntryRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        true
    }
    fn ssz_fixed_len() -> usize {
        12usize
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ssz_types::view::ToOwnedSsz<ExportEntry> for ExportEntryRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    fn to_owned(&self) -> ExportEntry {
        <ExportEntryRef<'a>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportEntryRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> ExportEntry {
        ExportEntry {
            value: self.value().expect("valid view"),
            data: self.data().expect("valid view"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct ExportContainer {
    pub entries: VariableList<ExportEntry, 4096usize>,
    pub name: u32,
}
impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for ExportContainer {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.entries).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.name).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`ExportContainer`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct ExportContainerRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportContainerRef<'a> {
    pub fn entries(
        &self,
    ) -> Result<VariableListRef<'a, ExportEntryRef<'a>, 4096usize>, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            8usize,
            1usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            8usize,
            1usize,
            1usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn name(&self) -> Result<u32, ssz::DecodeError> {
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
for ExportContainerRef<'a> {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(2usize);
        {
            let entries = self.entries().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&entries);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let offset = 4usize;
            let field_bytes = &self.bytes[offset..offset + 4usize];
            hasher.write(field_bytes).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<'a> ssz::view::DecodeView<'a> for ExportContainerRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 8usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 8usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..1usize {
            let offset = ssz::layout::read_variable_offset(bytes, 8usize, 1usize, i)?;
            if i == 0 && offset != 8usize {
                return Err(ssz::DecodeError::OffsetIntoFixedPortion(offset));
            }
            if let Some(prev) = prev_offset && offset < prev {
                return Err(ssz::DecodeError::OffsetsAreDecreasing(offset));
            }
            if offset > bytes.len() {
                return Err(ssz::DecodeError::OffsetOutOfBounds(offset));
            }
            prev_offset = Some(offset);
        }
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ExportContainerRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        false
    }
    fn ssz_fixed_len() -> usize {
        0
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ssz_types::view::ToOwnedSsz<ExportContainer> for ExportContainerRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    fn to_owned(&self) -> ExportContainer {
        <ExportContainerRef<'a>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportContainerRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> ExportContainer {
        ExportContainer {
            entries: self.entries().expect("valid view").to_owned().expect("valid view"),
            name: self.name().expect("valid view"),
        }
    }
}
