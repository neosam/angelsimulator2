use bevy::prelude::*;

use crate::{component, entity, resource};

pub fn ingame_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> anyhow::Result<()> {
    let sprites = resource::Sprites {
        player: materials.add(asset_server.load("sprites/player.png").into()),
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle {
        ..Default::default()
    });

    entity::PlayerEntityGenerator::new()
        .with_sprites(&sprites)
        .build(&mut commands);
    entity::SanityDrainGenerator::new()
        .with_radius(100.0)
        .with_position(-300.0, -300.0)
        .build(&mut commands);
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

    commands.insert_resource(sprites);
    Ok(())
}
