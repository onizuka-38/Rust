pub mod parser;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EdfHeader {
    pub version: String,
    pub patient_id: String,
    pub recording_id: String,
    pub start_date: String,
    pub start_time: String,
    pub header_bytes: usize,
    pub num_data_records: i64,
    pub data_record_duration_sec: f32,
    pub num_signals: usize,
    pub labels: Vec<String>,
    pub samples_per_record: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct EdfData {
    pub header: EdfHeader,
    pub channels: Vec<Vec<f32>>,
}
