use bevy::prelude::*;

pub fn handle_error_system(In(result): In<anyhow::Result<()>>) {
    if let Err(error) = result {
        println!("Error: {}", error);
    }
}
