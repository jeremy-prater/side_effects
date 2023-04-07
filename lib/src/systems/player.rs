use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{
    animation::AnimationMarker,
    movement::{Direction, Momentum},
    player::Player,
};
use crate::plugins::camera::component::MainCameraTarget;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/Tanuki.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(AnimationMarker::new("tanuki", "idle"))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(Direction::default())
        .insert(Momentum::default())
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
        .insert(Player);
}
