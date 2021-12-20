use bevy::prelude::*;
use anyhow::Context;

use crate::{resource, gamestate::GameState};

pub fn ingame_termination_system(ingame_state: Res<resource::IngameState>, mut game_state: ResMut<State<GameState>>) -> anyhow::Result<()> {
    if ingame_state.won {
        game_state.set(GameState::Heaven).context("Cannot switch state from ingame to heaven")?;
    }
    Ok(())
}