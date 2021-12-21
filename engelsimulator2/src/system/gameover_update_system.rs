use anyhow::Context;
use bevy::prelude::*;

use crate::{gamestate::GameState, resource};

pub fn gameover_update_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) -> anyhow::Result<()> {
    if keyboard_input.just_pressed(KeyCode::Return) {
        commands.insert_resource(resource::IngameStore::new(120.0));
        state
            .set(GameState::Heaven)
            .context("Could not set the game state from heaven to ingame")?;
    }
    Ok(())
}
