use crate::components::{mushroom::Mushroom, camera::MainCamera};
use bevy::prelude::*;



pub fn init_mushrooms(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
    camera_query: Query<&MainCamera>,
) {
    let _origin = camera_query.get_single().unwrap().offset;
    // asset_server.l
}

pub fn spawn_mushroom(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
    camera_query: Query<&MainCamera>,
    _query: Query<&Mushroom>,
) {
    let _origin = camera_query.get_single().unwrap().offset;
}
