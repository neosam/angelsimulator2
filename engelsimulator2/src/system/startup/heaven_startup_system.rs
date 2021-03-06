use bevy::prelude::*;

use crate::event;
use crate::resource;

pub fn heaven_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut heaven_state: ResMut<resource::HeavenStore>,
    ingame_state: Res<resource::IngameStore>,
    mut initialize_events: EventWriter<event::InitializeEvent>,
) {
    heaven_state.player_sanity = ingame_state.sanity_on_death;

    commands.spawn_bundle(UiCameraBundle {
        ..Default::default()
    });

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "welcome to heaven",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Percent(40.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "press space to start",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Percent(40.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    initialize_events.send(event::InitializeEvent);
}
