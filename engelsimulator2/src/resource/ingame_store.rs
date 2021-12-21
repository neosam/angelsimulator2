#[derive(Default)]

/// Store data which is requird by InGame state and other systems which
/// initialize from InGame.
pub struct IngameStore {
    pub won: bool,
    pub dead: bool,
    pub sanity_on_death: f32,
}

impl IngameStore {
    pub fn new(sanity: f32) -> Self {
        IngameStore {
            won: false,
            dead: false,
            sanity_on_death: sanity,
        }
    }
}
