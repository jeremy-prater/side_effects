use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

/// Utility system which uses Rapier3D raycasting to convert cursor position to coordinates in the
/// 3D world.
pub fn cursor_world_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(Entity, &Camera, &GlobalTransform)>,

    rapier_context: Res<RapierContext>,
) -> Option<Vec3> {
    let window = windows.single();

    let cursor_position = window.cursor_position()?;

    if let Some((_, camera, camera_transform)) = cameras.into_iter().next() {
        let ray = camera.viewport_to_world(camera_transform, cursor_position)?;

        return Some(
            rapier_context
                .cast_ray_and_get_normal(
                    ray.origin,
                    ray.direction,
                    f32::MAX,
                    true,
                    QueryFilter::only_fixed(),
                )?
                .1
                .point,
        );
    }

    None
}
