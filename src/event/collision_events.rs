use bevy::prelude::*;

/// When an `Entity` is draining the sanity of another `Entity`
///
/// The first argument is the causer / the drainer and the second
/// one is the victim / the one who get's drained.
pub enum CollisionState {
    Started,
    Stopped,
}
pub struct SanityDrainEvent(pub Entity, pub Entity, pub CollisionState);
