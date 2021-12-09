use bevy::{prelude::*, render::camera::OrthographicProjection};

use crate::entity;

pub fn ingame_startup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            left: -400.0,
            right: 400.0,
            top: 300.0,
            bottom: 300.0,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });

    entity::PlayerEntityGenerator::new().build(&mut commands);
}
