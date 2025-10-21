# cland-rust-share

[![Crates.io](https://img.shields.io/crates/v/cland-rust-share.svg)](https://crates.io/crates/cland-rust-share)
[![Documentation](https://docs.rs/cland-rust-share/badge.svg)](https://docs.rs/cland-rust-share)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A Rust library providing shared functionality for various use cases.

## Features

- Simple greeting functionality
- Utility struct with basic operations
- Mathematical operations module
- Well-documented and tested

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cland-rust-share = "0.1.0"
```

## Usage

```rust
use cland_rust_share::{greet, Utility, math};

fn main() {
    // Basic greeting
    println!("{}", greet("World"));
    
    // Utility struct
    let mut util = Utility::new(10);
    util.double();
    println!("Utility value: {}", util.value());
    
    // Math operations
    println!("2 + 3 = {}", math::add(2, 3));
    println!("5! = {}", math::factorial(5));
}
```

## API Documentation

### Functions

- `greet(name: &str) -> String` - Returns a greeting message

### Structs

- `Utility` - A simple utility struct
  - `new(value: i32) -> Self`
  - `value(&self) -> i32`
  - `set_value(&mut self, value: i32)`
  - `double(&mut self)`

### Modules

- `math` - Mathematical operations
  - `add(a: i32, b: i32) -> i32`
  - `multiply(a: i32, b: i32) -> i32`
  - `factorial(n: u32) -> u64`

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
