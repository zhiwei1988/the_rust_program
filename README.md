# the_rust_program

本项目用于记录 rust 语言的学习过程

## 使用 Cargo 命令

本项目是一个 workspace 包含多个 package

在 Workspace 根目录运行

- cargo build: 构建 workspace 中的所有 packages。
- cargo test: 运行 workspace 中所有 packages 的测试。
- cargo run -p package_one: 运行名为 package_one 的二进制 package (如果它是一个可执行文件)。
- cargo check: 检查 workspace 中所有 packages 的代码。
- cargo clean: 清理所有 package 的构建产物。

在特定 Package 目录运行

- cd package_one
- cargo build: 只构建 package_one。
- cargo test: 只运行 package_one 的测试。