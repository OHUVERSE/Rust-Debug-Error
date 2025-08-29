
//! A lightweight error handling library with automatic line number tracking and logging
//!
//! `DebugError` provides macros for creating errors that automatically capture
//! the file and line number of created errors, making debugging much easier.
//!
//! # Features
//! - Automatically captures error location (file + line)
//! - Optional automatic logging when errors occur
//! - Seamless integration with Rust's `?` operator
//! - Compatible with any logger that implements the `log` crate trait
//! - Supports text and variables like the 'format!' macro
//! 
//! # Quick Start
//!
//! ```rust
//! use debug_error::{DebugError, debug_error, debug_error_with_log};
//! use log::info;
//!
//! fn main() -> Result<(), DebugError> {
//!     // Initialize your logger (env_logger, pretty_env_logger, etc.)
//!     env_logger::init();
//!     
//!     info!("Starting application");
//!     
//!     // This will return an error with location information
//!     let result = might_fail()?;
//!     
//!     Ok(())
//! }
//!
//! fn might_fail() -> Result<(), DebugError> {
//!     if some_condition() {
//!         // Automatically logs the error and includes location
//!         Err(debug_error_with_log!("Operation failed due to condition"))?
//!     }
//!     
//!     Ok(())
//! }
//!
//! fn some_condition() -> bool {
//!     true
//! }
//! ```
#[derive(Debug, Clone)]
pub struct DebugError 
{
    pub message: String,
    pub location: &'static std::panic::Location<'static>,
}

/// An error type that captures the location where it was created
///
/// `DebugError` automatically grabs the file and line number where it was created,
/// making debugging much easier by showing exactly where errors originate.
impl DebugError 
{
    pub fn new(message: String, location: &'static std::panic::Location<'static>) -> Self 
    {
        Self { message, location }
    }
}

impl std::fmt::Display for DebugError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{} at {}", self.message, self.location)
    }
}

impl ::std::error::Error for DebugError {}

/// Creates a DebugError and automatically logs it
///
/// This macro creates a `DebugError` and immediately logs it at the error level.
/// It's perfect for rapid development when you want immediate visibility into errors.
/// During development you don't need to match on every low-level function.
/// Just use this macro and you get the location immediately, when you are about to finish you can set
/// up all your error handling and replace this macro with the 'debug_error' macro to suppress immediate logging.
///
/// # When to Use the macro
/// - During initial development and prototyping
/// - For quick debugging sessions
/// - When you want immediate error visibility
///
/// # When NOT to Use (and use 'debug_error' macro)
/// - Production code (too verbose)
/// - When you have proper error handling infrastructure set up already
///
/// # Examples
///
/// ```rust
/// use debug_error::{debug_error_with_log, DebugError};
///
/// fn connect_to_database() -> Result<(), DebugError> {
///     // Simulate a connection failure
///     Err(debug_error_with_log!("Database connection timeout after 30s"))
/// }
/// ```
#[macro_export]
macro_rules! debug_error_with_log
{
    ($($arg:tt)*) => {{
        let message = format!($($arg)*); // Format the message
        let err = DebugError::new(message, std::panic::Location::caller());
        // Log the error with the location
        ::log::error!("Error: {} at {}:{}:{}", err.message, err.location.file(), err.location.line(), err.location.column());
        err
    }};
}

/// Creates a DebugError without automatic logging
///
/// Use this macro when you want the location tracking benefits of DebugError
/// but prefer to handle logging yourself or don't want automatic logging.
///
/// # When to Use
/// - Production code where you control logging
/// - When you want to add custom context before logging
/// - When using custom error handling middleware
/// - When you have proper error handling infrastructure set up
///
/// # Examples
///
/// ```rust
/// use debug_error::{debug_error, DebugError};
/// use log::warn;
///
/// fn validate_input(input: &str) -> Result<(), DebugError> {
///     if input.is_empty() {
///         let err = debug_error!("Input cannot be empty");
///         warn!("Validation warning: {}", err);
///         return Err(err);
///     }
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! debug_error
{
    ($($arg:tt)*) => {{
        let message = format!($($arg)*); // Format the message
        let err = DebugError::new(message, std::panic::Location::caller());
        err
    }};
}