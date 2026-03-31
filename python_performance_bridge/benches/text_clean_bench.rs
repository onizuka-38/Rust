use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use python_performance_bridge::{clean_texts_parallel_for_bench, clean_texts_serial_for_bench};

fn make_dataset(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            format!(
                "Order {} completed for BTCUSDT at price 67123.10. Visit https://exchange.example/trade/{} now!",
                i, i
            )
        })
        .collect()
}

fn text_clean_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_preprocessing");

    for size in [50_000usize, 200_000usize] {
        let data = make_dataset(size);
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("serial", size), &data, |b, d| {
            b.iter(|| {
                let out = clean_texts_serial_for_bench(black_box(d));
                black_box(out.len())
            })
        });

        group.bench_with_input(BenchmarkId::new("parallel", size), &data, |b, d| {
            b.iter(|| {
                let out = clean_texts_parallel_for_bench(black_box(d));
                black_box(out.len())
            })
        });
    }

    group.finish();
}

criterion_group!(benches, text_clean_benchmark);
criterion_main!(benches);
