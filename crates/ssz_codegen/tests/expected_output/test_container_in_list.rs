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
#[ssz(struct_behaviour = "container")]
pub struct ExportEntry {
    pub value: u64,
    pub data: u32,
}
impl tree_hash::TreeHash for ExportEntry {
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
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.value).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.data).as_ref())
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
#[derive(
    std::clone::Clone,
    std::fmt::Debug,
    std::cmp::PartialEq,
    std::cmp::Eq,
    std::marker::Copy
)]
pub struct ExportEntryRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportEntryRef<'a> {
    pub fn value(&self) -> Result<u64, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            <u64 as ssz::Encode>::is_ssz_fixed_len(),
            0usize,
            <u64 as ssz::Encode>::ssz_fixed_len(),
            <u64 as ssz::Encode>::ssz_fixed_len()
                + <u32 as ssz::Encode>::ssz_fixed_len(),
            usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
            0usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn data(&self) -> Result<u32, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            <u32 as ssz::Encode>::is_ssz_fixed_len(),
            <u64 as ssz::Encode>::ssz_fixed_len(),
            <u32 as ssz::Encode>::ssz_fixed_len(),
            <u64 as ssz::Encode>::ssz_fixed_len()
                + <u32 as ssz::Encode>::ssz_fixed_len(),
            usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
            usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
}
impl<'a> tree_hash::TreeHash for ExportEntryRef<'a> {
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
            let value = self.value().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&value);
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
impl<'a> ssz::view::DecodeView<'a> for ExportEntryRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        let fixed_portion_size = <u64 as ssz::Encode>::ssz_fixed_len()
            + <u32 as ssz::Encode>::ssz_fixed_len();
        let num_variable_fields = usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len());
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
        }
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ExportEntryRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
            + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()) == 0
    }
    fn ssz_fixed_len() -> usize {
        if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
            <u64 as ssz::Encode>::ssz_fixed_len() + <u32 as ssz::Encode>::ssz_fixed_len()
        } else {
            0
        }
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
#[derive(
    std::clone::Clone,
    std::fmt::Debug,
    std::cmp::PartialEq,
    std::cmp::Eq,
    ssz_derive::Encode,
    ssz_derive::Decode
)]
#[ssz(struct_behaviour = "container")]
pub struct ExportContainer {
    pub entries: VariableList<ExportEntry, 4096usize>,
    pub name: u32,
}
impl tree_hash::TreeHash for ExportContainer {
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
                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.entries).as_ref(),
            )
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.name).as_ref())
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
#[derive(
    std::clone::Clone,
    std::fmt::Debug,
    std::cmp::PartialEq,
    std::cmp::Eq,
    std::marker::Copy
)]
pub struct ExportContainerRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ExportContainerRef<'a> {
    pub fn entries(
        &self,
    ) -> Result<ListRef<'a, ExportEntryRef<'a>, 4096usize>, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
            0usize,
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::ssz_fixed_len(),
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::ssz_fixed_len()
                + <u32 as ssz::Encode>::ssz_fixed_len(),
            usize::from(
                !<VariableList<
                    ExportEntry,
                    4096usize,
                > as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
            0usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn name(&self) -> Result<u32, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            <u32 as ssz::Encode>::is_ssz_fixed_len(),
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::ssz_fixed_len(),
            <u32 as ssz::Encode>::ssz_fixed_len(),
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::ssz_fixed_len()
                + <u32 as ssz::Encode>::ssz_fixed_len(),
            usize::from(
                !<VariableList<
                    ExportEntry,
                    4096usize,
                > as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()),
            usize::from(
                !<VariableList<
                    ExportEntry,
                    4096usize,
                > as ssz::Encode>::is_ssz_fixed_len(),
            ),
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
}
impl<'a> tree_hash::TreeHash for ExportContainerRef<'a> {
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
            let entries = self.entries().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&entries);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let name = self.name().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&name);
            hasher.write(root.as_ref()).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<'a> ssz::view::DecodeView<'a> for ExportContainerRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        let fixed_portion_size = <VariableList<
            ExportEntry,
            4096usize,
        > as ssz::Encode>::ssz_fixed_len() + <u32 as ssz::Encode>::ssz_fixed_len();
        let num_variable_fields = usize::from(
            !<VariableList<ExportEntry, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
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
        }
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ExportContainerRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        usize::from(
            !<VariableList<ExportEntry, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
        ) + usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len()) == 0
    }
    fn ssz_fixed_len() -> usize {
        if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
            <VariableList<ExportEntry, 4096usize> as ssz::Encode>::ssz_fixed_len()
                + <u32 as ssz::Encode>::ssz_fixed_len()
        } else {
            0
        }
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
            entries: {
                let view = self.entries().expect("valid view");
                let items: Result<Vec<_>, _> = view
                    .iter()
                    .map(|item_result| {
                        item_result
                            .map(|item| ssz_types::view::ToOwnedSsz::to_owned(&item))
                    })
                    .collect();
                let items = items.expect("valid view");
                ssz_types::VariableList::new(items).expect("valid view")
            },
            name: self.name().expect("valid view"),
        }
    }
}
