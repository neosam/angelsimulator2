use bevy::prelude::*;

/// Is chained to systems which return an errors.
pub fn handle_error_system(In(result): In<anyhow::Result<()>>) {
    if let Err(error) = result {
        println!("Error: {}", error);
    }
}
