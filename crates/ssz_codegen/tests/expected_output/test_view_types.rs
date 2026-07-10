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
    pub key: u32,
    pub value: u64,
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
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.key).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.value).as_ref())
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
    pub fn key(&self) -> Result<u32, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            &[
                (
                    <u32 as ssz::Encode>::is_ssz_fixed_len(),
                    <u32 as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <u64 as ssz::Encode>::is_ssz_fixed_len(),
                    <u64 as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
            0usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn value(&self) -> Result<u64, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            &[
                (
                    <u32 as ssz::Encode>::is_ssz_fixed_len(),
                    <u32 as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <u64 as ssz::Encode>::is_ssz_fixed_len(),
                    <u64 as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
            1usize,
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
            let key = self.key().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&key);
            hasher.write(root.as_ref()).expect("write field");
        }
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
impl<'a> ssz::view::DecodeView<'a> for ExportEntryRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        ssz::layout::validate_container(
            bytes,
            &[
                (
                    <u32 as ssz::Encode>::is_ssz_fixed_len(),
                    <u32 as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <u64 as ssz::Encode>::is_ssz_fixed_len(),
                    <u64 as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
        )?;
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ExportEntryRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        usize::from(!<u32 as ssz::Encode>::is_ssz_fixed_len())
            + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
    }
    fn ssz_fixed_len() -> usize {
        if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
            <u32 as ssz::Encode>::ssz_fixed_len() + <u64 as ssz::Encode>::ssz_fixed_len()
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
            key: self.key().expect("valid view"),
            value: self.value().expect("valid view"),
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
pub struct ViewTypeTest {
    pub payload: VariableList<u8, 4096usize>,
    pub entries: VariableList<ExportEntry, 256usize>,
    pub hash: FixedBytes<32usize>,
}
impl tree_hash::TreeHash for ViewTypeTest {
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
                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.payload).as_ref(),
            )
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(
                <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.entries).as_ref(),
            )
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.hash).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`ViewTypeTest`].
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
pub struct ViewTypeTestRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ViewTypeTestRef<'a> {
    pub fn payload(&self) -> Result<BytesRef<'a, 4096usize>, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            &[
                (
                    <VariableList<u8, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<u8, 4096usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <VariableList<
                        ExportEntry,
                        256usize,
                    > as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<ExportEntry, 256usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
            0usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn entries(
        &self,
    ) -> Result<ListRef<'a, ExportEntryRef<'a>, 256usize>, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            &[
                (
                    <VariableList<u8, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<u8, 4096usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <VariableList<
                        ExportEntry,
                        256usize,
                    > as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<ExportEntry, 256usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
            1usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn hash(&self) -> Result<FixedBytesRef<'a, 32usize>, ssz::DecodeError> {
        let bytes = ssz::layout::read_field_bytes(
            self.bytes,
            &[
                (
                    <VariableList<u8, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<u8, 4096usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <VariableList<
                        ExportEntry,
                        256usize,
                    > as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<ExportEntry, 256usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
            2usize,
        )?;
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
}
impl<'a> tree_hash::TreeHash for ViewTypeTestRef<'a> {
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
            let payload = self.payload().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&payload);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let entries = self.entries().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&entries);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let hash = self.hash().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                H,
            >(&hash);
            hasher.write(root.as_ref()).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<'a> ssz::view::DecodeView<'a> for ViewTypeTestRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        ssz::layout::validate_container(
            bytes,
            &[
                (
                    <VariableList<u8, 4096usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<u8, 4096usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <VariableList<
                        ExportEntry,
                        256usize,
                    > as ssz::Encode>::is_ssz_fixed_len(),
                    <VariableList<ExportEntry, 256usize> as ssz::Encode>::ssz_fixed_len(),
                ),
                (
                    <FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len(),
                    <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len(),
                ),
            ],
        )?;
        Ok(Self { bytes })
    }
}
impl<'a> ssz::view::SszTypeInfo for ViewTypeTestRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        usize::from(!<VariableList<u8, 4096usize> as ssz::Encode>::is_ssz_fixed_len())
            + usize::from(
                !<VariableList<ExportEntry, 256usize> as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<FixedBytes<32usize> as ssz::Encode>::is_ssz_fixed_len())
            == 0
    }
    fn ssz_fixed_len() -> usize {
        if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
            <VariableList<u8, 4096usize> as ssz::Encode>::ssz_fixed_len()
                + <VariableList<ExportEntry, 256usize> as ssz::Encode>::ssz_fixed_len()
                + <FixedBytes<32usize> as ssz::Encode>::ssz_fixed_len()
        } else {
            0
        }
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ssz_types::view::ToOwnedSsz<ViewTypeTest> for ViewTypeTestRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    fn to_owned(&self) -> ViewTypeTest {
        <ViewTypeTestRef<'a>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ViewTypeTestRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> ViewTypeTest {
        ViewTypeTest {
            payload: ssz_types::VariableList::new(
                    self.payload().expect("valid view").to_owned(),
                )
                .expect("valid view"),
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
            hash: ssz_types::FixedBytes(self.hash().expect("valid view").to_owned()),
        }
    }
}
