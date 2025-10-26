//! Internal shared library for common functionality across team projects.
//!
//! This library provides common utilities and abstractions used across multiple
//! team projects, including error handling, configuration management, and
//! cryptographic operations.

pub mod error;
pub mod config;
pub mod crypto;
pub mod utils;
pub mod http_response;

// Re-export commonly used types for convenience
pub use error::CommonError;
pub use config::Config;
pub use crypto::CryptoError;
pub use http_response::{Response, Pagination};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
