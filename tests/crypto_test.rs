//! Integration tests for cryptographic module

use cland_rust_share::crypto;

#[test]
fn test_aes_encryption_decryption() {
    let plaintext = "Hello, World!";
    let key = "0123456789abcdef0123456789abcdef"; // 32 bytes
    
    let encrypted = crypto::aes_encrypt(plaintext, key).unwrap();
    let decrypted = crypto::aes_decrypt(&encrypted, key).unwrap();
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_aes_invalid_key_length() {
    let plaintext = "Hello, World!";
    let invalid_key = "short_key"; // Too short
    
    let result = crypto::aes_encrypt(plaintext, invalid_key);
    assert!(result.is_err());
    
    if let Err(crypto::CryptoError::InvalidKeyLength { expected, actual }) = result {
        assert_eq!(expected, 32);
        assert_eq!(actual, invalid_key.len());
    } else {
        panic!("Expected InvalidKeyLength error");
    }
}

#[test]
fn test_sha256_hash() {
    let data = b"test data";
    let hash = crypto::sha256_hash(data);
    
    assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
    assert_ne!(hash, vec![0u8; 32]); // Should not be all zeros
}

#[test]
fn test_generate_random_bytes() {
    let length = 16;
    let bytes = crypto::generate_random_bytes(length).unwrap();
    
    assert_eq!(bytes.len(), length);
    
    // Generate another set and ensure they're different (very high probability)
    let bytes2 = crypto::generate_random_bytes(length).unwrap();
    assert_ne!(bytes, bytes2);
}

#[test]
fn test_generate_random_string() {
    let length = 10;
    let random_string = crypto::generate_random_string(length).unwrap();
    
    assert_eq!(random_string.len(), length);
    
    // Should only contain alphanumeric characters
    assert!(random_string.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_xor_encryption() {
    let data = b"Hello, World!";
    let key = b"secret_key";
    
    let encrypted = crypto::xor_encrypt(data, key);
    let decrypted = crypto::xor_decrypt(&encrypted, key);
    
    assert_eq!(decrypted, data);
    assert_ne!(encrypted, data); // Encrypted should be different from original
}
