use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

mod component;
mod entity;
mod event;
mod gamestate;
mod resource;
mod system;

use gamestate::GameState;

fn main() {
    let mut builder = App::build();
    // Plugin setup
    builder.add_plugins(DefaultPlugins);
    builder.add_plugin(ShapePlugin);
    builder.add_plugin(PhysicsPlugin::default());
    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin);

    // Set up resources
    builder.insert_resource(resource::IngameState::new(120.0));
    builder.insert_resource(resource::HeavenState {
        player_sanity: 120.0,
    });

    // Set up events
    builder.add_event::<event::collision_events::SanityDrainEvent>();
    builder.add_event::<event::InitializeEvent>();

    // Set up state
    builder.add_state(GameState::Heaven);


    // Set up InGame Systems
    builder.add_system_set(
        SystemSet::on_enter(GameState::Ingame).with_system(
            system::startup::ingame_startup_system
                .system()
                .chain(system::handle_error_system.system()),
        ),
    );
    builder.add_system_set(
        SystemSet::on_update(GameState::Ingame)
            .with_system(system::input_system.system())
            .with_system(system::camera_movement_system.system())
            .with_system(system::collision_handler_system.system())
            .with_system(system::sanity_drain_system.system())
            .with_system(
                system::ui_update_system
                    .system()
                    .chain(system::handle_error_system.system()),
            )
            .with_system(
                system::player_controller_system
                    .system()
                    .chain(system::handle_error_system.system()),
            )
            .with_system(
                system::ingame_termination_system
                    .system()
                    .chain(system::handle_error_system.system()),
            ),
    );
    builder.add_system_set(
        SystemSet::on_exit(GameState::Ingame).with_system(system::cleanup_system.system()),
    );


    // Set up Heaven systems
    builder.add_system_set(
        SystemSet::on_enter(GameState::Heaven)
            .with_system(system::startup::heaven_startup_system.system()),
    );
    builder.add_system_set(
        SystemSet::on_update(GameState::Heaven).with_system(
            system::heaven_update_system
                .system()
                .chain(system::handle_error_system.system()),
        ),
    );
    builder.add_system_set(
        SystemSet::on_exit(GameState::Heaven).with_system(system::cleanup_system.system()),
    );


    // Set up GameOver systems
    builder.add_system_set(
        SystemSet::on_enter(GameState::GameOver)
            .with_system(system::startup::gameover_startup_system.system()),
    );
    builder.add_system_set(
        SystemSet::on_update(GameState::GameOver).with_system(
            system::gameover_update_system
                .system()
                .chain(system::handle_error_system.system()),
        ),
    );
    builder.add_system_set(
        SystemSet::on_exit(GameState::GameOver).with_system(system::cleanup_system.system()),
    );

    builder.run();
}
