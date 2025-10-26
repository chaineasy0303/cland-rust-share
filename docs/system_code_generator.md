# 系统状态码生成器 — 使用说明

本模块实现并封装了仓库中定义的 11 位结构化状态码生成、解析与校验逻辑，位置：`src/utils/http_code.rs`。

## 规则回顾
状态码采用 `{异常类别:3}{异常系统:4}{异常自定义:4}` 的结构，共 11 位（例如 `40010010001`）。

- `category`（3 位）: HTTP 类别，如 `200`、`400`、`500`。
- `system`（4 位）: 系统或模块标识，取值范围 `0..=9999`。
- `detail`（4 位）: 具体错误码，取值范围 `0..=9999`。

短码（仅 `200`/`400`/`500`）也被支持，解析后 `system` 与 `detail` 为 `0`。

## API（Rust）

模块路径：`cland_rust_share::utils::http_code`，同时本 crate 顶层已通过 `cland_rust_share::make_code` 等函数重导出，常用函数：

- `make_code(category: i32, system: i32, detail: i32) -> Result<i64, CodeError>`
  - 生成整型状态码。验证 `category`（只接受 200/400/500）以及 `system`/`detail` 在 `0..=9999` 范围内。
- `parse_code(code: i64) -> Result<StructuredCode, CodeError>`
  - 解析整型状态码为 `StructuredCode { category, system, detail }`。
- `is_valid_code(code: i64) -> bool` — 简单校验接口，出现解析错误返回 `false`。

示例：

```rust
use cland_rust_share::{make_code, parse_code};

let code = make_code(400, 1001, 1).unwrap();
assert_eq!(code, 40010010001i64);

let parsed = parse_code(code).unwrap();
assert_eq!(parsed.category, 400);
assert_eq!(parsed.system, 1001);
assert_eq!(parsed.detail, 1);
```

## 错误类型

`CodeError` 描述了常见的错误情况：非法类别、数值越界或负数等。生产使用中可以根据错误类型映射到合适的 HTTP 响应。

## 迁移与使用建议

- 该逻辑已移至 `utils` 目录（`src/utils/http_code.rs`），并由 `src/utils/mod.rs` 重新导出，外部可使用 `cland_rust_share::make_code` 直接调用。
- 若需要更严格的类别扩展（例如允许更多 category 值），可以将 `make_code` 中类别判断改为配置或由枚举管理。
