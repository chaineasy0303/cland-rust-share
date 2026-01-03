//! Simple usage example for cland-rust-share library
//!
//! This example demonstrates basic usage of the library's core modules.

use cland_rust_share::{CommonError, config, crypto, utils};

fn main() -> Result<(), CommonError> {
    println!("=== cland-rust-share Simple Usage Example ===");

    // Configuration example
    println!("\n1. Configuration Example:");
    let config = config::Config::builder()
        .with_app_name("example_app")
        .with_app_version("1.0.0")
        .with_environment("development")
        .with_debug(true)
        .with_database_url("postgres://localhost:5432/mydb")
        .with_encryption_key("my_secret_key_1234567890123456")
        .build();

    println!("   App Name: {}", config.app.name);
    println!("   Environment: {}", config.app.environment);
    println!("   Debug Mode: {}", config.app.debug);

    // Crypto example
    println!("\n2. Crypto Example:");
    let plaintext = "Hello, secret world!";
    let encryption_key = "0123456789abcdef0123456789abcdef"; // 32 bytes for AES-256

    let encrypted = crypto::aes_encrypt(plaintext, encryption_key)?;
    let decrypted = crypto::aes_decrypt(&encrypted, encryption_key)?;

    println!("   Original: {}", plaintext);
    println!("   Encrypted: {:?}", encrypted);
    println!("   Decrypted: {}", decrypted);

    // Hash example
    let data = b"Hello, World!";
    let hash = crypto::sha256_hash(data);
    println!("   SHA-256 Hash: {:02x?}", hash);

    // Random generation example
    let random_bytes = crypto::generate_random_bytes(16)?;
    let random_string = crypto::generate_random_string(12)?;
    println!("   Random Bytes: {:?}", random_bytes);
    println!("   Random String: {}", random_string);

    // Utils example
    println!("\n3. Utilities Example:");

    // String utilities
    let test_string = "HelloWorld";
    let snake_case = utils::to_snake_case(test_string);
    println!("   Snake Case: {} -> {}", test_string, snake_case);

    // Validation
    let email = "test@example.com";
    println!(
        "   Email '{}' is valid: {}",
        email,
        utils::is_valid_email(email)
    );

    let invalid_email = "invalid-email";
    println!(
        "   Email '{}' is valid: {}",
        invalid_email,
        utils::is_valid_email(invalid_email)
    );

    // Timestamp
    let timestamp = utils::current_timestamp();
    println!("   Current timestamp: {}", timestamp);

    // Duration formatting
    println!("   90 seconds: {}", utils::format_duration(90));
    println!("   3660 seconds: {}", utils::format_duration(3660));

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
