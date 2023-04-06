use bevy::{prelude::*, utils::HashSet};

#[derive(Resource)]
pub struct TreeDatabase {
    pub tree_locations: HashSet<(i32, i32)>,
}
