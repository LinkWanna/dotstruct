#include "vector.h"


// 创建一个默认容量的空 vector
Vector* vec_new() {
    Vector* vec = (Vector*) malloc(sizeof(Vector));
    vec->data = malloc(sizeof(T) * SIZE);
    vec->size = 0;
    vec->capacity = SIZE;
    return vec;
}

void vec_push(Vector *v, T value) {
    // 扩容
    if (v->size >= v->capacity) {
        v->capacity *= 2;
        v->data = realloc(v->data, sizeof(T) * v->capacity);
    }
    v->data[v->size++] = value;
}

// 弹出一个元素
T vec_pop(Vector *v) {
    // 如果 v->size == 0，直接报错
    assert(v->size > 0);
    return v->data[--(v->size)];
}

// 获取最后一个元素
T vec_peek(Vector *v) {
    assert(v->size > 0);
    return v->data[v->size - 1];
}

bool vec_contain(Vector *v, T value) {
    for (int i = 0; i < v->size; i++) {
        if (v->data[i] == value) return true;
    }
    return false;
}

// 释放 vector
void vec_free(Vector *v) {
    free(v->data);
    free(v);
}

// int main() {
//     Vector v = vec_new();
//     for (T i = 0; i < SIZE; i++) {
//         vec_append(&v, i);
//     }
//     for (T i = 0; i < v.size; i++) {
//         printf("%lu\n", v.data[i]);
//     }
//     return 0;
// }
