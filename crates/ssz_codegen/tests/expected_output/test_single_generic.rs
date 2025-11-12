#![allow(unused_imports, reason = "generated code using ssz-gen")]
use ssz_types::*;
use ssz_types::view::{FixedVectorRef, VariableListRef};
use ssz_derive::{Encode, Decode};
use tree_hash::TreeHashDigest;
use tree_hash_derive::TreeHash;
use ssz::view::*;
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
pub struct RawMerkleProof<H: ssz::Encode + ssz::Decode + MerkleHash> {
    pub cohashes: VariableList<H, 1024usize>,
}
impl<
    H: tree_hash::TreeHashDigest + tree_hash::TreeHash<H> + ssz::Encode + ssz::Decode
        + MerkleHash,
> tree_hash::TreeHash<H> for RawMerkleProof<H> {
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
        let mut hasher = tree_hash::MerkleHasher::<H>::with_leaves(1usize);
        hasher
            .write(
                <_ as tree_hash::TreeHash<H>>::tree_hash_root(&self.cohashes).as_ref(),
            )
            .expect("tree hash derive should not apply too many leaves");
        hasher.finish().expect("tree hash derive should not have a remaining buffer")
    }
}
/// Zero-copy view over [`RawMerkleProof`].
///
/// This type wraps SSZ-encoded bytes without allocating. Fields are accessed
/// via lazy getter methods. Use `.to_owned()` to convert to the owned type when
/// needed.
#[allow(dead_code, reason = "generated code using ssz-gen")]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct RawMerkleProofRef<
    'a,
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> {
    bytes: &'a [u8],
    _phantom: core::marker::PhantomData<(H,)>,
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> RawMerkleProofRef<'a, H> {
    pub fn cohashes(
        &self,
    ) -> Result<VariableListRef<'a, H, 1024usize>, ssz::DecodeError> {
        let start = ssz::layout::read_variable_offset(
            self.bytes,
            4usize,
            1usize,
            0usize,
        )?;
        let end = ssz::layout::read_variable_offset_or_end(
            self.bytes,
            4usize,
            1usize,
            1usize,
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
    H: tree_hash::TreeHashDigest + tree_hash::TreeHash<H> + ssz::Encode + ssz::Decode
        + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo + 'a + MerkleHash,
> tree_hash::TreeHash<H> for RawMerkleProofRef<'a, H> {
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
            let cohashes = self.cohashes().expect("valid view");
            let root: <H as tree_hash::TreeHashDigest>::Output = tree_hash::TreeHash::<
                H,
            >::tree_hash_root(&cohashes);
            hasher.write(root.as_ref()).expect("write field");
        }
        hasher.finish().expect("finish hasher")
    }
}
impl<
    'a,
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> ssz::view::DecodeView<'a> for RawMerkleProofRef<'a, H> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
        if bytes.len() < 4usize {
            return Err(ssz::DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: 4usize,
            });
        }
        let mut prev_offset: Option<usize> = None;
        for i in 0..1usize {
            let offset = ssz::layout::read_variable_offset(bytes, 4usize, 1usize, i)?;
            if i == 0 && offset != 4usize {
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
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> ssz::view::SszTypeInfo for RawMerkleProofRef<'a, H> {
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
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> ssz_types::view::ToOwnedSsz<RawMerkleProof<H>> for RawMerkleProofRef<'a, H> {
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    fn to_owned(&self) -> RawMerkleProof<H> {
        <RawMerkleProofRef<'a, H>>::to_owned(self)
    }
}
#[allow(dead_code, reason = "generated code using ssz-gen")]
impl<
    'a,
    H: ssz::Encode + ssz::Decode + ssz::view::DecodeView<'a> + ssz::view::SszTypeInfo
        + 'a + MerkleHash,
> RawMerkleProofRef<'a, H>
where
    H: ssz_types::view::ToOwnedSsz<H>,
{
    #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
    pub fn to_owned(&self) -> RawMerkleProof<H> {
        RawMerkleProof {
            cohashes: self
                .cohashes()
                .expect("valid view")
                .to_owned()
                .expect("valid view"),
        }
    }
}
