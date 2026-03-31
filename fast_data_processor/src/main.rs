use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use fast_data_processor::{
    aggregate_lines_parallel, aggregate_lines_serial, generate_trade_line, AggregationResult, SymbolStats,
};
use regex::RegexBuilder;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "fdp")]
#[command(about = "Fast Data Processor for logs and crypto NDJSON")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(long)]
        include: Option<String>,
        #[arg(long)]
        exclude: Option<String>,
        #[arg(long, default_value_t = false)]
        ignore_case: bool,
        #[arg(long, default_value_t = false)]
        count_only: bool,
        #[arg(long)]
        limit: Option<usize>,
    },
    CryptoStats {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, value_enum, default_value_t = Mode::Parallel)]
        mode: Mode,
        #[arg(long, default_value_t = 50_000)]
        batch_size: usize,
        #[arg(long, default_value_t = 10)]
        top: usize,
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    BenchInput {
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 1_000_000)]
        lines: usize,
        #[arg(long, default_value_t = 4)]
        symbols: usize,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Mode {
    Serial,
    Parallel,
}

#[derive(Serialize)]
struct SymbolSummary {
    symbol: String,
    trades: u64,
    buy_trades: u64,
    sell_trades: u64,
    total_qty: f64,
    total_notional: f64,
    vwap: f64,
    min_price: f64,
    max_price: f64,
    first_ts: u64,
    last_ts: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            files,
            include,
            exclude,
            ignore_case,
            count_only,
            limit,
        } => run_scan(files, include, exclude, ignore_case, count_only, limit),
        Commands::CryptoStats {
            input,
            mode,
            batch_size,
            top,
            json,
        } => run_crypto_stats(input, mode, batch_size, top, json),
        Commands::BenchInput {
            output,
            lines,
            symbols,
        } => run_bench_input(output, lines, symbols),
    }
}

fn run_scan(
    files: Vec<PathBuf>,
    include: Option<String>,
    exclude: Option<String>,
    ignore_case: bool,
    count_only: bool,
    limit: Option<usize>,
) -> Result<()> {
    let include_regex = include
        .map(|p| RegexBuilder::new(&p).case_insensitive(ignore_case).build())
        .transpose()
        .context("Invalid include regex")?;

    let exclude_regex = exclude
        .map(|p| RegexBuilder::new(&p).case_insensitive(ignore_case).build())
        .transpose()
        .context("Invalid exclude regex")?;

    let mut total = 0usize;
    let mut matched = 0usize;
    let mut printed = 0usize;

    for path in files {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        let mut line = String::new();
        let mut line_no = 0usize;

        loop {
            line.clear();
            let bytes = reader
                .read_line(&mut line)
                .with_context(|| format!("Failed while reading file: {}", path.display()))?;
            if bytes == 0 {
                break;
            }
            line_no += 1;
            total += 1;

            let include_ok = include_regex
                .as_ref()
                .map(|r| r.is_match(&line))
                .unwrap_or(true);
            let exclude_ok = exclude_regex
                .as_ref()
                .map(|r| !r.is_match(&line))
                .unwrap_or(true);

            if include_ok && exclude_ok {
                matched += 1;
                if !count_only {
                    let cleaned = line.trim_end_matches(['\r', '\n']);
                    println!("{}:{}:{}", path.display(), line_no, cleaned);
                    printed += 1;
                    if let Some(max_print) = limit {
                        if printed >= max_print {
                            break;
                        }
                    }
                }
            }
        }

        if let Some(max_print) = limit {
            if printed >= max_print {
                break;
            }
        }
    }

    println!("scanned_lines={total} matched_lines={matched}");
    Ok(())
}

fn run_crypto_stats(
    input: PathBuf,
    mode: Mode,
    batch_size: usize,
    top: usize,
    json: bool,
) -> Result<()> {
    if batch_size == 0 {
        anyhow::bail!("batch_size must be > 0");
    }

    let started = Instant::now();
    let file = File::open(&input)
        .with_context(|| format!("Failed to open input file: {}", input.display()))?;
    let mut reader = BufReader::new(file);

    let mut acc = AggregationResult::default();
    let mut batch: Vec<String> = Vec::with_capacity(batch_size);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader
            .read_line(&mut line)
            .with_context(|| format!("Failed while reading input file: {}", input.display()))?;
        if bytes == 0 {
            if !batch.is_empty() {
                process_batch(&mut acc, &batch, mode);
            }
            break;
        }

        let cleaned = line.trim_end_matches(['\r', '\n']).to_string();
        batch.push(cleaned);

        if batch.len() >= batch_size {
            process_batch(&mut acc, &batch, mode);
            batch.clear();
        }
    }

    let elapsed = started.elapsed();
    output_summary(&acc.stats, top, json)?;

    println!("\nprocessed_lines={} invalid_lines={}", acc.processed_lines, acc.invalid_lines);
    println!("mode={mode:?} elapsed_ms={}", elapsed.as_millis());

    Ok(())
}

fn process_batch(acc: &mut AggregationResult, batch: &[String], mode: Mode) {
    let part = match mode {
        Mode::Serial => aggregate_lines_serial(batch),
        Mode::Parallel => aggregate_lines_parallel(batch),
    };
    merge_into(acc, part);
}

fn merge_into(target: &mut AggregationResult, part: AggregationResult) {
    target.processed_lines += part.processed_lines;
    target.invalid_lines += part.invalid_lines;

    for (symbol, st) in part.stats {
        target.stats.entry(symbol).or_default().merge_from(&st);
    }
}

fn output_summary(stats: &HashMap<String, SymbolStats>, top: usize, json: bool) -> Result<()> {
    let mut rows: Vec<SymbolSummary> = stats
        .iter()
        .map(|(symbol, st)| SymbolSummary {
            symbol: symbol.clone(),
            trades: st.trades,
            buy_trades: st.buy_trades,
            sell_trades: st.sell_trades,
            total_qty: st.total_qty,
            total_notional: st.total_notional,
            vwap: st.vwap(),
            min_price: st.min_price,
            max_price: st.max_price,
            first_ts: st.first_ts,
            last_ts: st.last_ts,
        })
        .collect();

    rows.sort_by(|a, b| {
        b.total_notional
            .partial_cmp(&a.total_notional)
            .unwrap_or(Ordering::Equal)
    });

    let rows = rows.into_iter().take(top).collect::<Vec<_>>();

    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
    } else {
        println!("symbol   | trades | buy   | sell  | qty        | notional      | vwap      | min      | max");
        println!("---------+--------+-------+-------+------------+---------------+-----------+----------+----------");
        for row in rows {
            println!(
                "{:<8} | {:>6} | {:>5} | {:>5} | {:>10.4} | {:>13.4} | {:>9.4} | {:>8.4} | {:>8.4}",
                row.symbol,
                row.trades,
                row.buy_trades,
                row.sell_trades,
                row.total_qty,
                row.total_notional,
                row.vwap,
                row.min_price,
                row.max_price
            );
        }
    }

    Ok(())
}

fn run_bench_input(output: PathBuf, lines: usize, symbols: usize) -> Result<()> {
    let started = Instant::now();
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }
    let file = File::create(&output)
        .with_context(|| format!("Failed to create output file: {}", output.display()))?;
    let mut writer = BufWriter::new(file);

    for i in 0..lines {
        let line = generate_trade_line(i, symbols);
        writer
            .write_all(line.as_bytes())
            .with_context(|| format!("Failed to write line {}", i))?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;

    println!(
        "generated_lines={} output={} elapsed_ms={}",
        lines,
        output.display(),
        started.elapsed().as_millis()
    );

    Ok(())
}

