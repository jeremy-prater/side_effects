// Player info

use bevy::{prelude::*, utils::HashSet};

#[derive(Resource)]
pub struct MushroomDatabase {
    pub mushroom_locations: HashSet<(i32, i32)>,
    pub picked_mushroom_locations: HashSet<(i32, i32)>,
}
