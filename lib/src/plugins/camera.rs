use crate::systems::camera::{
    lerp_to_desired_position, rotate_camera, spawn_main_camera, update_camera_desired_position,
};
use bevy::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_main_camera).add_systems(
            (
                rotate_camera,
                update_camera_desired_position,
                lerp_to_desired_position,
            )
                .chain(),
        );
    }
}
