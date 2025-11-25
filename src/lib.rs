//! Internal shared library for common functionality across team projects.
//!
//! This library provides common utilities and abstractions used across multiple
//! team projects, including error handling, configuration management, and
//! cryptographic operations.

pub mod error;
pub mod config;
pub mod crypto;
pub mod utils;
pub mod api;
pub mod model;
pub mod dto;

// Re-export commonly used types for convenience
pub use config::Config;
pub use crypto::CryptoError;
pub use model::Pagination;
pub use dto::ApiResponse;
pub use error::CommonError;
pub use utils::{make_code, parse_code, is_valid_code, StructuredCode};
pub use utils::ErrorCode;

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
