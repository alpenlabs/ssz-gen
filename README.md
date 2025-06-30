# ssz-gen

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache-blue.svg)](https://opensource.org/licenses/apache-2-0)
[![ci](https://github.com/alpenlabs/ssz-gen/actions/workflows/lint.yml/badge.svg?event=push)](https://github.com/alpenlabs/ssz-gen/actions)
[![docs](https://img.shields.io/badge/docs-docs.rs-orange)](https://docs.rs/ssz-gen)

A Rust codegen tool that generates Rust code from pythonic SSZ (Simple Serialize) definitions. This project parses Python-style SSZ schema definitions using [`sizzle-parser`](https://codeberg.org/treyd/sizzle-parser/) and generates equivalent Rust implementations utilizing modified versions of libraries from [`sigp`](https://github.com/sigp):

- `ethereum_ssz` (`ssz` and `ssz_derive`): For SSZ encoding / decoding
- `ssz_types`: For SSZ types such as List, Vector, Bitfield, etc
- `tree_hash` (`tree_hash` and `tree_hash_derive`): For merklelization

These libraries have been modified to add StableContainer support and other enhancements.

## Features

- Support for `Container`, `StableContainer`, and `Profile` types
- Inheritance in container definitions
- Union types
- Constants and type aliases
- Built-in type aliases (`byte`, `bit`, `null`, `BytesX`)

## Usage

### 1. Create a `build.rs` file in your crate root:

```rust
use ssz_codegen::build_ssz_files;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&out_dir).join("generated_ssz.rs");
    
    build_ssz_files(
        &["schema.ssz"],           // Entry point SSZ files
        "specs/",                  // Base directory containing SSZ files
        output_path.to_str().unwrap(),
    )
    .expect("Failed to generate SSZ types");
}
```

### 2. Include the generated code in your `lib.rs`:

```rust
use ssz_types::*;
use ssz::{Decode, Encode};
use tree_hash::TreeHash;

include!(concat!(env!("OUT_DIR"), "/generated_ssz.rs"));

// Your SSZ types are now available
pub fn example() {
    let data = crate::specs::schema::MyContainer { /* ... */ };
    // Use SSZ encoding/decoding, tree hashing, etc.
}
```

## Contributing

Contributions are generally welcome.
If you intend to make larger changes please discuss them in an issue
before opening a PR to avoid duplicate work and architectural mismatches.

For more information please see [`CONTRIBUTING.md`](/CONTRIBUTING.md).

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.