use super::component::*;
use crate::plugins::player::component::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

#[cfg(not(target_arch = "wasm32"))]
use bevy_atmosphere::prelude::*;

pub fn spawn_main_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle::default(),
            #[cfg(not(target_arch = "wasm32"))]
            AtmosphereCamera::default(),
        ))
        .insert(MainCamera::new(None, 10.0, 0.05 * PI, -PI / 2.0, 10.0));
}

pub fn zoom_camera(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    // mouse_wheel_events: Res<Events<MouseWheel>>,
    mut camera_query: Query<&mut MainCamera>,
) {
    let mut zoom = 0.0;
    for event in mouse_wheel_event_reader.iter() {
        zoom += event.y;
    }
    camera_query.get_single_mut().unwrap().add_distance(-zoom);
}

pub fn rotate_camera(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut MainCamera>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single_mut() else {
        return;
    };

    if mouse_button_input.pressed(MouseButton::Middle) || key_input.pressed(KeyCode::LShift) {
        let mut yaw = 0.0;
        let mut pitch = 0.0;
        for event in mouse_motion_event_reader.iter() {
            let (delta_yaw, delta_pitch) = event.delta.into();
            yaw += delta_yaw;
            pitch += delta_pitch;
        }
        let yaw = -yaw * 2.0 * PI / window.width();
        let pitch = pitch * PI / window.height();
        let mut orbit_camera = camera_query.get_single_mut().unwrap();
        orbit_camera.add_yaw(yaw);
        orbit_camera.add_pitch(pitch);
    }
}

pub fn update_camera(
    mut camera_query: Query<&mut MainCamera>,
    target_query: Query<(Entity, &MainCameraTarget, &Transform)>,
    player_query: Query<Entity, &Player>,
) {
    let mut orbit_camera = camera_query.get_single_mut().unwrap();
    if let Ok(player) = player_query.get_single() {
        orbit_camera.set_target(Some(player));
    }
    if let Some(target_entity) = orbit_camera.target {
        if let Ok(target_transform) = target_query.get(target_entity) {
            orbit_camera.focus = target_transform.2.translation;
        }
    }
}

pub fn lerp_to_desired_position(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &MainCamera)>,
) {
    let (mut transform, camera) = camera_query.single_mut();

    let lerped_position = transform
        .translation
        .lerp(camera.position(), time.delta_seconds() * camera.easing);

    transform.translation = lerped_position;
    transform.look_at(camera.focus, Vec3::Y);
}
