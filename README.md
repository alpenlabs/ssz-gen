# ssz-gen

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache-blue.svg)](https://opensource.org/licenses/apache-2-0)
[![ci](https://github.com/alpenlabs/ssz-gen/actions/workflows/lint.yml/badge.svg?event=push)](https://github.com/alpenlabs/ssz-gen/actions)
[![docs](https://img.shields.io/badge/docs-docs.rs-orange)](https://docs.rs/ssz-gen)

A Rust codegen tool that generates Rust code from pythonic SSZ (Simple Serialize) definitions. This project parses Python-style SSZ schema definitions using [`sizzle-parser`](https://codeberg.org/treyd/sizzle-parser/) and generates equivalent Rust implementations utilizing modified versions of libraries from [`sigp`](https://github.com/sigp):

- `ethereum_ssz` (ssz and ssz_derive): For SSZ encoding / decoding
- `ssz_types`: For SSZ types such as List, Vector, Bitfield, etc
- `tree_hash` (tree_hash and tree_hash_derive): For merklelization

These libraries have been modified to add StableContainer support and other enhancements.

## Features

- Support for `Container`, `StableContainer`, and `Profile` types
- Inheritance in container definitions
- Union types
- Constants and type aliases
- Built-in type aliases (`byte`, `bit`, `null`, `BytesX`)

## Usage

There is currently no clean way to use the library as it is in very early stages, however there is a test in `ssz_codegen` that reads all the `.ssz` files in [`crates/ssz_codegen/tests/input`](crates/ssz_codegen/tests/input) and generates rust code for them in [`crates/ssz_codegen/tests/output`](crates/ssz_codegen/tests/output)

## Contributing

Contributions are generally welcome.
If you intend to make larger changes please discuss them in an issue
before opening a PR to avoid duplicate work and architectural mismatches.

For more information please see [`CONTRIBUTING.md`](/CONTRIBUTING.md).

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.