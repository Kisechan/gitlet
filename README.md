# CS61B Gitlet - Rust Implementation

> A simplified version control system inspired by Git, implemented in Rust as a solution to UC Berkeley CS61B Project 2

## 项目概述

UCB CS61B SP21 Project 2 Gitlet 的 Rust 实现。

- 官方 Spec：[https://sp21.datastructur.es/materials/proj/proj2/proj2](https://sp21.datastructur.es/materials/proj/proj2/proj2)
- 官方框架仓库：[https://github.com/Berkeley-CS61B/skeleton-sp21](https://github.com/Berkeley-CS61B/skeleton-sp21)
  - 参考提供的 Java 代码，编写了 Rust 的代码框架，存放在 `skeleton` 分支

**仅供学习交流使用**。

## 实现功能

实现了 Spec 中 `init` 到 `merge` 命令的所有功能，也就是本地的提交、分支管理、合并等。~~有点累了没做 Extra Credit，所以一个远程的功能都没做。~~

## 项目架构

```
src/
├── main.rs              # 命令行接口和参数解析
├── repository.rs        # 仓库核心逻辑
├── commit.rs            # 提交
├── blob.rs              # 文件内容的二进制对象表示
├── index.rs             # 索引
├── utils.rs             # 工具函数（文件 I/O、SHA-1 哈希等）
└── lib.rs               # 库的导出
```

## 编译和运行

要求：
- Rust 2021 或更高版本
- Cargo

### 编译

```bash
# 在项目根目录执行
cargo build --bin gitlet
```

### 运行

```bash
# 基本形式
cargo run --bin gitlet -- <命令> [参数]

# 例如初始化仓库
cargo run --bin gitlet -- init
```

## 测试

可以参考官方代码仓库中提供的测试 Python 脚本编写测试功能，使用方法例如：

```bash
cd testing
python3 runner.py samples/*.in
```

预期输出：

```
test01-init: 
OK
test02-basic-checkout: 
OK
test03-basic-log: 
OK
test04-prev-checkout: 
OK

Ran 4 tests. All passed.
```
