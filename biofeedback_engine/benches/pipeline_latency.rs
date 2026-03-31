use biofeedback_engine::dsp::{compute_fft_bands, preprocess_artifact};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn pipeline_latency(c: &mut Criterion) {
    let sample_rate = 1024.0f32;
    let mut samples = (0..2048)
        .map(|i| {
            let t = i as f32 / sample_rate;
            (2.0 * std::f32::consts::PI * 10.0 * t).sin() * 70.0
        })
        .collect::<Vec<_>>();

    c.bench_function("artifact_preprocess_plus_fft", |b| {
        b.iter(|| {
            let mut s = samples.clone();
            preprocess_artifact(black_box(&mut s), 500.0);
            let bands = compute_fft_bands(black_box(&s), black_box(sample_rate));
            black_box(bands.alpha)
        })
    });
}

criterion_group!(benches, pipeline_latency);
criterion_main!(benches);
