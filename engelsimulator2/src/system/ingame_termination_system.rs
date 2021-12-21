use anyhow::Context;
use bevy::prelude::*;

use crate::{component, gamestate::GameState, resource};

pub fn ingame_termination_system(
    mut ingame_state: ResMut<resource::IngameState>,
    mut game_state: ResMut<State<GameState>>,
    query: Query<&component::Sanity, With<component::Player>>,
) -> anyhow::Result<()> {
    if ingame_state.won {
        let sanity = query
            .single()
            .context("Could not find player with Sanity")?;
        ingame_state.sanity_on_death = sanity.current;
        game_state
            .set(GameState::Heaven)
            .context("Cannot switch state from ingame to heaven")?;
    }
    Ok(())
}
