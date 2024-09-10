#include "common.h"
#include "vector.h"
#include <stdio.h>
#include <string.h>

u64 m;
bool *visited;

/// 输出图
void debug(Vector *table) {
    // 添加边
    for (int i=0; i < m; i++) {
        for (int j=0; j < table[i].size; j++) {
            edge(i, table[i].data[j]);
        }
    }
    recoder();
}

void dfs(Vector *table, u64 from) {
    if (visited[from]) return;
    visited[from] = true;

    for (int i=0; i < table[from].size; i++) {
        dfs(table, table[from].data[i]);
    }
}

// 测试
int main() {
    scanf("%lu", &m);
    // 临接矩阵
    Vector table[m];
    for (int i=0; i < m; i++) table[i] = *vec_new();

    // 初始化
    u64 from, to;
    for (int i=0; i < m; i++) {
        scanf("%lu %lu", &from, &to);
        vec_push(&table[from], to);
    }
    // 打印
    for (int i=0; i < m; i++) {
        printf("%d: ", i);
        for (int j=0; j < table[i].size; j++) {
            printf("%lu ", table[i].data[j]);
        }
        printf("\n");
    }


    visited = malloc(sizeof(bool) * m);
    memset(visited, 0, sizeof(bool) * m);

    // 深度遍历
    dfs(table, 0);
    for (int i=0; i < m; i++) {
        if (!visited[i]) {
            printf("%d is not visited\n", i);
        } else {
            printf("%d is visited\n", i);
        }
    }
    free(visited);

    debug(table);
    return 0;
}
/* 输入
4
0 1
1 2
2 3
1 3
*/

