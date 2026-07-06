pub mod test_cross_entry_state {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    use ssz_types::*;
    use ssz_types::view::{FixedVectorRef, VariableListRef};
    use ssz_primitives::{U128, U256};
    use ssz_derive::{Encode, Decode};
    use tree_hash::TreeHashDigest;
    use tree_hash_derive::TreeHash;
    use ssz::view::*;
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    pub const MAX_VK_BYTES: u64 = 48u64;
    #[derive(
        std::clone::Clone,
        std::fmt::Debug,
        std::cmp::PartialEq,
        std::cmp::Eq,
        ssz_derive::Encode,
        ssz_derive::Decode
    )]
    #[ssz(struct_behaviour = "container")]
    pub struct State {
        pub data: FixedBytes<48usize>,
        pub counter: u64,
    }
    impl tree_hash::TreeHash for State {
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
                    <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.data).as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher
                .write(
                    <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.counter)
                        .as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher.finish().expect("tree hash derive should not have a remaining buffer")
        }
    }
    /// Zero-copy view over [`State`].
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
    pub struct StateRef<'a> {
        bytes: &'a [u8],
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> StateRef<'a> {
        pub fn data(&self) -> Result<FixedBytesRef<'a, 48usize>, ssz::DecodeError> {
            let bytes = ssz::layout::read_field_bytes(
                self.bytes,
                <FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len(),
                0usize,
                <FixedBytes<48usize> as ssz::Encode>::ssz_fixed_len(),
                <FixedBytes<48usize> as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len(),
                usize::from(!<FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len())
                    + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
                0usize,
            )?;
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
        pub fn counter(&self) -> Result<u64, ssz::DecodeError> {
            let bytes = ssz::layout::read_field_bytes(
                self.bytes,
                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                <FixedBytes<48usize> as ssz::Encode>::ssz_fixed_len(),
                <u64 as ssz::Encode>::ssz_fixed_len(),
                <FixedBytes<48usize> as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len(),
                usize::from(!<FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len())
                    + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
                usize::from(!<FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len()),
            )?;
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
    }
    impl<'a> tree_hash::TreeHash for StateRef<'a> {
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
                let data = self.data().expect("valid view");
                let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                    H,
                >(&data);
                hasher.write(root.as_ref()).expect("write field");
            }
            {
                let counter = self.counter().expect("valid view");
                let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                    H,
                >(&counter);
                hasher.write(root.as_ref()).expect("write field");
            }
            hasher.finish().expect("finish hasher")
        }
    }
    impl<'a> ssz::view::DecodeView<'a> for StateRef<'a> {
        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
            let fixed_portion_size = <FixedBytes<
                48usize,
            > as ssz::Encode>::ssz_fixed_len() + <u64 as ssz::Encode>::ssz_fixed_len();
            let num_variable_fields = usize::from(
                !<FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len());
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
    impl<'a> ssz::view::SszTypeInfo for StateRef<'a> {
        fn is_ssz_fixed_len() -> bool {
            usize::from(!<FixedBytes<48usize> as ssz::Encode>::is_ssz_fixed_len())
                + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()) == 0
        }
        fn ssz_fixed_len() -> usize {
            if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                <FixedBytes<48usize> as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len()
            } else {
                0
            }
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> ssz_types::view::ToOwnedSsz<State> for StateRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        fn to_owned(&self) -> State {
            <StateRef<'a>>::to_owned(self)
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> StateRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        pub fn to_owned(&self) -> State {
            State {
                data: ssz_types::FixedBytes(self.data().expect("valid view").to_owned()),
                counter: self.counter().expect("valid view"),
            }
        }
    }
}
pub mod test_cross_entry_update {
    #![allow(unused_imports, reason = "generated code using ssz-gen")]
    use ssz_types::*;
    use ssz_types::view::{FixedVectorRef, VariableListRef};
    use ssz_primitives::{U128, U256};
    use ssz_derive::{Encode, Decode};
    use tree_hash::TreeHashDigest;
    use tree_hash_derive::TreeHash;
    use ssz::view::*;
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    pub const MAX_UPDATES: u64 = 10u64;
    #[derive(
        std::clone::Clone,
        std::fmt::Debug,
        std::cmp::PartialEq,
        std::cmp::Eq,
        ssz_derive::Encode,
        ssz_derive::Decode
    )]
    #[ssz(struct_behaviour = "container")]
    pub struct Update {
        pub state: crate::tests::input::test_cross_entry_state::State,
        pub timestamp: u64,
        pub updates: VariableList<u8, 10usize>,
    }
    impl tree_hash::TreeHash for Update {
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
                    <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.state).as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher
                .write(
                    <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.timestamp)
                        .as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher
                .write(
                    <_ as tree_hash::TreeHash>::tree_hash_root::<H>(&self.updates)
                        .as_ref(),
                )
                .expect("tree hash derive should not apply too many leaves");
            hasher.finish().expect("tree hash derive should not have a remaining buffer")
        }
    }
    /// Zero-copy view over [`Update`].
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
    pub struct UpdateRef<'a> {
        bytes: &'a [u8],
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> UpdateRef<'a> {
        pub fn state(
            &self,
        ) -> Result<
            crate::tests::input::test_cross_entry_state::StateRef<'a>,
            ssz::DecodeError,
        > {
            let bytes = ssz::layout::read_field_bytes(
                self.bytes,
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                0usize,
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len(),
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len()
                    + <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len(),
                usize::from(
                    !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                    + usize::from(
                        !<VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                    ),
                0usize,
            )?;
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
        pub fn timestamp(&self) -> Result<u64, ssz::DecodeError> {
            let bytes = ssz::layout::read_field_bytes(
                self.bytes,
                <u64 as ssz::Encode>::is_ssz_fixed_len(),
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len(),
                <u64 as ssz::Encode>::ssz_fixed_len(),
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len()
                    + <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len(),
                usize::from(
                    !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                    + usize::from(
                        !<VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                    ),
                usize::from(
                    !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                ),
            )?;
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
        pub fn updates(&self) -> Result<BytesRef<'a, 10usize>, ssz::DecodeError> {
            let bytes = ssz::layout::read_field_bytes(
                self.bytes,
                <VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len(),
                <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len(),
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len()
                    + <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len(),
                usize::from(
                    !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                    + usize::from(
                        !<VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                    ),
                usize::from(
                    !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
                ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len()),
            )?;
            ssz::view::DecodeView::from_ssz_bytes(bytes)
        }
    }
    impl<'a> tree_hash::TreeHash for UpdateRef<'a> {
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
                let state = self.state().expect("valid view");
                let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                    H,
                >(&state);
                hasher.write(root.as_ref()).expect("write field");
            }
            {
                let timestamp = self.timestamp().expect("valid view");
                let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                    H,
                >(&timestamp);
                hasher.write(root.as_ref()).expect("write field");
            }
            {
                let updates = self.updates().expect("valid view");
                let root: <H as tree_hash::TreeHashDigest>::Output = <_ as tree_hash::TreeHash>::tree_hash_root::<
                    H,
                >(&updates);
                hasher.write(root.as_ref()).expect("write field");
            }
            hasher.finish().expect("finish hasher")
        }
    }
    impl<'a> ssz::view::DecodeView<'a> for UpdateRef<'a> {
        fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
            let fixed_portion_size = <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                + <u64 as ssz::Encode>::ssz_fixed_len()
                + <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len();
            let num_variable_fields = usize::from(
                !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                + usize::from(
                    !<VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                );
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
    impl<'a> ssz::view::SszTypeInfo for UpdateRef<'a> {
        fn is_ssz_fixed_len() -> bool {
            usize::from(
                !<crate::tests::input::test_cross_entry_state::State as ssz::Encode>::is_ssz_fixed_len(),
            ) + usize::from(!<u64 as ssz::Encode>::is_ssz_fixed_len())
                + usize::from(
                    !<VariableList<u8, 10usize> as ssz::Encode>::is_ssz_fixed_len(),
                ) == 0
        }
        fn ssz_fixed_len() -> usize {
            if <Self as ssz::view::SszTypeInfo>::is_ssz_fixed_len() {
                <crate::tests::input::test_cross_entry_state::State as ssz::Encode>::ssz_fixed_len()
                    + <u64 as ssz::Encode>::ssz_fixed_len()
                    + <VariableList<u8, 10usize> as ssz::Encode>::ssz_fixed_len()
            } else {
                0
            }
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> ssz_types::view::ToOwnedSsz<Update> for UpdateRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        fn to_owned(&self) -> Update {
            <UpdateRef<'a>>::to_owned(self)
        }
    }
    #[allow(dead_code, reason = "generated code using ssz-gen")]
    impl<'a> UpdateRef<'a> {
        #[allow(clippy::wrong_self_convention, reason = "API convention for view types")]
        pub fn to_owned(&self) -> Update {
            Update {
                state: {
                    let view = self.state().expect("valid view");
                    ssz_types::view::ToOwnedSsz::to_owned(&view)
                },
                timestamp: self.timestamp().expect("valid view"),
                updates: ssz_types::VariableList::new(
                        self.updates().expect("valid view").to_owned(),
                    )
                    .expect("valid view"),
            }
        }
    }
}
