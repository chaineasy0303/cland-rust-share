use serde::{Deserialize, Serialize};
use serde_json::Value;

// 条件导入：仅保留必要的 ToSchema（4.2.3 兼容）
#[cfg(feature = "utoipa_support")]
use utoipa::ToSchema;

use crate::ErrorCode;

/// 全局统一 API 响应体格式
///
/// 格式说明：
/// - code: 业务状态码（字符串类型，200=成功）
/// - msg: 响应描述信息
/// - data: 响应数据（可选，失败时可省略）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// 条件派生：完全依赖派生宏生成 ToSchema
#[cfg_attr(feature = "utoipa_support", derive(ToSchema))]
// 结构体 Swagger 示例（仅保留 example，移除不支持的 description）
#[cfg_attr(
    feature = "utoipa_support",
    schema(
        example = json!({
            "code": "200",
            "msg": "Success",
            "data": { "user_id": "123e4567-e89b-12d3-a456-426614174000", "username": "example" }
        })
    )
)]
pub struct ApiResponse<T> {
    /// 业务状态码（200=成功，常见错误码：40010010001=参数错误、50020030002=系统错误）
    #[cfg_attr(
        feature = "utoipa_support",
        schema(example = "200")  // 仅保留支持的 example 属性
    )]
    pub code: String,

    /// 响应描述信息（成功时为 Success，错误时为具体原因）
    #[cfg_attr(
        feature = "utoipa_support",
        schema(example = "Success")  // 仅保留支持的 example 属性
    )]
    pub msg: String,

    /// 响应数据（可选，失败时可省略）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "utoipa_support",
        schema(nullable = true)  // 仅保留支持的 nullable 属性
    )]
    pub data: Option<T>,
}

// 原有业务逻辑完全不变（以下代码无修改）
impl<T> ApiResponse<T> {
    /// Create a success response with data
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: "200".to_string(),
            msg: "Success".to_string(),
            data: Some(data),
        }
    }

    /// Create a success response without data
    pub fn ok() -> Self {
        ApiResponse {
            code: "200".to_string(),
            msg: "Success".to_string(),
            data: None,
        }
    }

    /// Create an error response with code and message
    pub fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        ApiResponse {
            code: code.into(),
            msg: msg.into(),
            data: None,
        }
    }

    /// Create an error response with data
    pub fn error_with_data(code: impl Into<String>, msg: impl Into<String>, data: T) -> Self {
        ApiResponse {
            code: code.into(),
            msg: msg.into(),
            data: Some(data),
        }
    }
}

/// 通用 API JSON envelope 构造器
///
/// 格式:
/// {
///   "code": <string>,
///   "msg": "<string>",
///   "data": <any|null>
/// }
pub fn envelope<T: Serialize>(
    code: impl AsRef<str>,
    msg: impl AsRef<str>,
    data: Option<&T>,
) -> Value {
    let data_value = match data {
        Some(d) => serde_json::to_value(d).unwrap_or(Value::Null),
        None => Value::Null,
    };
    serde_json::json!({
        "code": code.as_ref(),
        "msg": msg.as_ref(),
        "data": data_value
    })
}

/// 快捷构造器
pub fn ok<T: Serialize>(data: &T) -> Value {
    envelope(ErrorCode::Ok.value().to_string(), "", Some(data))
}

/// 简化的对外 API：只提供三个常用构造器
///
/// - `success`：200 成功
/// - `param_error`：参数错误，使用文档示例的参数错误码 40010010001
/// - `system_error`：系统错误，使用文档示例的系统错误码 50020030002
pub fn success<T: Serialize>(data: &T) -> Value {
    envelope(ErrorCode::Ok.value().to_string(), "Success", Some(data))
}

pub fn param_error(msg: &str) -> Value {
    envelope::<()>(ErrorCode::BadRequest.value().to_string(), msg, None::<&()>)
}

pub fn system_error(msg: &str) -> Value {
    envelope::<()>(ErrorCode::Internal.value().to_string(), msg, None::<&()>)
}

/// 无 data 的错误/提示
pub fn error(code: impl AsRef<str>, msg: &str) -> Value {
    envelope::<()>(code, msg, None::<&()>)
}

/// 带 data 的错误
pub fn error_data<T: Serialize>(code: impl AsRef<str>, msg: &str, data: &T) -> Value {
    envelope(code, msg, Some(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_success_with_data() {
        let r = ApiResponse::success(serde_json::json!({ "user_id": 123, "username": "example" }));
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("\"code\":\"200\""));
        assert!(s.contains("\"user_id\":123"));
    }

    #[test]
    fn serialize_error() {
        let r: ApiResponse<()> =
            ApiResponse::error("40010010001", "Invalid parameter: user_id is missing");
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("\"code\":\"40010010001\""));
        assert!(s.contains("Invalid parameter"));
    }
}
