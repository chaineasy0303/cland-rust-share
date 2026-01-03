//! Internal shared library for common functionality across team projects.
//!
//! This library provides common utilities and abstractions used across multiple
//! team projects, including error handling, configuration management, and
//! cryptographic operations.

pub mod config;
pub mod crypto;
pub mod dto;
pub mod error;
pub mod model;
pub mod utils;

// Re-export commonly used types for convenience
pub use config::Config;
pub use crypto::CryptoError;
pub use dto::ApiResponse;
pub use error::CommonError;
pub use model::Pagination;
pub use utils::ErrorCode;
pub use utils::{StructuredCode, is_valid_code, make_code, parse_code};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
