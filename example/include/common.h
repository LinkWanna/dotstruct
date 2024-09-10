#ifndef COMMON_H
#define COMMON_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t  i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

// 用于测试的测试样本大小
#define SIZE 20

// 转换数字为字符串
inline char* str(u64 n) {
    char* s = (char* )malloc(sizeof(char) * 1024);
    sprintf(s, "%lu", n);
    return s;
}

extern void recoder();

extern void node(u64 id, char* label);
extern void edge(u64 from, u64 to);

#endif