use bevy::prelude::*;

pub fn cleanup_system(mut query: Query<Entity>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
