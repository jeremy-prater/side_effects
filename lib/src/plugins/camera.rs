use crate::states::game_state::GameState;
use crate::systems::camera::{
    lerp_to_desired_position,
    spawn_main_camera,
    zoom_camera,
    rotate_camera,
    update_camera,
};
use bevy::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_camera.in_schedule(OnEnter(GameState::InGame)))
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
