use crate::states::game_state::GameState;
use crate::systems::terrain::spawn_terrain;
use bevy::prelude::*;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_terrain.in_schedule(OnEnter(GameState::InGame)));
    }
}
