#include "legacy_math.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct lm_context {
    char last_error[256];
};

struct lm_matrix {
    size_t rows;
    size_t cols;
    double* data;
};

static void lm_set_error(lm_context* ctx, const char* msg) {
    if (!ctx) return;
    if (!msg) {
        ctx->last_error[0] = '\0';
        return;
    }
    snprintf(ctx->last_error, sizeof(ctx->last_error), "%s", msg);
}

lm_context* lm_context_new(void) {
    lm_context* ctx = (lm_context*)calloc(1, sizeof(lm_context));
    if (!ctx) return NULL;
    lm_set_error(ctx, "");
    return ctx;
}

void lm_context_free(lm_context* ctx) {
    free(ctx);
}

const char* lm_last_error(const lm_context* ctx) {
    if (!ctx) return "context is null";
    return ctx->last_error;
}

lm_matrix* lm_matrix_new(lm_context* ctx, size_t rows, size_t cols) {
    if (!ctx || rows == 0 || cols == 0) {
        lm_set_error(ctx, "invalid matrix shape");
        return NULL;
    }

    lm_matrix* m = (lm_matrix*)calloc(1, sizeof(lm_matrix));
    if (!m) {
        lm_set_error(ctx, "failed to allocate matrix struct");
        return NULL;
    }

    size_t len = rows * cols;
    m->data = (double*)calloc(len, sizeof(double));
    if (!m->data) {
        free(m);
        lm_set_error(ctx, "failed to allocate matrix data");
        return NULL;
    }

    m->rows = rows;
    m->cols = cols;
    lm_set_error(ctx, "");
    return m;
}

void lm_matrix_free(lm_context* ctx, lm_matrix* m) {
    (void)ctx;
    if (!m) return;
    free(m->data);
    m->data = NULL;
    free(m);
}

int lm_matrix_fill(lm_context* ctx, lm_matrix* m, const double* values, size_t len) {
    if (!ctx || !m || !m->data || !values) {
        lm_set_error(ctx, "invalid argument in lm_matrix_fill");
        return LM_ERR_INVALID_ARG;
    }

    size_t expected = m->rows * m->cols;
    if (len != expected) {
        lm_set_error(ctx, "shape mismatch in lm_matrix_fill");
        return LM_ERR_SHAPE_MISMATCH;
    }

    memcpy(m->data, values, sizeof(double) * expected);
    lm_set_error(ctx, "");
    return LM_OK;
}

int lm_matrix_mul(lm_context* ctx, const lm_matrix* a, const lm_matrix* b, lm_matrix* out) {
    if (!ctx || !a || !b || !out || !a->data || !b->data || !out->data) {
        lm_set_error(ctx, "invalid argument in lm_matrix_mul");
        return LM_ERR_INVALID_ARG;
    }

    if (a->cols != b->rows || out->rows != a->rows || out->cols != b->cols) {
        lm_set_error(ctx, "shape mismatch in lm_matrix_mul");
        return LM_ERR_SHAPE_MISMATCH;
    }

    size_t out_len = out->rows * out->cols;
    memset(out->data, 0, sizeof(double) * out_len);

    for (size_t i = 0; i < a->rows; ++i) {
        for (size_t k = 0; k < a->cols; ++k) {
            double a_ik = a->data[i * a->cols + k];
            for (size_t j = 0; j < b->cols; ++j) {
                out->data[i * out->cols + j] += a_ik * b->data[k * b->cols + j];
            }
        }
    }

    lm_set_error(ctx, "");
    return LM_OK;
}

int lm_matrix_add_inplace(lm_context* ctx, lm_matrix* a, const lm_matrix* b) {
    if (!ctx || !a || !b || !a->data || !b->data) {
        lm_set_error(ctx, "invalid argument in lm_matrix_add_inplace");
        return LM_ERR_INVALID_ARG;
    }

    if (a->rows != b->rows || a->cols != b->cols) {
        lm_set_error(ctx, "shape mismatch in lm_matrix_add_inplace");
        return LM_ERR_SHAPE_MISMATCH;
    }

    size_t n = a->rows * a->cols;
    for (size_t i = 0; i < n; ++i) {
        a->data[i] += b->data[i];
    }

    lm_set_error(ctx, "");
    return LM_OK;
}

int lm_matrix_sigmoid_inplace(lm_context* ctx, lm_matrix* m) {
    if (!ctx || !m || !m->data) {
        lm_set_error(ctx, "invalid argument in lm_matrix_sigmoid_inplace");
        return LM_ERR_INVALID_ARG;
    }

    size_t n = m->rows * m->cols;
    for (size_t i = 0; i < n; ++i) {
        double x = m->data[i];
        m->data[i] = 1.0 / (1.0 + exp(-x));
    }

    lm_set_error(ctx, "");
    return LM_OK;
}

size_t lm_matrix_rows(const lm_matrix* m) {
    if (!m) return 0;
    return m->rows;
}

size_t lm_matrix_cols(const lm_matrix* m) {
    if (!m) return 0;
    return m->cols;
}

int lm_matrix_data(const lm_context* ctx, const lm_matrix* m, const double** out_ptr, size_t* out_len) {
    if (!ctx || !m || !out_ptr || !out_len || !m->data) {
        return LM_ERR_INVALID_ARG;
    }

    *out_ptr = m->data;
    *out_len = m->rows * m->cols;
    return LM_OK;
}
