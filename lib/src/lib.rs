pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

use bevy::prelude::*;

// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
