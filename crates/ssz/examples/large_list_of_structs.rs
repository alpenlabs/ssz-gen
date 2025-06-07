// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Encode and decode a list many times.
//!
//! Useful for `cargo flamegraph`.

use alloy_primitives as _;
use itertools as _;
use serde as _;
use serde_json as _;
use serde_utils as _;
use smallvec as _;
use typenum as _;

use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};

#[derive(Clone, Copy, Encode, Decode)]
pub struct FixedLen {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

fn main() {
    let fixed_len = FixedLen {
        a: 42,
        b: 42,
        c: 42,
        d: 42,
    };

    let vec: Vec<FixedLen> = vec![fixed_len; 8196];

    let output: Vec<Vec<u64>> = (0..40_000)
        .map(|_| Vec::from_ssz_bytes(&vec.as_ssz_bytes()).unwrap())
        .collect();

    println!("{}", output.len());
}
