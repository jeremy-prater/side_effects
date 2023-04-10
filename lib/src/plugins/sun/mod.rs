pub mod component;
pub mod resource;
pub mod system;

use super::GameState;
use bevy::prelude::*;
use component::Sun;
use system::*;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_sun.in_schedule(OnEnter(GameState::InGame)))
            .add_system(update_sun.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::despawn_with::<Sun>.in_schedule(OnExit(GameState::InGame)));
    }
}
