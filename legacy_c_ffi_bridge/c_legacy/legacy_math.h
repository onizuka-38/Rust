#ifndef LEGACY_MATH_H
#define LEGACY_MATH_H

#include <stddef.h>

#define LM_OK 0
#define LM_ERR_INVALID_ARG 1
#define LM_ERR_SHAPE_MISMATCH 2
#define LM_ERR_ALLOC 3

typedef struct lm_context lm_context;
typedef struct lm_matrix lm_matrix;

lm_context* lm_context_new(void);
void lm_context_free(lm_context* ctx);
const char* lm_last_error(const lm_context* ctx);

lm_matrix* lm_matrix_new(lm_context* ctx, size_t rows, size_t cols);
void lm_matrix_free(lm_context* ctx, lm_matrix* m);

int lm_matrix_fill(lm_context* ctx, lm_matrix* m, const double* values, size_t len);
int lm_matrix_mul(lm_context* ctx, const lm_matrix* a, const lm_matrix* b, lm_matrix* out);
int lm_matrix_add_inplace(lm_context* ctx, lm_matrix* a, const lm_matrix* b);
int lm_matrix_sigmoid_inplace(lm_context* ctx, lm_matrix* m);

size_t lm_matrix_rows(const lm_matrix* m);
size_t lm_matrix_cols(const lm_matrix* m);
int lm_matrix_data(const lm_context* ctx, const lm_matrix* m, const double** out_ptr, size_t* out_len);

#endif
