# cland-rust-share

Internal shared library for common functionality across team projects.

## 功能简介

提供团队内部通用的 Rust 功能模块，包括：
- 统一错误处理
- 配置管理
- 加密工具
- HTTP 客户端
- 常用工具函数

## 快速开始

### 安装

在项目的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
cland-rust-share = { git = "https://github.com/chaineasy0303/cland-rust-share.git" }
```

### 基础用法

```rust
use cland_rust_share::{CommonError, config, crypto};

fn main() -> Result<(), CommonError> {
    // 使用配置模块
    let config = config::load_default()?;
    
    // 使用加密模块
    let encrypted = crypto::encrypt("secret data", &config.encryption_key)?;
    
    Ok(())
}
```

## 核心 API

### 错误处理

```rust
use cland_rust_share::CommonError;

fn process_data() -> Result<String, CommonError> {
    // 统一错误处理
    let data = read_file("data.txt")?;
    Ok(data)
}
```

### 配置管理

```rust
use cland_rust_share::config;

let config = config::Config::builder()
    .with_env_prefix("APP")
    .load()?;
```

### 加密工具

```rust
use cland_rust_share::crypto;

let encrypted = crypto::aes_encrypt("plaintext", "key")?;
let decrypted = crypto::aes_decrypt(&encrypted, "key")?;
```

## 版本变更记录

### v0.1.0 (当前版本)
- 初始版本发布
- 基础项目结构搭建
- 统一错误处理模块
- 配置管理模块
- 加密工具模块

详细变更记录请查看 [docs/changelog.md](docs/changelog.md)

## 维护者

- Team Maintainers

## 联系方式

如有问题或建议，请通过团队内部渠道联系维护者。
