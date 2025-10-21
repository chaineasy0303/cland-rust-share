//! Example binary that demonstrates usage of the cland-rust-share library

use cland_rust_share::{greet, Utility, math};

fn main() {
    println!("=== cland-rust-share Demo ===");
    
    // Demonstrate greeting functionality
    println!("\n1. Greeting:");
    println!("{}", greet("Rust Developer"));
    
    // Demonstrate utility struct
    println!("\n2. Utility Struct:");
    let mut util = Utility::new(5);
    println!("Initial value: {}", util.value());
    util.double();
    println!("After doubling: {}", util.value());
    util.set_value(100);
    println!("After setting to 100: {}", util.value());
    
    // Demonstrate math operations
    println!("\n3. Math Operations:");
    println!("2 + 3 = {}", math::add(2, 3));
    println!("4 * 5 = {}", math::multiply(4, 5));
    println!("Factorial of 6 = {}", math::factorial(6));
    
    println!("\n=== Demo Complete ===");
}
