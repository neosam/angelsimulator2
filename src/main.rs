use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

mod entity;
mod system;
mod resource;

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
    builder.run();
}
