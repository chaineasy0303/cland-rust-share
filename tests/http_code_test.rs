use cland_rust_share::{make_code, parse_code, is_valid_code, StructuredCode};

#[test]
fn make_and_parse_integration() {
    let code = make_code(400, 1001, 1).unwrap();
    assert_eq!(code, 40010010001i64);
    let parsed = parse_code(code).unwrap();
    assert_eq!(parsed, StructuredCode { category: 400, system: 1001, detail: 1 });
}

#[test]
fn validity_checks() {
    assert!(is_valid_code(200));
    assert!(is_valid_code(40010010001i64));
    assert!(!is_valid_code(201));
}
