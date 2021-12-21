use bevy::prelude::*;

use crate::gamestate::GameState;
use anyhow::Context;

pub fn heaven_update_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) -> anyhow::Result<()> {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state
            .set(GameState::Ingame)
            .context("Could not set the game state from heaven to ingame")?;
    }
    Ok(())
}
