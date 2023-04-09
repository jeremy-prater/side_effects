use super::component::*;
use crate::components::{
    animation::AnimationMarker,
    movement::{CharacterSpeed, Direction, Momentum, MovingCharacter},
};
use crate::plugins::camera::component::MainCameraTarget;
use crate::plugins::selection::*;
use crate::systems::movement::IsInMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/Tanuki.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(AnimationMarker::new("tanuki", "idle"))
        .insert(IsInMotion(false))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule_y(0.25, 0.25))
        .insert(Direction::default())
        .insert(Momentum::default())
        .insert(MovingCharacter)
        .insert(MainCameraTarget)
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 0.0,
        })
        .insert(Friction {
            coefficient: 8.0,
            combine_rule: CoefficientCombineRule::Average,
        })
        .insert(GravityScale(1.0))
        .insert(Name::new("Player"))
        .insert(Player::default())
        .insert(Selectable)
        .insert(CharacterSpeed::default())
        .insert(Player::default());
}

pub fn player_health(mut player: Query<&mut Player>, time: Res<Time>) {
    if let Ok(mut player) = player.get_single_mut() {
        player.hp -= time.delta_seconds() * 0.5;
    }
}
