use crate::components::{
    camera::MainCamera,
    movement::{Direction, Momentum},
    player::Player,
};
use crate::resources::player::PlayerSpeed;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn set_player_direction(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Direction, With<Player>>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = camera_query.single();
    for mut direction in &mut player_query {
        direction.set(get_direction_in_camera_space(camera_transform, &input));
    }
}

pub fn rotate_player_to_direction(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Direction), With<Player>>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction) in &mut player_query {
        rotation_target.translation = transform.translation;
        let flat_velo_direction =
            Vec3::new(direction.get().x, 0.0, direction.get().z).normalize_or_zero();
        if flat_velo_direction != Vec3::ZERO {
            let target_position = rotation_target.translation + flat_velo_direction;

            rotation_target.look_at(target_position, Vec3::Y);
            let turn_speed = 10.0;

            transform.rotation = transform
                .rotation
                .slerp(rotation_target.rotation, time.delta_seconds() * turn_speed);
        }
    }
}

pub fn handle_player_speed(
    time: Res<Time>,
    mut player_speed: ResMut<PlayerSpeed>,
    mut player_query: Query<(&mut Momentum, &Direction), With<Player>>,
) {
    for (mut momentum, direction) in &mut player_query {
        if direction.is_moving() {
            player_speed.accelerate(time.delta(), time.delta_seconds());
            momentum.set(player_speed.current());
        } else {
            momentum.reset();
            player_speed.reset();
        }
    }
}

pub fn apply_momentum(mut query: Query<(&mut Velocity, &Transform, &Momentum)>) {
    for (mut velocity, transform, momentum) in &mut query {
        let mut speed_to_apply = Vec3::ZERO;
        let mut should_change_velocity: bool = false;

        if momentum.has_momentum() {
            should_change_velocity = true;
            let forward = transform.forward();
            speed_to_apply += forward * momentum.get();
        }

        if should_change_velocity {
            velocity.linvel.x = speed_to_apply.x;
            velocity.linvel.z = speed_to_apply.z;
        }
    }
}

pub fn get_direction_in_camera_space(
    camera_transform: &Transform,
    input: &Res<Input<KeyCode>>,
) -> Vec3 {
    let mut x = 0.0;
    let mut z = 0.0;

    let mut forward = camera_transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = camera_transform.right();
    right.y = 0.0;
    right = right.normalize();

    if input.pressed(KeyCode::W) {
        z += 1.0;
    }

    if input.pressed(KeyCode::S) {
        z -= 1.0;
    }

    if input.pressed(KeyCode::A) {
        x += 1.0;
    }

    if input.pressed(KeyCode::D) {
        x -= 1.0;
    }

    let right_vec: Vec3 = x * right;
    let forward_vec: Vec3 = z * forward;

    (right_vec + forward_vec).normalize_or_zero()
}
