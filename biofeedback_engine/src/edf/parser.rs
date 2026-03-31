use super::{EdfData, EdfHeader};
use anyhow::{Context, Result};
use nom::bytes::complete::take;
use nom::IResult;

fn parse_ascii_field<'a>(input: &'a [u8], size: usize) -> IResult<&'a [u8], String> {
    let (rest, bytes) = take(size)(input)?;
    let text = String::from_utf8_lossy(bytes).trim().to_string();
    Ok((rest, text))
}

fn parse_usize_ascii(input: &[u8], size: usize) -> IResult<&[u8], usize> {
    let (rest, txt) = parse_ascii_field(input, size)?;
    let val = txt.parse::<usize>().unwrap_or(0);
    Ok((rest, val))
}

fn parse_i64_ascii(input: &[u8], size: usize) -> IResult<&[u8], i64> {
    let (rest, txt) = parse_ascii_field(input, size)?;
    let val = txt.parse::<i64>().unwrap_or(-1);
    Ok((rest, val))
}

fn parse_f32_ascii(input: &[u8], size: usize) -> IResult<&[u8], f32> {
    let (rest, txt) = parse_ascii_field(input, size)?;
    let val = txt.parse::<f32>().unwrap_or(0.0);
    Ok((rest, val))
}

pub fn parse_edf_header(input: &[u8]) -> Result<(&[u8], EdfHeader)> {
    let (i, version) = parse_ascii_field(input, 8).map_err(|_| anyhow::anyhow!("version parse"))?;
    let (i, patient_id) = parse_ascii_field(i, 80).map_err(|_| anyhow::anyhow!("patient parse"))?;
    let (i, recording_id) = parse_ascii_field(i, 80).map_err(|_| anyhow::anyhow!("recording parse"))?;
    let (i, start_date) = parse_ascii_field(i, 8).map_err(|_| anyhow::anyhow!("date parse"))?;
    let (i, start_time) = parse_ascii_field(i, 8).map_err(|_| anyhow::anyhow!("time parse"))?;
    let (i, header_bytes) = parse_usize_ascii(i, 8).map_err(|_| anyhow::anyhow!("header bytes parse"))?;

    let (i, _reserved) = take(44usize)(i).map_err(|_| anyhow::anyhow!("reserved parse"))?;
    let (i, num_data_records) = parse_i64_ascii(i, 8).map_err(|_| anyhow::anyhow!("records parse"))?;
    let (i, data_record_duration_sec) =
        parse_f32_ascii(i, 8).map_err(|_| anyhow::anyhow!("record duration parse"))?;
    let (mut i, num_signals) = parse_usize_ascii(i, 4).map_err(|_| anyhow::anyhow!("signals parse"))?;

    let mut labels = Vec::with_capacity(num_signals);
    for _ in 0..num_signals {
        let (rest, label) = parse_ascii_field(i, 16).map_err(|_| anyhow::anyhow!("label parse"))?;
        labels.push(label);
        i = rest;
    }

    let skip_fields = [80usize, 8, 8, 8, 8, 80];
    for field_size in skip_fields {
        let bytes = field_size * num_signals;
        let (rest, _) = take(bytes)(i).map_err(|_| anyhow::anyhow!("skip parse"))?;
        i = rest;
    }

    let mut samples_per_record = Vec::with_capacity(num_signals);
    for _ in 0..num_signals {
        let (rest, samples) = parse_usize_ascii(i, 8).map_err(|_| anyhow::anyhow!("samples parse"))?;
        samples_per_record.push(samples);
        i = rest;
    }

    let (i, _) = take(32usize * num_signals)(i).map_err(|_| anyhow::anyhow!("reserved2 parse"))?;

    let header = EdfHeader {
        version,
        patient_id,
        recording_id,
        start_date,
        start_time,
        header_bytes,
        num_data_records,
        data_record_duration_sec,
        num_signals,
        labels,
        samples_per_record,
    };

    Ok((i, header))
}

pub fn parse_edf_file(bytes: &[u8], max_records: Option<usize>) -> Result<EdfData> {
    let (_remaining_after_fixed, header) = parse_edf_header(bytes)?;

    if bytes.len() < header.header_bytes {
        anyhow::bail!("EDF file shorter than declared header size");
    }

    let mut data_ptr = &bytes[header.header_bytes..];
    let total_records = if header.num_data_records < 0 {
        0usize
    } else {
        header.num_data_records as usize
    };
    let records_to_parse = max_records.unwrap_or(total_records).min(total_records);

    let mut channels = header
        .samples_per_record
        .iter()
        .map(|s| Vec::with_capacity(s.saturating_mul(records_to_parse)))
        .collect::<Vec<_>>();

    for _ in 0..records_to_parse {
        for (ch_idx, samples) in header.samples_per_record.iter().enumerate() {
            for _ in 0..*samples {
                if data_ptr.len() < 2 {
                    anyhow::bail!("unexpected EOF while reading EDF signal samples");
                }
                let raw = i16::from_le_bytes([data_ptr[0], data_ptr[1]]);
                data_ptr = &data_ptr[2..];
                channels[ch_idx].push(raw as f32);
            }
        }
    }

    Ok(EdfData { header, channels })
}

pub fn parse_edf_from_path(path: &std::path::Path, max_records: Option<usize>) -> Result<EdfData> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read EDF file: {}", path.display()))?;
    parse_edf_file(&bytes, max_records)
}
