mod ffi;
mod safe;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use safe::{BridgeError, Context};

fn to_py_err(err: BridgeError) -> PyErr {
    PyValueError::new_err(err.to_string())
}

#[pyfunction]
fn matmul(
    a_rows: usize,
    a_cols: usize,
    a_data: Vec<f64>,
    b_rows: usize,
    b_cols: usize,
    b_data: Vec<f64>,
) -> PyResult<Vec<f64>> {
    let ctx = Context::new().map_err(to_py_err)?;
    let a = ctx
        .matrix_from_slice(a_rows, a_cols, &a_data)
        .map_err(to_py_err)?;
    let b = ctx
        .matrix_from_slice(b_rows, b_cols, &b_data)
        .map_err(to_py_err)?;
    let out = ctx.matmul(&a, &b).map_err(to_py_err)?;
    out.to_vec().map_err(to_py_err)
}

#[pyfunction]
fn affine_sigmoid(
    input_rows: usize,
    input_cols: usize,
    input: Vec<f64>,
    weight_rows: usize,
    weight_cols: usize,
    weights: Vec<f64>,
    bias_rows: usize,
    bias_cols: usize,
    bias: Vec<f64>,
) -> PyResult<Vec<f64>> {
    let ctx = Context::new().map_err(to_py_err)?;

    let input = ctx
        .matrix_from_slice(input_rows, input_cols, &input)
        .map_err(to_py_err)?;
    let weights = ctx
        .matrix_from_slice(weight_rows, weight_cols, &weights)
        .map_err(to_py_err)?;
    let bias = ctx
        .matrix_from_slice(bias_rows, bias_cols, &bias)
        .map_err(to_py_err)?;

    let out = ctx
        .affine_sigmoid(&input, &weights, &bias)
        .map_err(to_py_err)?;
    out.to_vec().map_err(to_py_err)
}

#[pyfunction]
fn rust_only_matmul(
    a_rows: usize,
    a_cols: usize,
    a_data: Vec<f64>,
    b_rows: usize,
    b_cols: usize,
    b_data: Vec<f64>,
) -> PyResult<Vec<f64>> {
    matmul(a_rows, a_cols, a_data, b_rows, b_cols, b_data)
}

#[pyfunction]
fn ping(iterations: usize) -> usize {
    let mut acc = 0usize;
    for i in 0..iterations {
        acc = acc.wrapping_add(i ^ 0x55AA55AA);
    }
    acc
}

#[pymodule]
fn _core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(matmul, m)?)?;
    m.add_function(wrap_pyfunction!(affine_sigmoid, m)?)?;
    m.add_function(wrap_pyfunction!(rust_only_matmul, m)?)?;
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    Ok(())
}

pub fn rust_safe_matmul(
    a_rows: usize,
    a_cols: usize,
    a_data: &[f64],
    b_rows: usize,
    b_cols: usize,
    b_data: &[f64],
) -> Result<Vec<f64>, BridgeError> {
    let ctx = Context::new()?;
    let a = ctx.matrix_from_slice(a_rows, a_cols, a_data)?;
    let b = ctx.matrix_from_slice(b_rows, b_cols, b_data)?;
    let out = ctx.matmul(&a, &b)?;
    out.to_vec()
}
