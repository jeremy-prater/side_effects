use crate::systems::terrain::spawn_terrain;
use bevy::prelude::*;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_terrain);
    }
}
