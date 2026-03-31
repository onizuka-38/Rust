use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeEvent {
    pub ts: u64,
    pub exchange: String,
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct SymbolStats {
    pub trades: u64,
    pub buy_trades: u64,
    pub sell_trades: u64,
    pub total_qty: f64,
    pub total_notional: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub first_ts: u64,
    pub last_ts: u64,
}

impl SymbolStats {
    pub fn update(&mut self, trade: &TradeEvent) {
        self.trades += 1;
        if trade.side.eq_ignore_ascii_case("buy") {
            self.buy_trades += 1;
        } else {
            self.sell_trades += 1;
        }

        self.total_qty += trade.qty;
        self.total_notional += trade.price * trade.qty;

        if self.trades == 1 {
            self.min_price = trade.price;
            self.max_price = trade.price;
            self.first_ts = trade.ts;
            self.last_ts = trade.ts;
        } else {
            self.min_price = self.min_price.min(trade.price);
            self.max_price = self.max_price.max(trade.price);
            self.first_ts = self.first_ts.min(trade.ts);
            self.last_ts = self.last_ts.max(trade.ts);
        }
    }

    pub fn merge_from(&mut self, other: &SymbolStats) {
        if other.trades == 0 {
            return;
        }

        let old_trades = self.trades;
        self.trades += other.trades;
        self.buy_trades += other.buy_trades;
        self.sell_trades += other.sell_trades;
        self.total_qty += other.total_qty;
        self.total_notional += other.total_notional;

        if old_trades == 0 {
            self.min_price = other.min_price;
            self.max_price = other.max_price;
            self.first_ts = other.first_ts;
            self.last_ts = other.last_ts;
        } else {
            self.min_price = self.min_price.min(other.min_price);
            self.max_price = self.max_price.max(other.max_price);
            self.first_ts = self.first_ts.min(other.first_ts);
            self.last_ts = self.last_ts.max(other.last_ts);
        }
    }

    pub fn vwap(&self) -> f64 {
        if self.total_qty == 0.0 {
            return 0.0;
        }
        self.total_notional / self.total_qty
    }
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct AggregationResult {
    pub stats: HashMap<String, SymbolStats>,
    pub processed_lines: u64,
    pub invalid_lines: u64,
}

fn parse_trade_line(line: &str) -> Option<TradeEvent> {
    serde_json::from_str::<TradeEvent>(line).ok()
}

pub fn aggregate_lines_serial(lines: &[String]) -> AggregationResult {
    let mut out = AggregationResult::default();

    for line in lines {
        out.processed_lines += 1;
        if let Some(trade) = parse_trade_line(line) {
            let entry = out.stats.entry(trade.symbol.clone()).or_default();
            entry.update(&trade);
        } else {
            out.invalid_lines += 1;
        }
    }

    out
}

pub fn aggregate_lines_parallel(lines: &[String]) -> AggregationResult {
    let partials: Vec<AggregationResult> = lines
        .par_chunks(4_096)
        .map(aggregate_lines_serial)
        .collect();

    merge_partial_results(partials)
}

pub fn merge_partial_results(partials: Vec<AggregationResult>) -> AggregationResult {
    let mut merged = AggregationResult::default();

    for part in partials {
        merged.processed_lines += part.processed_lines;
        merged.invalid_lines += part.invalid_lines;
        for (symbol, symbol_stats) in part.stats {
            merged
                .stats
                .entry(symbol)
                .or_default()
                .merge_from(&symbol_stats);
        }
    }

    merged
}

pub fn generate_trade_line(index: usize, symbol_count: usize) -> String {
    let symbols = [
        "BTCUSDT",
        "ETHUSDT",
        "SOLUSDT",
        "XRPUSDT",
        "DOGEUSDT",
        "BNBUSDT",
        "ADAUSDT",
        "AVAXUSDT",
    ];
    let symbol = symbols[index % symbol_count.min(symbols.len()).max(1)];
    let side = if index % 2 == 0 { "buy" } else { "sell" };
    let price = 100.0 + ((index % 10_000) as f64) * 0.01;
    let qty = 0.001 + ((index % 100) as f64) * 0.0001;
    let ts = 1_700_000_000_000u64 + index as u64;

    format!(
        "{{\"ts\":{ts},\"exchange\":\"binance\",\"symbol\":\"{symbol}\",\"side\":\"{side}\",\"price\":{price:.4},\"qty\":{qty:.6}}}"
    )
}
