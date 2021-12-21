#[derive(Default)]

/// Holds the current interpreted input data.  A system already checked the
/// input devices like keyboard, joystick and mouse and writes to this Store.
pub struct InputStore {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
}
