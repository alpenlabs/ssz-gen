#![allow(missing_docs)]
#![allow(unused_crate_dependencies)]

use ssz::view::{BytesRef, VectorRef};

#[test]
fn vectorref_get_variable_offsets() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&8u32.to_le_bytes());
    bytes.extend_from_slice(&10u32.to_le_bytes());
    bytes.extend_from_slice(b"aa");
    bytes.extend_from_slice(b"bbb");

    let view = VectorRef::<BytesRef<'_>, 2>::new(&bytes).unwrap();
    let second = view.get(1).unwrap();

    assert_eq!(second.as_bytes(), b"bbb");
}
