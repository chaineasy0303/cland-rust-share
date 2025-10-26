//! Integration tests for utilities module

use cland_rust_share::utils;

#[test]
fn test_string_utilities() {
    // Test is_blank
    assert!(utils::is_blank(""));
    assert!(utils::is_blank("   "));
    assert!(utils::is_blank("\t\n"));
    assert!(!utils::is_blank("hello"));
    assert!(!utils::is_blank("  hello  "));

    // Test to_snake_case
    assert_eq!(utils::to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(utils::to_snake_case("HTTPRequest"), "http_request");
    assert_eq!(utils::to_snake_case("already_snake"), "already_snake");
    assert_eq!(utils::to_snake_case(""), "");

    // Test truncate_with_ellipsis
    assert_eq!(utils::truncate_with_ellipsis("hello", 5), "hello");
    assert_eq!(utils::truncate_with_ellipsis("hello world", 8), "hello...");
    assert_eq!(utils::truncate_with_ellipsis("hello", 3), "...");
    assert_eq!(utils::truncate_with_ellipsis("hello", 2), "...");
}

#[test]
fn test_datetime_utilities() {
    // Test current timestamps (just verify they return reasonable values)
    let timestamp_secs = utils::current_timestamp();
    let timestamp_millis = utils::current_timestamp_millis();
    
    assert!(timestamp_secs > 0);
    assert!(timestamp_millis > 0);
    assert!(timestamp_millis > timestamp_secs as u128 * 1000);

    // Test format_duration
    assert_eq!(utils::format_duration(30), "30s");
    assert_eq!(utils::format_duration(90), "1m 30s");
    assert_eq!(utils::format_duration(3660), "1h 1m");
    assert_eq!(utils::format_duration(90000), "1d 1h");
}

#[test]
fn test_validation_utilities() {
    // Test email validation
    assert!(utils::is_valid_email("test@example.com"));
    assert!(utils::is_valid_email("user.name+tag@domain.co.uk"));
    assert!(!utils::is_valid_email("invalid-email"));
    assert!(!utils::is_valid_email("@domain.com"));
    assert!(!utils::is_valid_email("user@.com"));

    // Test URL validation
    assert!(utils::is_valid_url("https://example.com"));
    assert!(utils::is_valid_url("http://localhost:3000"));
    assert!(!utils::is_valid_url("not-a-url"));
    assert!(!utils::is_valid_url("ftp://example.com")); // Only http/https supported
    assert!(!utils::is_valid_url("example.com")); // Missing protocol

    // Test phone validation (simple check)
    assert!(utils::is_valid_phone("+1234567890"));
    assert!(utils::is_valid_phone("123-456-7890"));
    assert!(utils::is_valid_phone("(123) 456-7890"));
    assert!(!utils::is_valid_phone("123")); // Too short
    assert!(!utils::is_valid_phone("invalid-phone"));
}

#[test]
fn test_collection_utilities() {
    use cland_rust_share::utils::collection;

    // Test vec_to_hashmap
    let vec = vec!["a", "b", "c"];
    let map = collection::vec_to_hashmap(vec, |&s| (s.to_string(), s.len()));
    
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&1));
    assert_eq!(map.get("c"), Some(&1));
    assert_eq!(map.len(), 3);

    // Test all_unique
    assert!(collection::all_unique(&[1, 2, 3, 4, 5]));
    assert!(!collection::all_unique(&[1, 2, 3, 2, 5]));
    assert!(collection::all_unique::<i32>(&[])); // Empty vector is unique
    assert!(collection::all_unique(&["a", "b", "c"]));
}
