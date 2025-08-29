//! Basic usage example showing both macros
//!
//! Run with: `cargo run --example basic_usage`
//! Set log level: `RUST_LOG=info cargo run --example basic_usage`
//! 
//! For transforming errors user .map_err(|e| debug_error!("your_message: {}", e))?;
//! See more in examples/real_world_scenario.rs

use debug_error::{DebugError, debug_error, debug_error_with_log};
use log::{info, error};

fn main() {
    // Initialize logger
    env_logger::init();
    
    info!("Starting basic usage example");
    
    // Test debug_error (no automatic logging)
    match test_debug_error() {
        Ok(_) => info!("debug_error test passed"),
        Err(e) => error!("debug_error failed: {}", e),
    }
    
    // Test debug_error_with_log (automatic logging)
    match test_debug_error_with_log() {
        Ok(_) => info!("debug_error_with_log test passed"),
        Err(e) => error!("debug_error_with_log failed: {}", e),
    }
    
    info!("Example completed");
}

fn test_debug_error() -> Result<(), DebugError> {
    info!("Testing debug_error macro...");
    
    // This creates an error but doesn't log it automatically
    let err = debug_error!("This is a test error without automatic logging");
    // This line logs the error
    error!("Created error: {}", err);
    
    Ok(())
}

fn test_debug_error_with_log() -> Result<(), DebugError> {
    info!("Testing debug_error_with_log macro...");
    
    // This creates an error AND logs it automatically
    debug_error_with_log!("This error is automatically logged");
    Ok(())
}