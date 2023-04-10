use crate::components::animation::AnimationMarker;
use crate::components::movement::{CharacterSpeed, Direction, Momentum, MovingCharacter};
use crate::events::animation::AnimationTransitionEvent;
use crate::plugins::camera::component::MainCamera;
use crate::plugins::player::component::*;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct IsInMotion(pub bool);

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

pub fn animate_character_movement(
    mut character_query: Query<(Entity, &Direction, &mut IsInMotion), With<AnimationMarker>>,
    mut animation_transition_writer: EventWriter<AnimationTransitionEvent>,
) {
    for (entity, direction, mut is_moving) in character_query.iter_mut() {
        let previous_is_moving = is_moving.0;
        is_moving.0 = direction.is_moving();

        if is_moving.0 != previous_is_moving {
            let animation_name: String = if is_moving.0 {
                "run".to_string()
            } else {
                "idle".to_string()
            };
            animation_transition_writer.send(AnimationTransitionEvent {
                entity_id: entity,
                animation_name,
            });
        }
    }
}

pub fn rotate_character_to_direction(
    time: Res<Time>,
    mut character_query: Query<(&mut Transform, &Direction), With<MovingCharacter>>,
) {
    for (mut transform, direction) in &mut character_query {
        let mut rotation_target = Transform::from_xyz(
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        );
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
    mut player_query: Query<
        (&mut CharacterSpeed, &mut Momentum, &Direction),
        With<MovingCharacter>,
    >,
) {
    for (mut character_speed, mut momentum, direction) in &mut player_query {
        if direction.is_moving() {
            character_speed.accelerate(time.delta(), time.delta_seconds());
            momentum.set(character_speed.current());
        } else {
            momentum.reset();
            character_speed.reset();
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
            // println!("Current Velocity: {:?}", velocity.linvel);
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

    if input.pressed(KeyCode::D) {
        x += 1.0;
    }

    if input.pressed(KeyCode::A) {
        x -= 1.0;
    }

    let right_vec: Vec3 = x * right;
    let forward_vec: Vec3 = z * forward;

    (right_vec + forward_vec).normalize_or_zero()
}
