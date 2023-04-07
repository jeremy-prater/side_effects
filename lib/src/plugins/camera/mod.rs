pub mod component;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use system::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
            .add_plugin(AtmospherePlugin)
            .add_system(spawn_main_camera.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (
                    zoom_camera,
                    rotate_camera,
                    update_camera,
                    lerp_to_desired_position,
                )
                    .chain()
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}
