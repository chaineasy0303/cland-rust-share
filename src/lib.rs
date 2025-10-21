//! cland-rust-share
//!
//! A Rust library providing shared functionality for various use cases.
//!
//! # Examples
//!
//! ```
//! use cland_rust_share::greet;
//!
//! let message = greet("World");
//! assert_eq!(message, "Hello, World!");
//! ```

/// Greets the given name
///
/// # Examples
///
/// ```
/// use cland_rust_share::greet;
///
/// assert_eq!(greet("Alice"), "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// A simple utility struct for demonstration
#[derive(Debug, Clone, PartialEq)]
pub struct Utility {
    value: i32,
}

impl Utility {
    /// Creates a new Utility instance
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Gets the current value
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Sets a new value
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    /// Doubles the current value
    pub fn double(&mut self) {
        self.value *= 2;
    }
}

/// Mathematical operations
pub mod math {
    /// Adds two numbers
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiplies two numbers
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Calculates factorial of a number
    pub fn factorial(n: u32) -> u64 {
        match n {
            0 | 1 => 1,
            _ => (2..=n as u64).product(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_utility() {
        let mut util = Utility::new(10);
        assert_eq!(util.value(), 10);
        
        util.set_value(20);
        assert_eq!(util.value(), 20);
        
        util.double();
        assert_eq!(util.value(), 40);
    }

    #[test]
    fn test_math_operations() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::multiply(4, 5), 20);
        assert_eq!(math::factorial(5), 120);
    }
}
