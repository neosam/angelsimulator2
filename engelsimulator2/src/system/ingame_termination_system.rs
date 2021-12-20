use anyhow::Context;
use bevy::prelude::*;

use crate::{gamestate::GameState, resource};

pub fn ingame_termination_system(
    ingame_state: Res<resource::IngameState>,
    mut game_state: ResMut<State<GameState>>,
) -> anyhow::Result<()> {
    if ingame_state.won {
        game_state
            .set(GameState::Heaven)
            .context("Cannot switch state from ingame to heaven")?;
    }
    Ok(())
}
