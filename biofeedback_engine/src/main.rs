mod dsp;
mod edf;
mod pipeline;

use anyhow::Result;
use clap::{Parser, Subcommand};
use edf::parser::parse_edf_from_path;
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "biofeedback-engine")]
#[command(about = "High-speed EEG/EMG parsing and frequency analysis core engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ParseEdf {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        max_records: Option<usize>,
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    RealtimeSim {
        #[arg(long, default_value_t = 8)]
        channels: usize,
        #[arg(long, default_value_t = 1024)]
        sample_rate: usize,
        #[arg(long, default_value_t = 256)]
        chunk_size: usize,
        #[arg(long, default_value_t = 5)]
        seconds: usize,
        #[arg(long, default_value_t = 500.0)]
        artifact_clip: f32,
    },
}

#[derive(Debug, Serialize)]
struct ParseReport {
    parse_latency_ms: u128,
    signals: usize,
    records: i64,
    duration_sec: f32,
    total_samples: usize,
    labels: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ParseEdf {
            input,
            max_records,
            json,
        } => parse_edf_cmd(input, max_records, json),
        Commands::RealtimeSim {
            channels,
            sample_rate,
            chunk_size,
            seconds,
            artifact_clip,
        } => realtime_sim_cmd(channels, sample_rate, chunk_size, seconds, artifact_clip).await,
    }
}

fn parse_edf_cmd(input: PathBuf, max_records: Option<usize>, json: bool) -> Result<()> {
    let start = Instant::now();
    let data = parse_edf_from_path(&input, max_records)?;
    let elapsed = start.elapsed().as_millis();

    let total_samples = data.channels.iter().map(|c| c.len()).sum::<usize>();
    let report = ParseReport {
        parse_latency_ms: elapsed,
        signals: data.header.num_signals,
        records: data.header.num_data_records,
        duration_sec: data.header.data_record_duration_sec,
        total_samples,
        labels: data.header.labels.clone(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("EDF parsed in {} ms", report.parse_latency_ms);
        println!("signals={} records={} duration={}s", report.signals, report.records, report.duration_sec);
        println!("total_samples={}", report.total_samples);
        println!("labels={:?}", report.labels);
    }

    Ok(())
}

async fn realtime_sim_cmd(
    channels: usize,
    sample_rate: usize,
    chunk_size: usize,
    seconds: usize,
    artifact_clip: f32,
) -> Result<()> {
    let (tx_chunk, rx_chunk) = mpsc::channel::<dsp::ChannelChunk>(4096);
    let (tx_result, mut rx_result) = mpsc::channel::<dsp::ChannelAnalysis>(4096);

    let worker = tokio::spawn(pipeline::run_analysis_worker(rx_chunk, tx_result, artifact_clip));
    let feeder = tokio::spawn(pipeline::feed_simulated_stream(
        tx_chunk,
        channels,
        sample_rate,
        chunk_size,
        seconds,
    ));

    let expected_chunks = ((sample_rate / chunk_size.max(1)).max(1) * seconds) * channels;

    let started = Instant::now();
    feeder.await?;

    let mut processed = 0usize;
    let mut total_latency_ms = 0u128;
    while let Some(result) = rx_result.recv().await {
        processed += 1;
        total_latency_ms += result.latency_ms;
        if processed >= expected_chunks {
            break;
        }
    }

    worker.abort();

    let elapsed_ms = started.elapsed().as_millis();
    let avg_stage_latency = if processed == 0 {
        0.0
    } else {
        total_latency_ms as f64 / processed as f64
    };

    println!("Realtime pipeline complete");
    println!("processed_chunks={}", processed);
    println!("end_to_end_elapsed_ms={}", elapsed_ms);
    println!("avg_fft_stage_latency_ms={:.3}", avg_stage_latency);

    Ok(())
}
