use crate::components::camera::MainCamera;
use crate::components::player::Player;
use bevy::prelude::*;
use kayak_ui::CameraUIKayak;

pub fn spawn_main_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::splat(10.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            CameraUIKayak,
        ))
        .insert(MainCamera::default());
}
pub fn rotate_camera(
    mut camera_query: Query<&mut MainCamera>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    if input.pressed(KeyCode::Q) {
        camera.rotation -= 90.0 * time.delta_seconds();
    }

    if input.pressed(KeyCode::E) {
        camera.rotation += 90.0 * time.delta_seconds();
    }

    if camera.rotation > 360.0 {
        camera.rotation -= 360.0;
    }

    if camera.rotation < 0.0 {
        camera.rotation += 360.0;
    }
}
pub fn update_camera_desired_position(
    mut camera_query: Query<&mut MainCamera>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera = camera_query.single_mut();
    let player_transform = player_query.single();

    let mut origin_transform = *player_transform;
    origin_transform.rotation = Quat::default();
    origin_transform.rotate_y(camera.rotation.to_radians());

    let direction = origin_transform.forward().normalize_or_zero();
    camera.look_target = player_transform.translation;

    let desired_position =
        origin_transform.translation + (direction * camera.offset.z) + (Vec3::Y * camera.offset.y);

    camera.desired_position = desired_position;
}

pub fn lerp_to_desired_position(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &MainCamera)>,
) {
    let (mut transform, camera) = camera_query.single_mut();

    let lerped_position = transform.translation.lerp(
        camera.desired_position,
        time.delta_seconds() * camera.easing,
    );

    transform.translation = lerped_position;
    transform.look_at(camera.look_target, Vec3::Y);
}
