//! Common utility functions module.
//!
//! This module provides various utility functions that are commonly
//! used across different projects.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::CommonError;

/// String utilities
pub mod string {
    // use super::*;

    /// Check if a string is empty or contains only whitespace
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// Convert string to snake_case
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        let mut prev_was_upper = false;
        
        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                // Add underscore if:
                // 1. Not the first character AND
                // 2. Previous character was lowercase OR
                // 3. Next character is lowercase (handles HTTPRequest -> http_request)
                if !result.is_empty() && (prev_was_upper == false || chars.peek().map_or(false, |&next| next.is_lowercase())) {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
                prev_was_upper = true;
            } else {
                result.push(c);
                prev_was_upper = false;
            }
        }
        
        result
    }

    /// Truncate string with ellipsis
    pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            return s.to_string();
        }

        if max_len <= 3 {
            return "...".to_string();
        }

        format!("{}...", &s[..max_len - 3])
    }
}

/// Date and time utilities
pub mod datetime {
    // use super::*;

    /// Get current timestamp in seconds
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Get current timestamp in milliseconds
    pub fn current_timestamp_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    /// Format duration as human readable string
    pub fn format_duration(seconds: u64) -> String {
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            format!("{}m {}s", seconds / 60, seconds % 60)
        } else if seconds < 86400 {
            format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
        } else {
            format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
        }
    }
}

/// File system utilities
pub mod fs {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// Read file content as string
    pub fn read_file(path: &str) -> Result<String, CommonError> {
        fs::read_to_string(path).map_err(CommonError::from)
    }

    /// Write string content to file
    pub fn write_file(path: &str, content: &str) -> Result<(), CommonError> {
        fs::write(path, content).map_err(CommonError::from)
    }

    /// Check if file exists
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Create directory if it doesn't exist
    pub fn create_dir_if_not_exists(path: &str) -> Result<(), CommonError> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path).map_err(CommonError::from)
        } else {
            Ok(())
        }
    }
}

/// Validation utilities
pub mod validation {
    use super::*;

    /// Validate email format
    pub fn is_valid_email(email: &str) -> bool {
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    /// Validate URL format
    pub fn is_valid_url(url: &str) -> bool {
        let re = regex::Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        re.is_match(url)
    }

    /// Validate phone number format (simple check)
    pub fn is_valid_phone(phone: &str) -> bool {
        let re = regex::Regex::new(r"^\+?[0-9\s\-()]{10,}$").unwrap();
        re.is_match(phone)
    }
}

/// Collection utilities
pub mod collection {
    use std::collections::HashMap;

    /// Convert vector to HashMap with key extraction
    pub fn vec_to_hashmap<T, K, V, F>(vec: Vec<T>, key_extractor: F) -> HashMap<K, V>
    where
        K: std::hash::Hash + Eq,
        F: Fn(&T) -> (K, V),
    {
        vec.into_iter().map(|item| key_extractor(&item)).collect()
    }

    /// Check if all elements in vector are unique
    pub fn all_unique<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
        let mut seen = std::collections::HashSet::new();
        for item in vec {
            if !seen.insert(item) {
                return false;
            }
        }
        true
    }
}

// Re-export commonly used utilities
pub use string::{is_blank, to_snake_case, truncate_with_ellipsis};
pub use datetime::{current_timestamp, current_timestamp_millis, format_duration};
pub use fs::{create_dir_if_not_exists, file_exists, read_file, write_file};
pub use validation::{is_valid_email, is_valid_phone, is_valid_url};
