//! Sanity is what usually 'live' is.  The entity counts as dead when the
//! sanity is 0 or below.
pub struct Sanity {
    pub max: f32,
    pub current: f32,
}

impl Sanity {
    pub fn new(current: f32, max: f32) -> Self {
        Sanity { current, max }
    }
}
