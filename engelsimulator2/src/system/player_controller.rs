use ::bevy::math::Vec3;
use anyhow::Context;
use bevy::prelude::*;
use heron::prelude as heron;

use crate::{component, resource};

/// Moves the player.
pub fn player_controller_system(
    mut query: Query<&mut heron::Velocity, With<component::Player>>,
    input_state: Res<resource::InputStore>,
) -> anyhow::Result<()> {
    let speed = 500.0;
    let mut player_velocity = query
        .single_mut()
        .context("More or less than one player entity was found")?;
    let mut velocity_vector = Vec3::ZERO;
    if input_state.move_up {
        velocity_vector += Vec3::Y * speed;
    }
    if input_state.move_down {
        velocity_vector -= Vec3::Y * speed;
    }
    if input_state.move_left {
        velocity_vector -= Vec3::X * speed;
    }
    if input_state.move_right {
        velocity_vector += Vec3::X * speed;
    }

    player_velocity.linear = velocity_vector;

    Ok(())
}
