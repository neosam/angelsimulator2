use bevy::{prelude::*, render::camera::OrthographicProjection};

use crate::{component, entity, resource};

pub fn ingame_startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    commands.spawn_bundle(UiCameraBundle {
        ..Default::default()
    });

    entity::PlayerEntityGenerator::new().build(&mut commands);
    commands.insert_resource(resource::InputState::default());

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Santiy:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
                Default::default(),
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(component::UiSanity);
}
