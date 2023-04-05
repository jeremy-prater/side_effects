//! A module containing the [`DebugPlugin`] which is only inserted when `debug_assertions` allow
//! its conditional compilation.
//!
//! This is intended to be an arae for scrap work.

use std::collections::{HashSet, VecDeque};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use big_brain::prelude::*;

use super::{
    ai::{components::UnassignedTanukiJob, TanukiAiBundle},
    GameState,
};
use crate::{
    components::{animation::AnimationMarker, movement::Direction, player::Player, tanuki::Tanuki},
    plugins::{
        ai::components::{MoveAbsoluteScorer, MoveToAbsolute, TanukiJob},
        ui::utils::cursor_world_position,
    },
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        warn!("Running with debug_assertions enabled");
        app.add_system(spawn_tanuki.in_set(OnUpdate(GameState::InGame)))
            .add_system(change_order.in_set(OnUpdate(GameState::InGame)));
    }
}

fn change_order(
    mut job: Query<&mut TanukiJob, With<UnassignedTanukiJob>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(Entity, &Camera, &GlobalTransform)>,

    rapier_context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::M) {
        if let Some(world_pos) = cursor_world_position(windows, cameras, rapier_context) {
            let mut job = job.single_mut();
            *job = TanukiJob::MoveAbsolute(world_pos);
        }
    }
}

fn spawn_tanuki(
    debug_marker: Query<Entity, With<UnassignedTanukiJob>>,
    query: Query<&GlobalTransform, With<Player>>,

    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,

    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Comma) {
        let mut transform = query.single().compute_transform();
        let parent = debug_marker.into_iter().next().unwrap();

        transform.translation.x -= 3.0;
        transform.scale = Vec3::new(0.5, 0.5, 0.5);

        let tanuki = commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load("models/Tanuki.glb#Scene0"),
                    transform,
                    ..Default::default()
                },
                TanukiAiBundle::default(),
            ))
            .insert(Visibility::Visible)
            .insert(AnimationMarker::new("tanuki", "idle"))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(!LockedAxes::ROTATION_LOCKED_Y)
            .insert(Collider::capsule_y(0.25, 0.25))
            .insert(Direction::default())
            .insert(Damping {
                linear_damping: 0.2,
                angular_damping: 0.0,
            })
            .insert(Friction {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(GravityScale(1.0))
            .insert(Tanuki {
                age: 1,
                max_hp: 1,
                hp: 1,
                current_effects: Vec::new(),
                blocked_effects: HashSet::new(),
                next_actions: VecDeque::new(),
            })
            .id();

        commands.entity(parent).add_child(tanuki);
    }
}
