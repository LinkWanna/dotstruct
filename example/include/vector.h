#ifndef VECTOR_H
#define VECTOR_H
#include "common.h"

#define T u64

// 默认容量
const static u64 default_size = 20;

typedef struct {
    T *data;
    u64 size;
    u64 capacity;
} Vector;

Vector* vec_new();

void vec_push(Vector *v, T value);

bool vec_contains(Vector *v, T value);

void vec_free(Vector *v);

#endif