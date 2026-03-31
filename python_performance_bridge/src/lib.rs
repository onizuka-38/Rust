mod processing;

use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

use processing::{clean_and_tokenize, token_counts};

fn clean_texts_serial_inner(texts: &[String]) -> Vec<Vec<String>> {
    texts.iter().map(|t| clean_and_tokenize(t)).collect()
}

fn clean_texts_parallel_inner(texts: &[String]) -> Vec<Vec<String>> {
    texts.par_iter().map(|t| clean_and_tokenize(t)).collect()
}

#[pyfunction]
fn clean_text(text: &str) -> Vec<String> {
    clean_and_tokenize(text)
}

#[pyfunction]
fn clean_texts(texts: Vec<String>, parallel: Option<bool>) -> Vec<Vec<String>> {
    if parallel.unwrap_or(true) {
        clean_texts_parallel_inner(&texts)
    } else {
        clean_texts_serial_inner(&texts)
    }
}

#[pyfunction]
fn token_frequency(texts: Vec<String>, parallel: Option<bool>) -> HashMap<String, usize> {
    let rows = if parallel.unwrap_or(true) {
        clean_texts_parallel_inner(&texts)
    } else {
        clean_texts_serial_inner(&texts)
    };
    token_counts(&rows)
}

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn _core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clean_text, m)?)?;
    m.add_function(wrap_pyfunction!(clean_texts, m)?)?;
    m.add_function(wrap_pyfunction!(token_frequency, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

pub fn clean_texts_serial_for_bench(texts: &[String]) -> Vec<Vec<String>> {
    clean_texts_serial_inner(texts)
}

pub fn clean_texts_parallel_for_bench(texts: &[String]) -> Vec<Vec<String>> {
    clean_texts_parallel_inner(texts)
}
