use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

mod component;
mod entity;
mod resource;
mod system;

fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);
    builder.add_plugin(ShapePlugin);
    builder.add_plugin(PhysicsPlugin::default());
    //builder.add_plugin(bevy_svg::prelude::SvgPlugin);
    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin);
    builder.add_startup_system(system::startup::ingame_startup_system.system());
    builder.add_system(system::input_system.system());
    builder.add_system(
        system::player_controller_system
            .system()
            .chain(system::handle_error_system.system()),
    );
    builder.add_system(
        system::ui_update_system
            .system()
            .chain(system::handle_error_system.system()),
    );

    builder.run();
}
