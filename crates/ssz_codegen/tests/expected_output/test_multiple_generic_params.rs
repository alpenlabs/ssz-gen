#![allow(unused_imports, reason = "generated code using ssz-gen")]
use ssz_types::*;
use ssz_types::view::{FixedVectorRef, VariableListRef};
use ssz_derive::{Encode, Decode};
use tree_hash::TreeHashDigest;
use tree_hash_derive::TreeHash;
use ssz::view::*;
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct Pair<T: ssz::Encode + ssz::Decode, U: ssz::Encode + ssz::Decode> {
    pub first: T,
    pub second: U,
}
impl<
    T: tree_hash::TreeHashDigest + tree_hash::TreeHash<T> + ssz::Encode + ssz::Decode,
    U: tree_hash::TreeHashDigest + tree_hash::TreeHash<U> + ssz::Encode + ssz::Decode,
> tree_hash::TreeHash<H> for Pair<T, U> {
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
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.first).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.second).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`Pair`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct PairRef<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> {
    bytes: &'a [u8],
    _phantom: core::marker::PhantomData<(T, U)>,
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> PairRef<'a, T, U> {
    pub fn first(&self) -> Result<T, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            8usize,
            2usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            8usize,
            2usize,
            1usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn second(&self) -> Result<U, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            8usize,
            2usize,
            1usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            8usize,
            2usize,
            2usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
}
impl<
    'a,
    T: tree_hash::TreeHashDigest + tree_hash::TreeHash<T> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a,
    U: tree_hash::TreeHashDigest + tree_hash::TreeHash<U> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a,
> tree_hash::TreeHash<H> for PairRef<'a, T, U> {
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
            let first = self.first().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&first);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let second = self.second().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&second);
            hasher.write(root.as_ref()).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::DecodeView<'a> for PairRef<'a, T, U> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 8usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 8usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..2usize {
            let offset = ssz::layout::read_variable_offset(bytes, 8usize, 2usize, i)?;
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
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::SszTypeInfo for PairRef<'a, T, U> {
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
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz_types::view::ToOwnedSsz<Pair<T, U>> for PairRef<'a, T, U> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    #[allow(
        unconditional_recursion,
        reason = "false positive - delegates to inherent method"
    )]
    fn to_owned(&self) -> Pair<T, U> {
        <PairRef<'a, T, U>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> PairRef<'a, T, U>
where
    T: ssz_types::view::ToOwnedSsz<T>,
    U: ssz_types::view::ToOwnedSsz<U>,
{
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> Pair<T, U> {
        Pair {
            first: self.first().expect("valid view").to_owned(),
            second: self.second().expect("valid view").to_owned(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct Triple<
    T: ssz::Encode + ssz::Decode,
    U: ssz::Encode + ssz::Decode,
    V: ssz::Encode + ssz::Decode,
> {
    pub first: T,
    pub second: U,
    pub third: V,
    pub count: u64,
}
impl<
    T: tree_hash::TreeHashDigest + tree_hash::TreeHash<T> + ssz::Encode + ssz::Decode,
    U: tree_hash::TreeHashDigest + tree_hash::TreeHash<U> + ssz::Encode + ssz::Decode,
    V: tree_hash::TreeHashDigest + tree_hash::TreeHash<V> + ssz::Encode + ssz::Decode,
> tree_hash::TreeHash<H> for Triple<T, U, V> {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(4usize);
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.first).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.second).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.third).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher
            .write(<_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.count).as_ref())
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`Triple`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct TripleRef<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> {
    bytes: &'a [u8],
    _phantom: core::marker::PhantomData<(T, U, V)>,
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> TripleRef<'a, T, U, V> {
    pub fn first(&self) -> Result<T, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            20usize,
            3usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            20usize,
            3usize,
            1usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn second(&self) -> Result<U, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            20usize,
            3usize,
            1usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            20usize,
            3usize,
            2usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn third(&self) -> Result<V, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            20usize,
            3usize,
            2usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            20usize,
            3usize,
            3usize,
        )?;
        if start > end || end > self.bytes.len() {
            return Err(ssz::DecodeError::OffsetsAreDecreasing(end));
        }
        let bytes = &self.bytes[start..end];
        ssz::view::DecodeView::from_ssz_bytes(bytes)
    }
    pub fn count(&self) -> Result<u64, ssz::DecodeError> {
        let offset = 12usize;
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
    U: tree_hash::TreeHashDigest + tree_hash::TreeHash<U> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a,
    V: tree_hash::TreeHashDigest + tree_hash::TreeHash<V> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a,
> tree_hash::TreeHash<H> for TripleRef<'a, T, U, V> {
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
            let first = self.first().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&first);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let second = self.second().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&second);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let third = self.third().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&third);
            hasher.write(root.as_ref()).expect("write field");
        }
        {
            let offset = 12usize;
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
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::DecodeView<'a> for TripleRef<'a, T, U, V> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 20usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 20usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..3usize {
            let offset = ssz::layout::read_variable_offset(bytes, 20usize, 3usize, i)?;
            if i == 0 && offset != 20usize {
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
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz::view::SszTypeInfo for TripleRef<'a, T, U, V> {
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
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> ssz_types::view::ToOwnedSsz<Triple<T, U, V>> for TripleRef<'a, T, U, V> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    #[allow(
        unconditional_recursion,
        reason = "false positive - delegates to inherent method"
    )]
    fn to_owned(&self) -> Triple<T, U, V> {
        <TripleRef<'a, T, U, V>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    T: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    U: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
    V: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a,
> TripleRef<'a, T, U, V>
where
    T: ssz_types::view::ToOwnedSsz<T>,
    U: ssz_types::view::ToOwnedSsz<U>,
    V: ssz_types::view::ToOwnedSsz<V>,
{
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> Triple<T, U, V> {
        Triple {
            first: self.first().expect("valid view").to_owned(),
            second: self.second().expect("valid view").to_owned(),
            third: self.third().expect("valid view").to_owned(),
            count: self.count().expect("valid view"),
        }
    }
}
