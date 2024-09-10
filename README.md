
这是一个基于Rust的写的C语言数据结构可视化的库，把dot Language中的一些关键信息进行了简单的封装，最后利用graphviz的dot工具来生成图可视化的图


进入项目目录，执行以下命令
```sh
# 添加依赖
sudo pacman -S graphviz

# 构建动态库
cargo build -r
cp target/release/libdotstruct.so example/libdotstruct.so

# 添加动态链接库
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/example/libdotstruct.so

# 运行测试（测试使用的 C 语言版本为 C23）
cd example
make NAME=gragh_table run
```
