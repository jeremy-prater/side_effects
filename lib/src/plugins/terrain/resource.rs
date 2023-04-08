use bevy::{prelude::*, utils::HashSet};

#[derive(Resource)]
pub struct TerrainDatabase {
    pub terrain_locations: HashSet<(i32, i32)>,
}
