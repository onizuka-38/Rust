use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use fast_data_processor::{aggregate_lines_parallel, aggregate_lines_serial, generate_trade_line};

fn make_lines(n: usize, symbols: usize) -> Vec<String> {
    (0..n).map(|i| generate_trade_line(i, symbols)).collect()
}

fn processing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ndjson_processing");

    for size in [100_000usize, 500_000usize] {
        let lines = make_lines(size, 4);
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("serial", size), &lines, |b, lines| {
            b.iter(|| {
                let result = aggregate_lines_serial(black_box(lines));
                black_box(result.stats.len())
            })
        });

        group.bench_with_input(BenchmarkId::new("parallel", size), &lines, |b, lines| {
            b.iter(|| {
                let result = aggregate_lines_parallel(black_box(lines));
                black_box(result.stats.len())
            })
        });
    }

    group.finish();
}

criterion_group!(benches, processing_benchmark);
criterion_main!(benches);
