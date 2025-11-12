#![allow(unused_imports, reason = "generated code using ssz-gen")]
use ssz_types::*;
use ssz_types::view::{FixedVectorRef, VariableListRef};
use ssz_derive::{Encode, Decode};
use tree_hash::TreeHashDigest;
use tree_hash_derive::TreeHash;
use ssz::view::*;
pub type Hash32 = FixedBytes<32usize>;
pub type BoxHash32 = Box<Hash32>;
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct Box<T: ssz::Encode + ssz::Decode> {
    pub value: T,
    pub count: u64,
}
impl<
    T: tree_hash::TreeHashDigest + tree_hash::TreeHash<T> + ssz::Encode + ssz::Decode,
> tree_hash::TreeHash<H> for Box<T> {
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
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.count).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`Box`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct BoxRef<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> {
    bytes: &'a [u8],
    _phantom: core::marker::PhantomData<(T,)>,
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> BoxRef<'a, T> {
    pub fn value(&self) -> Result<T, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            12usize,
            1usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            12usize,
            1usize,
            1usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn count(&self) -> Result<u64, ssz::DecodeError> {
        let offset = 4usize;
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
}
impl<
    'a,
    T: tree_hash::TreeHashDigest + tree_hash::TreeHash<T> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a,
> tree_hash::TreeHash<H> for BoxRef<'a, T> {
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
            let value = self.value().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&value);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let offset = 4usize;
            let field_bytes = &self.bytes[offset..offset + 8usize];
            hasher.write(field_bytes).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::DecodeView<'a> for BoxRef<'a, T> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 12usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 12usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..1usize {
            let offset = ssz::layout::read_variable_offset(bytes, 12usize, 1usize, i)?;
            if i == 0 && offset != 12usize {
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
        Ok(Self {
            bytes,
            _phantom: core::marker::PhantomData,
        })
    }
}
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::SszTypeInfo for BoxRef<'a, T> {
    fn is_ssz_fixed_len() -> bool {
        false
    }
    fn ssz_fixed_len() -> usize {
        0
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz_types::view::ToOwnedSsz<Box<T>> for BoxRef<'a, T> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    #[allow(
        unconditional_recursion,
        reason = "false positive - delegates to inherent method"
    )]
    fn to_owned(&self) -> Box<T> {
        <BoxRef<'a, T>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> BoxRef<'a, T>
where
    T: ssz_types::view::ToOwnedSsz<T>,
{
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> Box<T> {
        Box {
            value: self.value().expect("valid view").to_owned(),
            count: self.count().expect("valid view"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct Warehouse {
    pub item: BoxHash32,
    pub id: u64,
}
impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Warehouse {
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
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.item).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.id).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`Warehouse`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct WarehouseRef<'a> {
    bytes: &'a [u8],
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> WarehouseRef<'a> {
    pub fn item(&self) -> Result<BoxHash32Ref<'a>, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            12usize,
            1usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            12usize,
            1usize,
            1usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn id(&self) -> Result<u64, ssz::DecodeError> {
        let offset = 4usize;
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
}
impl<'a, H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for WarehouseRef<'a> {
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
            let item = self.item().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&item);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let offset = 4usize;
            let field_bytes = &self.bytes[offset..offset + 8usize];
            hasher.write(field_bytes).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<'a> ssz::view::DecodeView<'a> for WarehouseRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 12usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 12usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..1usize {
            let offset = ssz::layout::read_variable_offset(bytes, 12usize, 1usize, i)?;
            if i == 0 && offset != 12usize {
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
impl<'a> ssz::view::SszTypeInfo for WarehouseRef<'a> {
    fn is_ssz_fixed_len() -> bool {
        false
    }
    fn ssz_fixed_len() -> usize {
        0
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> ssz_types::view::ToOwnedSsz<Warehouse> for WarehouseRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    #[allow(
        unconditional_recursion,
        reason = "false positive - delegates to inherent method"
    )]
    fn to_owned(&self) -> Warehouse {
        <WarehouseRef<'a>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<'a> WarehouseRef<'a> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> Warehouse {
        Warehouse {
            item: self.item().expect("valid view").to_owned(),
            id: self.id().expect("valid view"),
        }
    }
}
