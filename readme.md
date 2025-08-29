# debug_error

[![Crates.io](https://img.shields.io/crates/v/debug_error.svg)](https://crates.io/crates/debug_error)
[![Docs.rs](https://docs.rs/debug_error/badge.svg)](https://docs.rs/debug_error)
[![GitHub Repo](https://img.shields.io/badge/github-repository-blue?logo=github)](https://github.com/OHUVERSE/Rust-Debug-Error)

A lightweight error handling library for Rust that automatically captures file and line information, making debugging significantly easier.

## Features

- 📍 **Automatic Location Tracking** - Errors automatically capture file and line numbers
- 📝 **Flexible Logging** - Choose between automatic logging or manual control
- 🔧 **Seamless Integration** - Works perfectly with Rust's `?` operator
- 🚀 **High compatibility** - Compatible with any logger that implements the `log` crate interface
- 📦 **Lightweight** - Minimal performance overhead

## Example output
```rust
[2025-08-28T13:54:40Z ERROR basic_usage] Created error: This is a test error without automatic logging at examples\basic_usage.rs:37:15
[2025-08-28T13:58:16Z ERROR basic_usage] Error: This error is automatically logged at examples\basic_usage.rs:48:5
```

## Example usage
```rust
use debug_error::{DebugError, debug_error, debug_error_with_log};
use log::info;

fn main() -> Result<(), DebugError> {
    // Initialize your logger (env_logger, pretty_env_logger, etc.)
    env_logger::init();
    
    info!("Starting application");
    
    // This will return an error with location information
    let result = might_fail()?;
    
    Ok(())
}

fn might_fail() -> Result<(), DebugError> {
    if some_condition() {
        // Automatically logs the error and includes location
        Err(debug_error_with_log!("Operation failed due to condition"))?
    }
    
    Ok(())
}

fn some_condition() -> bool {
    true
}
```

## Notes

-  More Examples:
   -  examples/basic_usage.rs
   -  examples/real_world_scenario.rs
-  How to use:
   -  Use the "debug_error_with_log" macro during development - no need for error handling infrastructure yet.
   -  Replace "debug_error_with_log" macro with "debug_error" - for production and when error handling infrastructure is set up.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
debug_error = "0.1"
log = "0.4"  # Required for logging functionality
```