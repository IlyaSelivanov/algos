use algos::sorting::{insertion_sort, merge_sort};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

const ARR_SIZE: usize = 100_000;

fn generate_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..100)).collect()
}

fn benchmark_insertion_sort(c: &mut Criterion) {
    let mut data = generate_data(ARR_SIZE);
    let mut arr = data.as_mut_slice();

    c.bench_function("insertion_sort", |b| {
        b.iter(|| insertion_sort(black_box(&mut arr)))
    });
}

fn benchmark_merge_sort(c: &mut Criterion) {
    let mut data = generate_data(ARR_SIZE);
    let mut arr = data.as_mut_slice();

    c.bench_function("merge_sort", |b| b.iter(|| merge_sort(black_box(&mut arr))));
}

criterion_group!(benches, benchmark_insertion_sort, benchmark_merge_sort);
criterion_main!(benches);
