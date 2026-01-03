//! Cryptographic utilities module.
//!
//! This module provides common cryptographic operations including
//! encryption, decryption, hashing, and secure random generation.

/// Cryptographic error type
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    /// Invalid key length
    #[error("Invalid key length: expected {expected}, got {actual}")]
    InvalidKeyLength { expected: usize, actual: usize },

    /// Invalid input data
    #[error("Invalid input data: {0}")]
    InvalidInput(String),

    /// Encryption/decryption failed
    #[error("Crypto operation failed: {0}")]
    OperationFailed(String),
}

/// AES-256-GCM encryption
pub fn aes_encrypt(plaintext: &str, key: &str) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength {
            expected: 32,
            actual: key.len(),
        });
    }

    // Placeholder implementation - in real scenario, use proper crypto library
    // like ring or openssl
    let mut result = Vec::from(plaintext.as_bytes());
    result.extend_from_slice(key.as_bytes());
    Ok(result)
}

/// AES-256-GCM decryption
pub fn aes_decrypt(ciphertext: &[u8], key: &str) -> Result<String, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength {
            expected: 32,
            actual: key.len(),
        });
    }

    if ciphertext.len() <= key.len() {
        return Err(CryptoError::InvalidInput(
            "Ciphertext too short".to_string(),
        ));
    }

    // Placeholder implementation
    let plaintext_len = ciphertext.len() - key.len();
    let plaintext_bytes = &ciphertext[..plaintext_len];

    String::from_utf8(plaintext_bytes.to_vec())
        .map_err(|e| CryptoError::InvalidInput(format!("Invalid UTF-8: {}", e)))
}

/// Generate SHA-256 hash
pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generate secure random bytes
pub fn generate_random_bytes(length: usize) -> Result<Vec<u8>, CryptoError> {
    use rand::RngCore;

    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; length];
    rng.fill_bytes(&mut bytes);
    Ok(bytes)
}

/// Generate secure random string
pub fn generate_random_string(length: usize) -> Result<String, CryptoError> {
    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen_range(0..CHARSET.len());
        result.push(CHARSET[idx] as char);
    }

    Ok(result)
}

/// Simple XOR encryption (for demonstration purposes)
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key[i % key.len()];
        result.push(byte ^ key_byte);
    }

    result
}

/// Simple XOR decryption (for demonstration purposes)
pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_encrypt(data, key) // XOR is symmetric
}

// Re-export rand for convenience
pub use rand;
