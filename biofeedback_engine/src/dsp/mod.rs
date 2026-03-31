use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

#[derive(Debug, Clone)]
pub struct ChannelChunk {
    pub channel_index: usize,
    pub sample_rate_hz: f32,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct FftBandPower {
    pub delta: f32,
    pub theta: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
}

#[derive(Debug, Clone)]
pub struct ChannelAnalysis {
    pub channel_index: usize,
    pub latency_ms: u128,
    pub rms: f32,
    pub bands: FftBandPower,
}

pub fn preprocess_artifact(samples: &mut [f32], clip_abs: f32) {
    if samples.is_empty() {
        return;
    }

    for s in samples.iter_mut() {
        if *s > clip_abs {
            *s = clip_abs;
        } else if *s < -clip_abs {
            *s = -clip_abs;
        }
    }

    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    for s in samples.iter_mut() {
        *s -= mean;
    }
}

pub fn compute_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let e = samples.iter().map(|v| v * v).sum::<f32>() / samples.len() as f32;
    e.sqrt()
}

pub fn compute_fft_bands(samples: &[f32], sample_rate_hz: f32) -> FftBandPower {
    let n = samples.len().max(1).next_power_of_two();
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);

    let mut buffer = vec![Complex::<f32>::new(0.0, 0.0); n];
    for (dst, src) in buffer.iter_mut().zip(samples.iter()) {
        dst.re = *src;
    }

    fft.process(&mut buffer);

    let hz_per_bin = sample_rate_hz / n as f32;

    let mut out = FftBandPower {
        delta: 0.0,
        theta: 0.0,
        alpha: 0.0,
        beta: 0.0,
        gamma: 0.0,
    };

    for (idx, c) in buffer.iter().take(n / 2).enumerate() {
        let freq = idx as f32 * hz_per_bin;
        let p = c.norm_sqr();
        match freq {
            f if (0.5..4.0).contains(&f) => out.delta += p,
            f if (4.0..8.0).contains(&f) => out.theta += p,
            f if (8.0..13.0).contains(&f) => out.alpha += p,
            f if (13.0..30.0).contains(&f) => out.beta += p,
            f if (30.0..80.0).contains(&f) => out.gamma += p,
            _ => {}
        }
    }

    out
}
