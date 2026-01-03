# API Response Types — 使用说明

This document explains how to use the `ApiResponse<T>` and `Pagination<T>` types for API responses in the cland-rust-share library.

## 项目结构概述

cland-rust-share 二方包按照以下结构组织（基于 .tree 文件）：

```
cland-rust-share/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── docs/
│  ├── HTTP 消息体标准定义.md
│  ├── http_response_usage.md
│  └── system_code_generator.md
├── examples/
│  └── simple_usage.rs
├── LICENSE
├── README.md
├── rustfmt.toml
├── src/
│  ├── config/
│  │  └── mod.rs                    # 配置管理
│  ├── crypto/
│  │  └── mod.rs                    # 加密操作
│  ├── dto/                         # 数据传输层
│  │  ├── mod.rs                    # 导出 DTO 类型和函数
│  │  └── common/                   # 通用 DTO
│  │      ├── mod.rs
│  │      └── response.rs           # API 响应模型和构造器 (ApiResponse<T> + envelope 函数)
│  ├── error.rs                     # 通用错误类型
│  ├── lib.rs                       # 库入口，导出常用类型
│  ├── model/                       # 核心数据模型层
│  │  ├── mod.rs                    # 导出模型类型
│  │  └── common/                   # 通用数据模型
│  │      ├── mod.rs
│  │      └── pagination.rs         # 分页模型 (Pagination<T>)
│  └── utils/
│      ├── http_code.rs             # HTTP 状态码和结构化编码工具
│      └── mod.rs
└── tests/
    ├── config_test.rs
    ├── crypto_test.rs
    ├── http_code_test.rs
    ├── http_error_code_test.rs
    └── utils_test.rs
```

## 主要类型和函数

### `ApiResponse<T>` (位于 DTO 层)
- **位置**: `src/dto/common/response.rs`
- **导出路径**: `cland_rust_share::ApiResponse` 或 `cland_rust_share::dto::ApiResponse`
- **字段**:
  - `code: String` — 业务状态码（字符串类型，支持项目的11位编码方案，200=成功）
  - `msg: String` — 响应描述信息
  - `data: Option<T>` — 可选的响应数据（失败时可省略）
- **常用构造器**:
  - `ApiResponse::success(data)` — 成功响应，包含数据和默认 `code = "200"`, `msg = "Success"`
  - `ApiResponse::ok()` — 成功响应，无数据
  - `ApiResponse::error(code, msg)` — 错误响应，包含状态码和消息
  - `ApiResponse::error_with_data(code, msg, data)` — 错误响应，包含数据

### 响应构造器函数
- **位置**: `src/dto/common/response.rs`
- **导出路径**: `cland_rust_share::dto::*`
- **函数**:
  - `envelope<T: Serialize>(code, msg, data)` — 通用 API JSON 构造器
  - `success<T: Serialize>(data)` — 成功响应构造器（返回 `serde_json::Value`）
  - `param_error(msg)` — 参数错误构造器（使用错误码 40010010001）
  - `system_error(msg)` — 系统错误构造器（使用错误码 50020030002）
  - `error(code, msg)` — 无数据的错误构造器
  - `error_data(code, msg, data)` — 带数据的错误构造器

### `Pagination<T>` (核心数据模型)
- **位置**: `src/model/common/pagination.rs`
- **导出路径**: `cland_rust_share::Pagination`
- **字段**:
  - `total: u64` — 总记录数（无符号整数）
  - `page: u64` — 当前页码（从1开始，无符号整数）
  - `size: u64` — 每页大小（无符号整数）
  - `pages: u64` — 总页数（自动计算，无符号整数）
  - `list: Vec<T>` — 当前页数据
- **构造器**: `Pagination::new(total, page, size, list)` — 自动计算 `pages`（当 `size` 为0时返回0）

### 错误码工具 (`ErrorCode` 和 `StructuredCode`)
- **位置**: `src/utils/http_code.rs`
- **导出路径**: `cland_rust_share::ErrorCode`, `cland_rust_share::StructuredCode`
- **主要功能**:
  - `ErrorCode` 枚举：`Ok` (200), `BadRequest` (400), `Internal` (500)
  - `StructuredCode` 结构体：解析和生成11位结构化编码
  - `make_code(category, system, detail)` — 生成11位编码
  - `parse_code(code)` — 解析编码（支持短码200/400/500和11位编码）
  - `is_valid_code(code)` — 验证编码有效性

## 使用示例

### Rust 使用示例

```rust
use cland_rust_share::{ApiResponse, Pagination, ErrorCode};
use serde_json::json;

// 成功响应，包含数据
let resp = ApiResponse::success(json!({ "user_id": 123, "username": "example" }));

// 错误响应，无数据
let err: ApiResponse<()> = ApiResponse::error("40010010001", "Invalid parameter: user_id is missing");

// 分页数据
let page = Pagination::new(100, 1, 10, vec!["a", "b", "c"]);
assert_eq!(page.pages, 10);

// 使用 DTO 构造器函数
use cland_rust_share::dto;
let json_response = dto::success(&json!({ "status": "ok" }));

// 使用错误码工具
let code = cland_rust_share::make_code(400, 1001, 1).unwrap(); // 40010010001
let structured = cland_rust_share::parse_code(40010010001).unwrap();
assert_eq!(structured.category, 400);
assert_eq!(structured.system, 1001);
assert_eq!(structured.detail, 1);
```

### JSON 输出示例

成功响应:
```json
{
  "code": "200",
  "msg": "Success",
  "data": { "user_id": 123, "username": "example" }
}
```

参数错误示例:
```json
{
  "code": "40010010001",
  "msg": "Invalid parameter: user_id is missing",
  "data": {
    "error_field": "user_id",
    "error_detail": "must be a positive integer"
  }
}
```

分页响应示例:
```json
{
  "code": "200",
  "msg": "Success",
  "data": {
    "total": 100,
    "page": 1,
    "size": 10,
    "pages": 10,
    "list": [
      { "id": 1, "name": "Item 1" },
      { "id": 2, "name": "Item 2" }
    ]
  }
}
```

## 运行测试

从项目根目录运行标准的 cargo test 命令:

```powershell
cd e:\work\code\rust\cland-rust-share
cargo test
```

或运行特定模块的测试:

```powershell
cargo test --test http_code_test      # 测试错误码工具
cargo test --test config_test         # 测试配置模块
cargo test --test crypto_test         # 测试加密模块
```

## 注意事项

1. **`ApiResponse.data` 是可选的**：以匹配文档中错误响应可能省略 `data` 的情况
2. **`code` 使用 `String` 类型**：以支持各种编码方案，包括11位结构化编码
3. **`Pagination` 使用无符号整数**：所有数值字段使用 `u64` 类型
4. **结构化编码支持**：项目支持11位结构化编码（格式：3位分类 + 4位系统 + 4位详情）
5. **utoipa 支持**：通过 `utoipa_support` feature 启用 OpenAPI/Swagger 文档生成
6. **向后兼容**：`api/http_response.rs` 已弃用，功能已迁移到 `dto` 和 `model` 模块

## 模块导出说明

在 `src/lib.rs` 中导出的常用类型：

```rust
pub use config::Config;
pub use crypto::CryptoError;
pub use dto::ApiResponse;
pub use error::CommonError;
pub use model::Pagination;
pub use utils::ErrorCode;
pub use utils::{StructuredCode, is_valid_code, make_code, parse_code};
```

这意味着你可以直接使用 `cland_rust_share::ApiResponse` 而不是 `cland_rust_share::dto::ApiResponse`。
