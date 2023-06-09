pub mod component;
pub mod resource;
pub mod system;

use self::component::Terrain;
use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_terrain.in_schedule(OnEnter(GameState::InGame)))
            .add_system(spawn_terrain.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::despawn_with::<Terrain>.in_schedule(OnExit(GameState::InGame)));
    }
}
