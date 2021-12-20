use bevy::prelude::*;
use anyhow::Context;

use crate::{component, entity, resource};

pub fn ingame_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> anyhow::Result<()> {
    let sprites = resource::Sprites {
        player: materials.add(asset_server.load("sprites/player.png").into()),
    };

    let level_content = include_str!("../../../resources/levels/assembly-hall-1.svg");
    let level = level_parser::parse_level_from_svg(level_content)
        .context("Could not load level")?;
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(component::MainCamera);
    commands.spawn_bundle(UiCameraBundle {
        ..Default::default()
    });


    let level_scale_factor = 1.0;
    let player_position = *level.spawns.get("player")
        .context("Could not find player positions in level")?
        .get(0)
        .context("Player positions in level are empty")?;
    entity::PlayerEntityGenerator::new()
        .with_sprites(&sprites)
        .with_position((player_position.0 * level_scale_factor, -player_position.1 * level_scale_factor))
        .build(&mut commands);

    entity::SanityDrainGenerator::new()
        .with_radius(100.0)
        .with_position(-300.0, -300.0)
        .build(&mut commands);
    
    let level_texture = asset_server.load("levels/assembly-hall-1.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(level_texture.into()),
        transform: Transform::from_xyz(level.size.0 / 2.0, level.size.1 / -2.0, 0.0),
        ..Default::default()
    });

    for barrier in level.barrier.iter() {
        match *barrier {
            level_parser::Barrier::Circle(x, y, radius) =>
                entity::circle_barrier(&mut commands, (x * level_scale_factor, -y * level_scale_factor, radius * level_scale_factor)),
            level_parser::Barrier::Rect(x, y, width, height, rotation) =>
                entity::rect_barrier(&mut commands, (x * level_scale_factor, -y * level_scale_factor - height, width * level_scale_factor, height * level_scale_factor, rotation)),
        }
    }

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
