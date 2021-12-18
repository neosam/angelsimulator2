pub struct Sanity {
    pub max: f32,
    pub current: f32,
}

impl Sanity {
    pub fn new_full(max: f32) -> Self {
        Sanity { max, current: max }
    }
}
