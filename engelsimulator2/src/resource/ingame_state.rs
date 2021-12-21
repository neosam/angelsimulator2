#[derive(Default)]
pub struct IngameState {
    pub won: bool,
    pub dead: bool,
    pub sanity_on_death: f32,
}

impl IngameState {
    pub fn new(sanity: f32) -> Self {
        IngameState {
            won: false,
            dead: false,
            sanity_on_death: sanity,
        }
    }
}