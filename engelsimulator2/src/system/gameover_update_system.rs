use bevy::prelude::*;
use anyhow::Context;

use crate::{resource, gamestate::GameState};

pub fn gameover_update_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) -> anyhow::Result<()> {
    if keyboard_input.just_pressed(KeyCode::Return) {
        commands.insert_resource(resource::IngameState::new(120.0));
        state
            .set(GameState::Heaven)
            .context("Could not set the game state from heaven to ingame")?;
    }
    Ok(())
}
