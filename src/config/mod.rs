//! Configuration management module.
//!
//! This module provides utilities for loading and managing application
//! configuration from various sources (environment variables, files, etc.).

use std::collections::HashMap;
use std::error::Error as StdError;

type DynError = Box<dyn StdError + Send + Sync + 'static>;

/// Main configuration structure
#[derive(Debug, Clone)]
pub struct Config {
    /// Application settings
    pub app: AppConfig,
    /// Database settings
    pub database: DatabaseConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Custom key-value pairs
    pub custom: HashMap<String, String>,
}

/// Application-specific configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Debug mode flag
    pub debug: bool,
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

/// Encryption configuration
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: Config {
                app: AppConfig {
                    name: String::new(),
                    version: String::new(),
                    environment: "development".to_string(),
                    debug: false,
                },
                database: DatabaseConfig {
                    url: String::new(),
                    max_connections: 10,
                    timeout_seconds: 30,
                },
                encryption: EncryptionConfig {
                    key: String::new(),
                    algorithm: "aes-256-gcm".to_string(),
                },
                custom: HashMap::new(),
            },
        }
    }

    /// Set application name
    pub fn with_app_name(mut self, name: impl Into<String>) -> Self {
        self.config.app.name = name.into();
        self
    }

    /// Set application version
    pub fn with_app_version(mut self, version: impl Into<String>) -> Self {
        self.config.app.version = version.into();
        self
    }

    /// Set environment
    pub fn with_environment(mut self, environment: impl Into<String>) -> Self {
        self.config.app.environment = environment.into();
        self
    }

    /// Set debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.config.app.debug = debug;
        self
    }

    /// Set database URL
    pub fn with_database_url(mut self, url: impl Into<String>) -> Self {
        self.config.database.url = url.into();
        self
    }

    /// Set maximum database connections
    pub fn with_max_connections(mut self, max_connections: u32) -> Self {
        self.config.database.max_connections = max_connections;
        self
    }

    /// Set database timeout
    pub fn with_database_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.database.timeout_seconds = timeout_seconds;
        self
    }

    /// Set encryption key
    pub fn with_encryption_key(mut self, key: impl Into<String>) -> Self {
        self.config.encryption.key = key.into();
        self
    }

    /// Set encryption algorithm
    pub fn with_encryption_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.config.encryption.algorithm = algorithm.into();
        self
    }

    /// Add custom configuration value
    pub fn with_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.custom.insert(key.into(), value.into());
        self
    }

    /// Build the final configuration
    pub fn build(self) -> Config {
        self.config
    }
}

/// Encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Encryption key
    pub key: String,
    /// Encryption algorithm
    pub algorithm: String,
}

impl Config {
    /// Create a new configuration builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, DynError> {
        let mut builder = ConfigBuilder::new();

        if let Ok(name) = std::env::var("APP_NAME") {
            builder = builder.with_app_name(name);
        }

        if let Ok(version) = std::env::var("APP_VERSION") {
            builder = builder.with_app_version(version);
        }

        if let Ok(env) = std::env::var("APP_ENV") {
            builder = builder.with_environment(env);
        }

        if let Ok(debug) = std::env::var("APP_DEBUG") {
            builder = builder.with_debug(debug.to_lowercase() == "true");
        }

        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            builder = builder.with_database_url(db_url);
        }

        if let Ok(enc_key) = std::env::var("ENCRYPTION_KEY") {
            builder = builder.with_encryption_key(enc_key);
        }

        Ok(builder.build())
    }

    /// Load default configuration
    pub fn load_default() -> Result<Self, DynError> {
        Self::from_env()
    }
}

impl Default for Config {
    fn default() -> Self {
        ConfigBuilder::new().build()
    }
}
