//! Entity which reduces senity form other entities when colliding.

pub struct SanityDrain {
    pub strength: f32,
}

impl SanityDrain {
    pub fn with_strength(strength: f32) -> Self {
        SanityDrain { strength }
    }
}
