# HTTP response types — 使用说明

This document explains how to use the `Response<T>` and `Pagination<T>` types introduced in `src/http_response.rs`.

## Types

- `Response<T>`
  - Fields:
    - `code: i64` — numeric status code (use the project's 11-digit scheme when needed).
    - `msg: String` — human-readable message.
    - `data: Option<T>` — optional response payload.
  - Common constructors:
    - `Response::success(data)` — success response with data and default `code = 200`, `msg = "Success"`.
    - `Response::ok()` — success response with no data.
    - `Response::error(code, msg)` — error response with code and message.
    - `Response::error_with_data(code, msg, data)` — error response carrying data.

- `Pagination<T>`
  - Fields:
    - `total: i64` — total number of items.
    - `page: i64` — current page (1-based).
    - `size: i64` — items per page.
    - `pages: i64` — total pages (calculated).
    - `list: Vec<T>` — items on current page.
  - Constructor: `Pagination::new(total, page, size, list)` — calculates `pages` automatically (returns 0 when `size` is 0).

## Examples

Rust usage example:

```rust
use cland_rust_share::{Response, Pagination};

// Success with data
let resp = Response::success(serde_json::json!({ "user_id": 123, "username": "example" }));

// Error without data
let err: Response<()> = Response::error(40010010001i64, "Invalid parameter: user_id is missing");

// Pagination
let page = Pagination::new(100, 1, 10, vec!["a", "b", "c"]);
assert_eq!(page.pages, 10);
```

JSON examples emitted by serde:

Success:

```json
{
  "code": 200,
  "msg": "Success",
  "data": { "user_id": 123, "username": "example" }
}
```

Parameter error example:

```json
{
  "code": 40010010001,
  "msg": "Invalid parameter: user_id is missing",
  "data": {
    "error_field": "user_id",
    "error_detail": "must be a positive integer"
  }
}
```

## Running tests

From the repository root run the standard cargo test command. On Windows PowerShell:

```powershell
cd D:\work\code\cland-rust-share
cargo test
```

The repository's tests were run after adding these types and passed locally.

## Notes and recommendations

- `Response.data` is optional to match the documentation where error responses may omit `data`.
- `code` uses `i64` to safely hold the 11-digit structured codes described in the HTTP message document.
- Consider adding a small helper to generate and validate the 11-digit structured codes if you need stricter enforcement.
