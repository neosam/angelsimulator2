use anyhow::Context;
use bevy::prelude::*;

use crate::{component, gamestate::GameState, resource};

pub fn ingame_termination_system(
    mut ingame_state: ResMut<resource::IngameStore>,
    mut game_state: ResMut<State<GameState>>,
    query: Query<&component::Sanity, With<component::Player>>,
) -> anyhow::Result<()> {
    let sanity = query
        .single()
        .context("Could not find player with Sanity")?;
    if ingame_state.won {
        ingame_state.sanity_on_death = sanity.current;
        game_state
            .set(GameState::Heaven)
            .context("Cannot switch state from ingame to heaven")?;
    }
    if sanity.current <= 0.0 {
        game_state
            .set(GameState::GameOver)
            .context("Cannot switch state from InGame to GameOver")?;        
    }
    Ok(())
}
