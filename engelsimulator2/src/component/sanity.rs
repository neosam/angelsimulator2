pub struct Sanity {
    pub max: f32,
    pub current: f32,
}

impl Sanity {
    pub fn new(current: f32, max: f32) -> Self {
        Sanity { current, max }
    }
}
