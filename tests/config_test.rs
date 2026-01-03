//! Integration tests for configuration module

use cland_rust_share::config;

#[test]
fn test_config_builder() {
    let config = config::Config::builder()
        .with_app_name("test_app")
        .with_app_version("1.0.0")
        .with_environment("test")
        .with_debug(true)
        .with_database_url("postgres://localhost:5432/test")
        .with_max_connections(20)
        .with_database_timeout(60)
        .with_encryption_key("test_key_1234567890123456789012")
        .with_encryption_algorithm("aes-256-gcm")
        .with_custom("custom_key", "custom_value")
        .build();

    assert_eq!(config.app.name, "test_app");
    assert_eq!(config.app.version, "1.0.0");
    assert_eq!(config.app.environment, "test");
    assert!(config.app.debug);
    assert_eq!(config.database.url, "postgres://localhost:5432/test");
    assert_eq!(config.database.max_connections, 20);
    assert_eq!(config.database.timeout_seconds, 60);
    assert_eq!(config.encryption.key, "test_key_1234567890123456789012");
    assert_eq!(config.encryption.algorithm, "aes-256-gcm");
    assert_eq!(
        config.custom.get("custom_key"),
        Some(&"custom_value".to_string())
    );
}

#[test]
fn test_config_default() {
    let config = config::Config::default();

    assert!(config.app.name.is_empty());
    assert!(config.app.version.is_empty());
    assert_eq!(config.app.environment, "development");
    assert!(!config.app.debug);
    assert!(config.database.url.is_empty());
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.database.timeout_seconds, 30);
    assert!(config.encryption.key.is_empty());
    assert_eq!(config.encryption.algorithm, "aes-256-gcm");
    assert!(config.custom.is_empty());
}
