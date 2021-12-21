use bevy::prelude::*;

use crate::resource;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<resource::InputStore>,
    mut ingame_state: ResMut<resource::IngameStore>,
) {
    state.move_up = keyboard_input.pressed(KeyCode::Up);
    state.move_down = keyboard_input.pressed(KeyCode::Down);
    state.move_left = keyboard_input.pressed(KeyCode::Left);
    state.move_right = keyboard_input.pressed(KeyCode::Right);
    ingame_state.won = keyboard_input.pressed(KeyCode::Return);
}
