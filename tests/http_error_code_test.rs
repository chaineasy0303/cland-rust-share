use cland_rust_share::ErrorCode;

#[test]
fn error_code_values() {
    assert_eq!(ErrorCode::Ok.value(), 200);
    assert_eq!(ErrorCode::BadRequest.value(), 400);
    assert_eq!(ErrorCode::Internal.value(), 500);
}

#[test]
fn error_code_structured() {
    let s = ErrorCode::BadRequest.as_structured();
    assert_eq!(s.category, 400);
    assert_eq!(s.system, 0);
    assert_eq!(s.detail, 0);
}
