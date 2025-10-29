//! Benchmarks comparing zero-copy view types vs owned types
//!
//! This benchmark suite measures the performance difference between:
//!
//! - View types (zero-copy, reference-backed)
//! - Owned types (allocating, copying)
//!
//! Areas tested:
//!
//! - Decoding performance
//! - Tree hashing performance
//! - Memory allocation counts
//! - Iteration performance

#![allow(missing_docs, reason = "criterion macros are annoying clippy")]
#![allow(
    unused_crate_dependencies,
    reason = "criterion macros are annoying clippy"
)]

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ssz::{Decode, Encode, view::DecodeView};
use ssz_types::{
    FixedVector, VariableList,
    view::{FixedVectorRef, VariableListRef},
};
use tree_hash::{Sha256Hasher, TreeHash};

// Helper to generate `Vec<u64>` test data.
fn generate_u64_vec(size: usize) -> Vec<u64> {
    (0..size as u64).collect()
}

/// Helper to generate `Vec<u8>` test data.
fn generate_byte_vec(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Benchmark decoding a [`VariableList`] of [`u64`] values.
fn bench_decode_variable_list_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_variable_list_u64");

    for size in [10, 100, 1000, 10000] {
        let data = generate_u64_vec(size);
        let list: VariableList<u64, 16384> = data.into();
        let encoded = list.as_ssz_bytes();

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned decode
        group.bench_with_input(BenchmarkId::new("owned", size), &encoded, |b, encoded| {
            b.iter(|| {
                let list = VariableList::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                black_box(list);
            });
        });

        // Benchmark view decode
        group.bench_with_input(BenchmarkId::new("view", size), &encoded, |b, encoded| {
            b.iter(|| {
                let view = VariableListRef::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                black_box(view);
            });
        });
    }

    group.finish();
}

/// Benchmark decoding a [`VariableList`] of bytes.
fn bench_decode_variable_list_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_variable_list_bytes");

    for size in [100, 1000, 10000, 100000] {
        let data = generate_byte_vec(size);
        let list: VariableList<u8, 131072> = data.into();
        let encoded = list.as_ssz_bytes();

        group.throughput(Throughput::Bytes(size as u64));

        // Benchmark owned decode
        group.bench_with_input(BenchmarkId::new("owned", size), &encoded, |b, encoded| {
            b.iter(|| {
                let list = VariableList::<u8, 131072>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                black_box(list);
            });
        });

        // Benchmark view decode
        group.bench_with_input(BenchmarkId::new("view", size), &encoded, |b, encoded| {
            b.iter(|| {
                let view = VariableListRef::<u8, 131072>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                black_box(view);
            });
        });
    }

    group.finish();
}

/// Benchmark decoding a [`FixedVector`] of [`u64`] values.
fn bench_decode_fixed_vector_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_fixed_vector_u64");

    for size in [8, 64, 256, 1024] {
        let data = generate_u64_vec(size);

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned decode
        group.bench_function(BenchmarkId::new("owned", size), |b| {
            // Create encoding once
            let encoded = match size {
                8 => {
                    let vec: FixedVector<u64, 8> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                64 => {
                    let vec: FixedVector<u64, 64> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                256 => {
                    let vec: FixedVector<u64, 256> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                1024 => {
                    let vec: FixedVector<u64, 1024> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                _ => unreachable!(),
            };

            b.iter(|| match size {
                8 => {
                    let vec = FixedVector::<u64, 8>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(vec);
                }
                64 => {
                    let vec = FixedVector::<u64, 64>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(vec);
                }
                256 => {
                    let vec = FixedVector::<u64, 256>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(vec);
                }
                1024 => {
                    let vec = FixedVector::<u64, 1024>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(vec);
                }
                _ => unreachable!(),
            });
        });

        // Benchmark view decode
        group.bench_function(BenchmarkId::new("view", size), |b| {
            // Create encoding once
            let encoded = match size {
                8 => {
                    let vec: FixedVector<u64, 8> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                64 => {
                    let vec: FixedVector<u64, 64> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                256 => {
                    let vec: FixedVector<u64, 256> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                1024 => {
                    let vec: FixedVector<u64, 1024> = data.clone().into();
                    vec.as_ssz_bytes()
                }
                _ => unreachable!(),
            };

            b.iter(|| match size {
                8 => {
                    let view = FixedVectorRef::<u64, 8>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(view);
                }
                64 => {
                    let view = FixedVectorRef::<u64, 64>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(view);
                }
                256 => {
                    let view = FixedVectorRef::<u64, 256>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(view);
                }
                1024 => {
                    let view = FixedVectorRef::<u64, 1024>::from_ssz_bytes(black_box(&encoded))
                        .expect("decode failed");
                    black_box(view);
                }
                _ => unreachable!(),
            });
        });
    }

    group.finish();
}

/// Benchmark tree hashing a [`VariableList`] of [`u64`] values.
fn bench_tree_hash_variable_list_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_hash_variable_list_u64");

    for size in [10, 100, 1000] {
        let data = generate_u64_vec(size);
        let list: VariableList<u64, 16384> = data.into();
        let encoded = list.as_ssz_bytes();

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned tree hash
        group.bench_with_input(BenchmarkId::new("owned", size), &list, |b, list| {
            b.iter(|| {
                let root: tree_hash::Hash256 =
                    TreeHash::<Sha256Hasher>::tree_hash_root(black_box(list));
                black_box(root);
            });
        });

        // Benchmark view tree hash
        group.bench_with_input(BenchmarkId::new("view", size), &encoded, |b, encoded| {
            b.iter(|| {
                let view = VariableListRef::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
                black_box(root);
            });
        });
    }

    group.finish();
}

/// Benchmark tree hashing a [`FixedVector`] of [`u64`] values.
fn bench_tree_hash_fixed_vector_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_hash_fixed_vector_u64");

    for size in [8, 64, 256] {
        let data = generate_u64_vec(size);

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned tree hash
        group.bench_function(BenchmarkId::new("owned", size), |b| {
            let vec_owned = match size {
                8 => {
                    let v: FixedVector<u64, 8> = data.clone().into();
                    (v.as_ssz_bytes(), 8)
                }
                64 => {
                    let v: FixedVector<u64, 64> = data.clone().into();
                    (v.as_ssz_bytes(), 64)
                }
                256 => {
                    let v: FixedVector<u64, 256> = data.clone().into();
                    (v.as_ssz_bytes(), 256)
                }
                _ => unreachable!(),
            };

            b.iter(|| match vec_owned.1 {
                8 => {
                    let vec = FixedVector::<u64, 8>::from_ssz_bytes(&vec_owned.0).unwrap();
                    let root: tree_hash::Hash256 =
                        TreeHash::<Sha256Hasher>::tree_hash_root(black_box(&vec));
                    black_box(root);
                }
                64 => {
                    let vec = FixedVector::<u64, 64>::from_ssz_bytes(&vec_owned.0).unwrap();
                    let root: tree_hash::Hash256 =
                        TreeHash::<Sha256Hasher>::tree_hash_root(black_box(&vec));
                    black_box(root);
                }
                256 => {
                    let vec = FixedVector::<u64, 256>::from_ssz_bytes(&vec_owned.0).unwrap();
                    let root: tree_hash::Hash256 =
                        TreeHash::<Sha256Hasher>::tree_hash_root(black_box(&vec));
                    black_box(root);
                }
                _ => unreachable!(),
            });
        });

        // Benchmark view tree hash
        group.bench_function(BenchmarkId::new("view", size), |b| {
            let encoded = match size {
                8 => {
                    let v: FixedVector<u64, 8> = data.clone().into();
                    (v.as_ssz_bytes(), 8)
                }
                64 => {
                    let v: FixedVector<u64, 64> = data.clone().into();
                    (v.as_ssz_bytes(), 64)
                }
                256 => {
                    let v: FixedVector<u64, 256> = data.clone().into();
                    (v.as_ssz_bytes(), 256)
                }
                _ => unreachable!(),
            };

            b.iter(|| match encoded.1 {
                8 => {
                    let view = FixedVectorRef::<u64, 8>::from_ssz_bytes(black_box(&encoded.0))
                        .expect("decode failed");
                    let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
                    black_box(root);
                }
                64 => {
                    let view = FixedVectorRef::<u64, 64>::from_ssz_bytes(black_box(&encoded.0))
                        .expect("decode failed");
                    let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
                    black_box(root);
                }
                256 => {
                    let view = FixedVectorRef::<u64, 256>::from_ssz_bytes(black_box(&encoded.0))
                        .expect("decode failed");
                    let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
                    black_box(root);
                }
                _ => unreachable!(),
            });
        });
    }

    group.finish();
}

/// Benchmark iteration over [`VariableList`].
fn bench_iterate_variable_list_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterate_variable_list_u64");

    for size in [100, 1000, 10000] {
        let data = generate_u64_vec(size);
        let list: VariableList<u64, 16384> = data.into();
        let encoded = list.as_ssz_bytes();

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned iteration
        group.bench_with_input(BenchmarkId::new("owned", size), &list, |b, list| {
            b.iter(|| {
                let sum: u64 = black_box(list).iter().sum();
                black_box(sum);
            });
        });

        // Benchmark view iteration
        group.bench_with_input(BenchmarkId::new("view", size), &encoded, |b, encoded| {
            b.iter(|| {
                let view = VariableListRef::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                let sum: u64 = view.iter().map(|r| r.unwrap()).sum();
                black_box(sum);
            });
        });
    }

    group.finish();
}

/// Benchmark [`Decode`] + [`TreeHash`] combined (common use case).
fn bench_decode_and_hash_variable_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_and_hash_variable_list");

    for size in [100, 1000] {
        let data = generate_u64_vec(size);
        let list: VariableList<u64, 16384> = data.into();
        let encoded = list.as_ssz_bytes();

        group.throughput(Throughput::Elements(size as u64));

        // Benchmark owned: decode + hash
        group.bench_with_input(BenchmarkId::new("owned", size), &encoded, |b, encoded| {
            b.iter(|| {
                let list = VariableList::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&list);
                black_box(root);
            });
        });

        // Benchmark view: decode + hash
        group.bench_with_input(BenchmarkId::new("view", size), &encoded, |b, encoded| {
            b.iter(|| {
                let view = VariableListRef::<u64, 16384>::from_ssz_bytes(black_box(encoded))
                    .expect("decode failed");
                let root: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
                black_box(root);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_decode_variable_list_u64,
    bench_decode_variable_list_bytes,
    bench_decode_fixed_vector_u64,
    bench_tree_hash_variable_list_u64,
    bench_tree_hash_fixed_vector_u64,
    bench_iterate_variable_list_u64,
    bench_decode_and_hash_variable_list,
);

criterion_main!(benches);
