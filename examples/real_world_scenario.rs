//! Real-world example showing practical usage patterns
//!
//! Run with: `cargo run --example real_world_scenario`
//! Debug mode: `RUST_LOG=debug cargo run --example real_world_scenario`

use debug_error::{DebugError, debug_error, debug_error_with_log};
use log::{debug, error, info, warn};

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

struct Database;

impl Database {
    fn connect() -> Result<Self, DebugError> {
        // Simulate connection failure
        Err(debug_error_with_log!("Database connection timeout"))?
    }
    
    fn get_user(&self, id: u64) -> Result<User, DebugError> {
        if id == 0 {
            Err(debug_error!("User not found with ID: {}", id))?
        } else {
            Ok(User { id, name: "Alice".to_string() })
        }
    }
}

fn main() -> Result<(), DebugError> {
    env_logger::init();
    
    info!("Starting real-world example");
    
    // Example 1: Database operations with error propagation
    let _ = Database::connect()
        .map_err(|e| { warn!("Example 1 failed: {}", e); e });
    
    // Example 2: Using if let operator with custom context
    let db = Database;

    if let Err(err) = db.get_user(123)
        .map_err(|e| { debug_error!("Example 2 failed: {}", e); e }) {
        error!("{}", err);
    }

    // Example 3: use different log level        
    let _ = db.get_user(123)
        .map_err(|e| { warn!("Example 3 failed: {}", e); e });
    
    
    // Example 4: Handling specific error cases
    match db.get_user(0) {
        Ok(user) => info!("Found user: {:?}", user),
        Err(e) => warn!("Example 4 error occurred: {}", e),
    }
    
    // Example 5: Chaining operations with error context
    let _ = process_user_data(&db, 123)
        .map_err(|e| { warn!("Example 5 failed: {}", e); e });
    
    info!("Real-world example completed (some examples may have failed as expected)");

    info!("The next line will let the program fail due to use of ? in main while an error occurs\n
        if used in an lower level function the error would be propagated.");
        
    let _db = db.get_user(0)
        .map_err(|e| debug_error!("Examples successfully finished: {}", e))?;

    Ok(())
}

fn process_user_data(db: &Database, user_id: u64) -> Result<(), DebugError> {
    info!("Processing data for user {}", user_id);
    
    let user = db.get_user(user_id)
        .map_err(|e| debug_error!("Error in process_user_data for ID {}: {}", user_id, e))?;
    
    // Simulate processing that might fail
    if user.name.is_empty() {
        Err(debug_error_with_log!("User {} has empty name", user_id))?
    }
    
    debug!("Successfully processed user: {} with id: {}", user.name, user.id);
    Ok(())
}