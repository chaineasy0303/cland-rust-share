use cland_rust_share::{Response, Pagination};

#[test]
fn test_response_success_serde() {
    let r = Response::success(serde_json::json!({"user_id":123, "username":"example"}));
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("\"code\":200"));
    assert!(s.contains("\"user_id\":123"));
}

#[test]
fn test_response_error_serde() {
    let r: Response<()> = Response::error(40010010001i64, "Invalid parameter: user_id is missing");
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("40010010001"));
    assert!(s.contains("Invalid parameter"));
}

#[test]
fn test_pagination() {
    let p = Pagination::new(100, 1, 10, vec![1,2,3]);
    assert_eq!(p.pages, 10);
}
