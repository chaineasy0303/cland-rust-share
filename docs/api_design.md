# API 设计说明

## 1. 设计原则

### 1.1 统一错误处理
- 使用 `thiserror` 库定义统一的错误类型 `CommonError`
- 所有公共 API 返回 `Result<T, CommonError>`
- 提供便捷的错误构造方法（如 `CommonError::config()`）

### 1.2 最小暴露原则
- 仅通过 `pub` 导出下游必需的接口
- 内部实现设为私有，避免不必要的依赖
- 通过 `pub use` 重新导出常用类型，简化下游使用

### 1.3 一致性命名
- 模块名使用 snake_case（如 `config`、`crypto`）
- 函数名使用 snake_case
- 类型名使用 PascalCase

## 2. 模块设计

### 2.1 错误模块 (`error.rs`)
```rust
pub enum CommonError {
    Config(String),
    Crypto(String),
    Io(std::io::Error),
    // ...
}
```

**设计考虑：**
- 聚合所有可能的错误类型
- 提供 `From` 转换，兼容标准库错误
- 支持详细的错误信息

### 2.2 配置模块 (`config/`)
```rust
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    // ...
}

pub struct ConfigBuilder {
    // 构建器模式实现
}
```

**设计考虑：**
- 使用构建器模式提供灵活的配置构造
- 支持环境变量加载
- 提供合理的默认值

### 2.3 加密模块 (`crypto/`)
```rust
pub fn aes_encrypt(plaintext: &str, key: &str) -> Result<Vec<u8>, CryptoError>
pub fn sha256_hash(data: &[u8]) -> Vec<u8>
```

**设计考虑：**
- 提供常用加密操作
- 明确的错误类型区分
- 安全的默认参数

### 2.4 工具模块 (`utils/`)
```rust
pub mod string {
    pub fn is_blank(s: &str) -> bool
    pub fn to_snake_case(s: &str) -> String
}

pub mod datetime {
    pub fn current_timestamp() -> u64
}
```

**设计考虑：**
- 按功能分组（字符串、时间、文件系统等）
- 提供常用工具函数
- 避免过度拆分

## 3. 导出策略

### 3.1 库入口 (`lib.rs`)
```rust
pub mod error;
pub mod config;
pub mod crypto;
pub mod utils;

// 重新导出常用类型
pub use error::CommonError;
pub use config::Config;
pub use crypto::CryptoError;
```

**设计考虑：**
- 通过 `pub use` 简化下游导入
- 保持 `lib.rs` 简洁，仅做模块声明和重新导出
- 避免在 `lib.rs` 中写具体实现

### 3.2 模块可见性控制
- 模块根目录通过 `mod.rs` 控制导出
- 仅导出必要的公共接口
- 内部辅助函数设为私有

## 4. 错误处理策略

### 4.1 错误转换
```rust
impl From<CryptoError> for CommonError {
    fn from(err: CryptoError) -> Self {
        CommonError::crypto(err.to_string())
    }
}
```

### 4.2 错误信息
- 提供详细的错误上下文
- 支持错误链（通过 `#[from]`）
- 统一的错误格式化

## 5. 性能考虑

### 5.1 零成本抽象
- 使用 Rust 的所有权系统避免不必要的拷贝
- 合理使用引用和切片
- 避免不必要的堆分配

### 5.2 内存管理
- 使用 `Vec<u8>` 处理二进制数据
- 合理使用 `String` 和 `&str`
- 避免内存泄漏

## 6. 扩展性设计

### 6.1 模块化设计
- 每个模块独立，可单独使用
- 清晰的模块边界
- 松耦合的模块关系

### 6.2 版本兼容
- 遵循语义化版本控制
- 避免破坏性变更
- 提供迁移指南

## 7. 测试策略

### 7.1 单元测试
- 每个模块包含内部测试
- 测试边界条件和错误情况

### 7.2 集成测试
- `tests/` 目录包含跨模块测试
- 模拟下游使用场景

### 7.3 性能测试
- `benches/` 目录包含性能基准测试
- 监控核心函数性能变化

## 8. 文档标准

### 8.1 代码文档
- 所有公共 API 包含 `///` 文档注释
- 使用示例代码
- 说明前置条件和后置条件

### 8.2 用户文档
- `README.md` 提供快速开始指南
- `examples/` 目录包含使用示例
- `docs/` 目录包含详细设计文档

通过以上设计，确保库的易用性、可维护性和扩展性。
