# Rust 笔记
## 1.升级
``````
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
``````
## 2.项目构建常用命令
``````
$ cargo new <project_name>  #新疆项目
$ cargo check               #检查代码是否能编译
$ cargo build               #编译
$ cargo run                 #运行
$ cargo build --release     #编译为release版本（开启优化）
``````