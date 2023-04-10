use std::time::Duration;

use super::component::*;
use crate::components::{
    animation::AnimationMarker,
    movement::{CharacterSpeed, Direction, Momentum, MovingCharacter}, tanuki::Tanuki,
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

pub fn player_effects(mut commands: Commands, mut player: Query<&mut Player>, tanukis: Query<&Tanuki>, time: Res<Time>) {
    if player.is_empty() {
        return;
    }

    let mut player = player.get_single_mut().unwrap();

    let old_hp = player.hp;

    //
    player.hp -= time.delta_seconds() * 0.2 * (1 + tanukis.iter().len()) as f32;

    if player.hp < 0.0 {
        commands.insert_resource(NextState(Some(
            crate::states::game_state::GameState::MainMenu,
        )));
    }

    if old_hp > 40.0 && player.hp < 40.0 {
        commands.insert_resource(NextState(Some(
            crate::plugins::audio::state::AudioState::BadIntro,
        )));
    }

    // Count down active effects
    player.active_effects.retain_mut(|effect| {
        effect.time_left = effect.time_left.saturating_sub(time.delta());

        if effect.time_left == Duration::ZERO {
            return false;
        }

        true
    });
}
