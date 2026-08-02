use broken_app::{algo, sum_even};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const SUM_EVEN_SIZES: [usize; 2] = [50_000, 1_000_000];
const FIB_INPUTS: [u64; 2] = [28, 34];
const DEDUP_SIZES: [usize; 2] = [2_000, 20_000];

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even");
    for &size in &SUM_EVEN_SIZES {
        let data: Vec<i64> = (0..size as i64).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| sum_even(black_box(data)));
        });
    }
    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_fib");
    for &n in &FIB_INPUTS {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| algo::fast_fib(black_box(n)));
        });
    }
    group.finish();
}

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_dedup");
    for &size in &DEDUP_SIZES {
        let data: Vec<u64> = (0..size as u64).flat_map(|n| [n, n]).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| algo::fast_dedup(black_box(data)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sum_even, bench_fib, bench_dedup);
criterion_main!(benches);
