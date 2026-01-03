# API Response Types — 使用说明

This document explains how to use the `ApiResponse<T>` and `Pagination<T>` types for API responses.

## 结构概述

cland-rust-share 二方包按照以下结构组织：

```
src/
├── lib.rs           # 库入口，导出常用类型
├── dto/             # 数据传输层
│   ├── mod.rs       # 导出 DTO 类型和函数
│   └── common/      # 通用 DTO
│       ├── mod.rs
│       └── response.rs    # API 响应模型和构造器 (ApiResponse<T> + envelope 函数)
└── model/           # 核心数据模型层
    ├── mod.rs       # 导出模型类型
    └── common/      # 通用数据模型
        ├── mod.rs
        └── pagination.rs  # 分页模型 (Pagination<T>)
```

## 主要类型和函数

### `ApiResponse<T>` (位于 DTO 层)
- **位置**: `src/dto/common/response.rs`
- **导出路径**: `cland_rust_share::ApiResponse` 或 `cland_rust_share::dto::ApiResponse`
- **字段**:
  - `code: String` — 状态码 (使用字符串，支持项目的11位编码方案)
  - `msg: String` — 描述信息
  - `data: Option<T>` — 可选的响应数据
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
  - `success<T: Serialize>(data)` — 成功响应构造器
  - `param_error(msg)` — 参数错误构造器
  - `system_error(msg)` — 系统错误构造器
  - `error(code, msg)` — 无数据的错误构造器
  - `error_data(code, msg, data)` — 带数据的错误构造器

### `Pagination<T>` (核心数据模型)
- **位置**: `src/model/common/pagination.rs`
- **导出路径**: `cland_rust_share::Pagination`
- **字段**:
  - `total: u64` — 总记录数 (无符号整数)
  - `page: u64` — 当前页码 (从1开始，无符号整数)
  - `size: u64` — 每页大小 (无符号整数)
  - `pages: u64` — 总页数 (自动计算，无符号整数)
  - `list: Vec<T>` — 当前页数据
- **构造器**: `Pagination::new(total, page, size, list)` — 自动计算 `pages` (当 `size` 为0时返回0)

## 使用示例

### Rust 使用示例

```rust
use cland_rust_share::{ApiResponse, Pagination};

// 成功响应，包含数据
let resp = ApiResponse::success(serde_json::json!({ "user_id": 123, "username": "example" }));

// 错误响应，无数据
let err: ApiResponse<()> = ApiResponse::error("40010010001", "Invalid parameter: user_id is missing");

// 分页数据
let page = Pagination::new(100, 1, 10, vec!["a", "b", "c"]);
assert_eq!(page.pages, 10);

// 使用 DTO 构造器函数
use cland_rust_share::dto;
let json_response = dto::success(&serde_json::json!({ "status": "ok" }));
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

## 运行测试

从项目根目录运行标准的 cargo test 命令:

```powershell
cd G:\work\rust\cland-rust-share
cargo test
```

## 注意事项

- `ApiResponse.data` 是可选的，以匹配文档中错误响应可能省略 `data` 的情况
- `code` 使用 `String` 以支持各种编码方案，包括11位结构化编码
- `Pagination` 的所有数值字段使用无符号整数 (`u64`)
- 如果需要更严格的验证，可以考虑添加小型助手来生成和验证11位结构化编码
- 项目现在使用更抽象的结构，支持各种API协议，不限于HTTP
