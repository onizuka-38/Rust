use crate::ffi;
use std::ptr::NonNull;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("allocation failed in legacy C library")]
    Allocation,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("shape mismatch: {0}")]
    ShapeMismatch(String),
    #[error("ffi error: {0}")]
    Ffi(String),
}

type Result<T> = std::result::Result<T, BridgeError>;

struct ContextInner {
    raw: NonNull<ffi::lm_context>,
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        unsafe {
            ffi::lm_context_free(self.raw.as_ptr());
        }
    }
}

#[derive(Clone)]
pub struct Context {
    inner: Arc<ContextInner>,
}

pub struct Matrix {
    raw: NonNull<ffi::lm_matrix>,
    rows: usize,
    cols: usize,
    ctx: Arc<ContextInner>,
}

impl Context {
    pub fn new() -> Result<Self> {
        let raw = unsafe { ffi::lm_context_new() };
        let raw = NonNull::new(raw).ok_or(BridgeError::Allocation)?;
        Ok(Self {
            inner: Arc::new(ContextInner { raw }),
        })
    }

    pub fn matrix_from_slice(&self, rows: usize, cols: usize, data: &[f64]) -> Result<Matrix> {
        if rows == 0 || cols == 0 {
            return Err(BridgeError::InvalidArgument("rows/cols must be > 0".to_string()));
        }
        if data.len() != rows * cols {
            return Err(BridgeError::ShapeMismatch(format!(
                "data length {} does not match shape {}x{}",
                data.len(), rows, cols
            )));
        }

        let ptr = unsafe { ffi::lm_matrix_new(self.inner.raw.as_ptr(), rows, cols) };
        let raw = NonNull::new(ptr).ok_or_else(|| self.last_error_or_alloc())?;

        let code = unsafe {
            ffi::lm_matrix_fill(
                self.inner.raw.as_ptr(),
                raw.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        };
        check_code(self, code)?;

        Ok(Matrix {
            raw,
            rows,
            cols,
            ctx: Arc::clone(&self.inner),
        })
    }

    pub fn matmul(&self, a: &Matrix, b: &Matrix) -> Result<Matrix> {
        if a.cols != b.rows {
            return Err(BridgeError::ShapeMismatch(format!(
                "cannot multiply {}x{} by {}x{}",
                a.rows, a.cols, b.rows, b.cols
            )));
        }

        let out_ptr = unsafe { ffi::lm_matrix_new(self.inner.raw.as_ptr(), a.rows, b.cols) };
        let out_raw = NonNull::new(out_ptr).ok_or_else(|| self.last_error_or_alloc())?;

        let code = unsafe {
            ffi::lm_matrix_mul(
                self.inner.raw.as_ptr(),
                a.raw.as_ptr(),
                b.raw.as_ptr(),
                out_raw.as_ptr(),
            )
        };
        if let Err(e) = check_code(self, code) {
            unsafe {
                ffi::lm_matrix_free(self.inner.raw.as_ptr(), out_raw.as_ptr());
            }
            return Err(e);
        }

        Ok(Matrix {
            raw: out_raw,
            rows: a.rows,
            cols: b.cols,
            ctx: Arc::clone(&self.inner),
        })
    }

    pub fn affine_sigmoid(&self, input: &Matrix, weights: &Matrix, bias: &Matrix) -> Result<Matrix> {
        let mut out = self.matmul(input, weights)?;
        out.add_inplace(bias)?;
        out.sigmoid_inplace()?;
        Ok(out)
    }

    fn last_error_or_alloc(&self) -> BridgeError {
        let msg = self.last_error();
        if msg.is_empty() {
            BridgeError::Allocation
        } else {
            BridgeError::Ffi(msg)
        }
    }

    fn last_error(&self) -> String {
        unsafe {
            let ptr = ffi::lm_last_error(self.inner.raw.as_ptr());
            if ptr.is_null() {
                return String::new();
            }
            std::ffi::CStr::from_ptr(ptr)
                .to_string_lossy()
                .trim()
                .to_string()
        }
    }
}

impl Matrix {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn add_inplace(&mut self, other: &Matrix) -> Result<()> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(BridgeError::ShapeMismatch(format!(
                "cannot add {}x{} and {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }

        let code = unsafe {
            ffi::lm_matrix_add_inplace(self.ctx.raw.as_ptr(), self.raw.as_ptr(), other.raw.as_ptr())
        };
        check_code_by_inner(&self.ctx, code)
    }

    pub fn sigmoid_inplace(&mut self) -> Result<()> {
        let code = unsafe { ffi::lm_matrix_sigmoid_inplace(self.ctx.raw.as_ptr(), self.raw.as_ptr()) };
        check_code_by_inner(&self.ctx, code)
    }

    pub fn to_vec(&self) -> Result<Vec<f64>> {
        let mut ptr: *const f64 = std::ptr::null();
        let mut len: usize = 0;
        let code = unsafe {
            ffi::lm_matrix_data(
                self.ctx.raw.as_ptr(),
                self.raw.as_ptr(),
                &mut ptr,
                &mut len,
            )
        };
        check_code_by_inner(&self.ctx, code)?;
        if ptr.is_null() {
            return Err(BridgeError::Ffi("lm_matrix_data returned null pointer".to_string()));
        }

        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        Ok(slice.to_vec())
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            ffi::lm_matrix_free(self.ctx.raw.as_ptr(), self.raw.as_ptr());
        }
    }
}

fn check_code(ctx: &Context, code: i32) -> Result<()> {
    check_code_by_inner(&ctx.inner, code)
}

fn check_code_by_inner(ctx: &Arc<ContextInner>, code: i32) -> Result<()> {
    if code == ffi::LM_OK as i32 {
        return Ok(());
    }

    let msg = unsafe {
        let ptr = ffi::lm_last_error(ctx.raw.as_ptr());
        if ptr.is_null() {
            String::new()
        } else {
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    };

    match code {
        x if x == ffi::LM_ERR_INVALID_ARG as i32 => Err(BridgeError::InvalidArgument(msg)),
        x if x == ffi::LM_ERR_SHAPE_MISMATCH as i32 => Err(BridgeError::ShapeMismatch(msg)),
        x if x == ffi::LM_ERR_ALLOC as i32 => Err(BridgeError::Allocation),
        _ => Err(BridgeError::Ffi(msg)),
    }
}

