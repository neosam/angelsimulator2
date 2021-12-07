use bevy::prelude::*;

fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin);
    builder.run();
}
