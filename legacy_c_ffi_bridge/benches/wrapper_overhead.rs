use criterion::{black_box, criterion_group, criterion_main, Criterion};
use legacy_c_ffi_bridge::rust_safe_matmul;

fn wrapper_overhead(c: &mut Criterion) {
    let a_rows = 64usize;
    let a_cols = 64usize;
    let b_rows = 64usize;
    let b_cols = 64usize;

    let a = (0..(a_rows * a_cols))
        .map(|i| (i % 17) as f64 * 0.1)
        .collect::<Vec<_>>();
    let b = (0..(b_rows * b_cols))
        .map(|i| (i % 13) as f64 * 0.2)
        .collect::<Vec<_>>();

    c.bench_function("safe_wrapper_matmul_64", |bench| {
        bench.iter(|| {
            let out = rust_safe_matmul(
                black_box(a_rows),
                black_box(a_cols),
                black_box(&a),
                black_box(b_rows),
                black_box(b_cols),
                black_box(&b),
            )
            .expect("matmul should succeed");
            black_box(out[0]);
        })
    });
}

criterion_group!(benches, wrapper_overhead);
criterion_main!(benches);
