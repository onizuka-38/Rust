use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static URL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https?://\S+|www\.\S+").expect("valid url regex"));
static NON_WORD_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^a-z\s]+").expect("valid non-word regex"));
static MULTI_SPACE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+").expect("valid space regex"));

pub fn clean_and_tokenize(text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    let no_url = URL_RE.replace_all(&lower, " ");
    let alpha_only = NON_WORD_RE.replace_all(&no_url, " ");
    let normalized = MULTI_SPACE_RE.replace_all(&alpha_only, " ");

    normalized
        .split_whitespace()
        .filter(|tok| tok.len() >= 2)
        .map(str::to_string)
        .collect()
}

pub fn token_counts(tokens_per_row: &[Vec<String>]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for row in tokens_per_row {
        for tok in row {
            *counts.entry(tok.clone()).or_insert(0) += 1;
        }
    }
    counts
}
