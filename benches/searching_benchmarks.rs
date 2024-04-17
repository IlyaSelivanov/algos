use algos::searching::search_subarray::find_maximum_subarray;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

const ARR_SIZE: usize = 100_000;

/// Generates a vector of random integers within the specified size range.
///
/// # Arguments
///
/// * `size` - The size of the vector to generate.
///
/// # Returns
///
/// A vector of random integers.
fn generate_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-10..10)).collect()
}

/// Benchmarks the `find_maximum_subarray` function using the Criterion library.
///
/// # Arguments
///
/// * `c` - A mutable reference to the Criterion struct.
fn benchmark_find_max_subarray(c: &mut Criterion) {
    let arr = generate_data(ARR_SIZE);
    c.bench_function("find_max_subarray", |b| {
        b.iter(|| find_maximum_subarray(&arr, 0, arr.len() - 1))
    });
}

criterion_group!(benches, benchmark_find_max_subarray);
criterion_main!(benches);
