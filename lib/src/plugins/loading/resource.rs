// Player info

use bevy::prelude::*;

#[derive(Resource)]
pub struct LoadingScreen {
    pub camera: Entity,
    pub background: Entity,
}
