use crate::dsp::{compute_fft_bands, compute_rms, preprocess_artifact, ChannelAnalysis, ChannelChunk};
use std::time::Instant;
use tokio::sync::mpsc;

pub async fn run_analysis_worker(
    mut rx: mpsc::Receiver<ChannelChunk>,
    tx: mpsc::Sender<ChannelAnalysis>,
    artifact_clip: f32,
) {
    while let Some(mut chunk) = rx.recv().await {
        let started = Instant::now();
        preprocess_artifact(&mut chunk.samples, artifact_clip);
        let rms = compute_rms(&chunk.samples);
        let bands = compute_fft_bands(&chunk.samples, chunk.sample_rate_hz);

        let analysis = ChannelAnalysis {
            channel_index: chunk.channel_index,
            latency_ms: started.elapsed().as_millis(),
            rms,
            bands,
        };

        if tx.send(analysis).await.is_err() {
            break;
        }
    }
}

pub async fn feed_simulated_stream(
    tx: mpsc::Sender<ChannelChunk>,
    channels: usize,
    sample_rate_hz: usize,
    chunk_size: usize,
    seconds: usize,
) {
    let chunks_per_sec = (sample_rate_hz / chunk_size.max(1)).max(1);
    let total_chunks = chunks_per_sec * seconds;

    for tick in 0..total_chunks {
        for ch in 0..channels {
            let base_freq = 6.0 + (ch as f32 * 2.5);
            let mut samples = Vec::with_capacity(chunk_size);
            for i in 0..chunk_size {
                let t = ((tick * chunk_size + i) as f32) / sample_rate_hz as f32;
                let signal = (2.0 * std::f32::consts::PI * base_freq * t).sin() * 80.0;
                let noise = (((i * 17 + ch * 13 + tick * 7) % 100) as f32 / 100.0 - 0.5) * 12.0;
                samples.push(signal + noise);
            }

            let packet = ChannelChunk {
                channel_index: ch,
                sample_rate_hz: sample_rate_hz as f32,
                samples,
            };

            if tx.send(packet).await.is_err() {
                return;
            }
        }
    }
}
