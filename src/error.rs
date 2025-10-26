//! Unified error type definitions for the library.
//!
//! This module provides a common error type `CommonError` that can represent
//! various error conditions across different modules.

use thiserror::Error;

/// Unified error type for the library.
///
/// This enum aggregates all possible error types that can occur when using
/// the library's functionality.
#[derive(Error, Debug)]
pub enum CommonError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Cryptographic operation errors
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// I/O operation errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Custom error with message
    #[error("{0}")]
    Custom(String),
}

impl CommonError {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a new cryptographic error
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Crypto(msg.into())
    }

    /// Create a new serialization error
    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::Serialization(msg.into())
    }

    /// Create a new network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a new custom error
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}
