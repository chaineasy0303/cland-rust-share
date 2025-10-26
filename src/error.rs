use thiserror::Error;

/// Unified error type used across the crate
#[derive(Debug, Error)]
pub enum CommonError {
    #[error("config error: {0}")]
    Config(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("other: {0}")]
    Other(String),
}

impl CommonError {
    pub fn config(msg: impl Into<String>) -> Self {
        CommonError::Config(msg.into())
    }

    pub fn crypto(msg: impl Into<String>) -> Self {
        CommonError::Crypto(msg.into())
    }
}

impl From<std::string::String> for CommonError {
    fn from(s: String) -> Self {
        CommonError::Other(s)
    }
}

impl From<crate::crypto::CryptoError> for CommonError {
    fn from(err: crate::crypto::CryptoError) -> Self {
        CommonError::Crypto(err.to_string())
    }
}
