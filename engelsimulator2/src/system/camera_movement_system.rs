use bevy::prelude::*;

use crate::component;

/// Should make sure that the player (or maybe later other important actions)
/// are visible in the screen.
pub fn camera_movement_system(
    mut q: QuerySet<(
        Query<&mut Transform, With<component::MainCamera>>,
        Query<&GlobalTransform, With<component::Player>>,
    )>,
) {
    let (x, y) = {
        if let Ok(player_transform) = q.q1().single() {
            (
                player_transform.translation.x,
                player_transform.translation.y,
            )
        } else {
            return;
        }
    };
    if let Ok(mut camera_transform) = q.q0_mut().single_mut() {
        camera_transform.translation.x = x;
        camera_transform.translation.y = y;
    }
}
