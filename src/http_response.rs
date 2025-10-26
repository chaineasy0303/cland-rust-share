use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 通用 HTTP JSON envelope 构造器
///
/// 格式:
/// {
///   "code": <int>,
///   "msg": "<string>",
///   "data": <any|null>
/// }
pub fn envelope<T: Serialize>(code: i64, msg: impl AsRef<str>, data: Option<&T>) -> Value {
    let data_value = match data {
        Some(d) => serde_json::to_value(d).unwrap_or(Value::Null),
        None => Value::Null,
    };
    serde_json::json!({
        "code": code,
        "msg": msg.as_ref(),
        "data": data_value
    })
}

/// 快捷构造器
pub fn ok<T: Serialize>(data: &T) -> Value {
    envelope(crate::ErrorCode::Ok.value(), "", Some(data))
}

/// 简化的对外 API：只提供三个常用构造器
///
/// - `success`：200 成功
/// - `param_error`：参数错误，使用文档示例的参数错误码 40010010001
/// - `system_error`：系统错误，使用文档示例的系统错误码 50020030002
pub fn success<T: Serialize>(data: &T) -> Value {
    envelope(crate::ErrorCode::Ok.value(), "Success", Some(data))
}

pub fn param_error(msg: &str) -> Value {
    envelope::<()>(crate::ErrorCode::BadRequest.value(), msg, None::<&()> )
}

pub fn system_error(msg: &str) -> Value {
    envelope::<()>(crate::ErrorCode::Internal.value(), msg, None::<&()> )
}

/// 无 data 的错误/提示
pub fn error(code: i64, msg: &str) -> Value {
    envelope::<()>(code, msg, None::<&()>)
}

/// 带 data 的错误
pub fn error_data<T: Serialize>(code: i64, msg: &str, data: &T) -> Value {
    envelope(code, msg, Some(data))
}

/// Standard HTTP response body defined in docs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response<T> {
    /// 状态码
    pub code: i64,
    /// 描述信息
    pub msg: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Response<T> {
    /// Create a success response with data
    pub fn success(data: T) -> Self {
        Response { code: 200, msg: "Success".to_string(), data: Some(data) }
    }

    /// Create a success response without data
    pub fn ok() -> Self {
        Response { code: 200, msg: "Success".to_string(), data: None }
    }

    /// Create an error response with code and message
    pub fn error(code: i64, msg: impl Into<String>) -> Self {
        Response { code, msg: msg.into(), data: None }
    }

    /// Create an error response with data
    pub fn error_with_data(code: i64, msg: impl Into<String>, data: T) -> Self {
        Response { code, msg: msg.into(), data: Some(data) }
    }
}

/// Pagination result structure as described in docs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pagination<T> {
    pub total: i64,
    pub page: i64,
    pub size: i64,
    pub pages: i64,
    pub list: Vec<T>,
}

impl<T> Pagination<T> {
    pub fn new(total: i64, page: i64, size: i64, list: Vec<T>) -> Self {
        let pages = if size > 0 { (total + size - 1) / size } else { 0 };
        Pagination { total, page, size, pages, list }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_success_with_data() {
        let r = Response::success(serde_json::json!({ "user_id": 123, "username": "example" }));
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("\"code\":200"));
        assert!(s.contains("\"user_id\":123"));
    }

    #[test]
    fn serialize_error() {
        let r: Response<()> = Response::error(40010010001i64, "Invalid parameter: user_id is missing");
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("40010010001"));
        assert!(s.contains("Invalid parameter"));
    }

    #[test]
    fn pagination_basic() {
        let p = Pagination::new(100, 1, 10, vec![1,2,3]);
        assert_eq!(p.pages, 10);
    }
}
