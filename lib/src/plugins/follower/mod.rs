//! Functionality related to follower tanuki who can be commanded by the player.

use std::collections::{HashSet, VecDeque};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use big_brain::prelude::*;

use self::ai::{MoveToAction, MoveToScorer};

use super::{
    camera::component::MainCamera, FlagAssets, GameState, Selectable, Selected,
    SelectionControlEvent,
};
use crate::{
    components::{
        animation::AnimationMarker,
        movement::{CharacterSpeed, Direction, Momentum, MovingCharacter},
        tanuki::Tanuki,
    },
    plugins::player::{component::Player, system::spawn_player},
    systems::movement::IsInMotion,
};

mod ai;
use ai::{FollowAction, FollowScorer, FollowerAiPlugin};

pub struct FollowerPlugin;
impl Plugin for FollowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FollowerAiPlugin)
            .add_system(
                apply_system_buffers
                    .in_schedule(OnEnter(GameState::InGame))
                    .after(spawn_player)
                    .before(init_default_job),
            )
            .add_system(
                init_default_job
                    .in_schedule(OnEnter(GameState::InGame))
                    .after(spawn_player)
                    .after(apply_system_buffers),
            )
            .add_system(clean_up_empty_jobs.in_set(OnUpdate(GameState::InGame)));
    }
}

pub struct DebugFollowerPlugin;
impl Plugin for DebugFollowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_job_input.in_set(OnUpdate(GameState::InGame)));
    }
}

fn clean_up_empty_jobs(
    query: Query<Entity, (With<FollowerJob>, Without<DefaultFollowerJobMarker>)>,
    query_children: Query<&Children>,
    query_is_follower: Query<&Tanuki>,
    mut commands: Commands,
) {
    'outer: for entity in query.iter() {
        for child in query_children.iter_descendants(entity) {
            if query_is_follower.get(child).is_ok() {
                continue 'outer;
            }
        }
        commands.entity(entity).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
fn debug_job_input(
    mut query: Query<(Entity, &Tanuki, &mut Transform, &GlobalTransform), With<Selected>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,

    mut clear_sel: EventWriter<SelectionControlEvent>,

    rapier: Res<RapierContext>,
    asset_server: Res<AssetServer>,
    flag_assets: Res<FlagAssets>,
    default_job: Res<DefaultFollowerJob>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,

    mut egui: EguiContexts,
    mut sprite_params: Sprite3dParams,
    mut commands: Commands,
) {
    let get_cursor_pos = || {
        let window = window.get_single().ok()?;
        let (camera, camera_pos) = camera.get_single().ok()?;

        let cursor_pos = window.cursor_position()?;

        let ray = camera.viewport_to_world(camera_pos, cursor_pos)?;

        Some(
            rapier
                .cast_ray_and_get_normal(
                    ray.origin,
                    ray.direction,
                    f32::MAX,
                    true,
                    QueryFilter::exclude_kinematic().exclude_sensors(),
                )?
                .1
                .point,
        )
    };

    if keyboard.just_pressed(KeyCode::C) {
        let tanuki = commands
            .spawn(TanukiBundle {
                scene: SceneBundle {
                    scene: asset_server.load("models/Tanuki.glb#Scene0"),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                    ..Default::default()
                },
                animation_marker: AnimationMarker::new("tanuki", "idle"),
                rigid_body: RigidBody::Dynamic,
                velocity: Velocity::default(),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                collider: Collider::capsule_y(0.125, 0.125),
                direction: Direction::default(),
                momentum: Momentum::default(),
                damping: Damping {
                    linear_damping: 0.2,
                    angular_damping: 0.0,
                },
                friction: Friction {
                    coefficient: 8.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
                gravity: GravityScale(1.0),
                tanuki: Tanuki {
                    age: 1,
                    max_hp: 1,
                    hp: 1,
                    current_effects: Vec::new(),
                    blocked_effects: HashSet::new(),
                    next_actions: VecDeque::new(),
                },
                selectable: Selectable,
                thinker: Thinker::build()
                    .label("tanuki_ai")
                    .picker(Highest)
                    .when(
                        FollowScorer,
                        FollowAction {
                            end_pos: None,
                            path_cache: Vec::new().into_iter().peekable(),
                        },
                    )
                    .when(
                        MoveToScorer,
                        MoveToAction {
                            end_pos: None,
                            path_cache: Vec::new().into_iter().peekable(),
                        },
                    ),
                motion_tracker: IsInMotion(false),
                moving_char_tag: MovingCharacter,
                character_speed: CharacterSpeed::default(),
            })
            .id();
        commands.entity(default_job.0).add_child(tanuki);

        return;
    }

    if mouse.just_pressed(MouseButton::Right) {
        let mut iter = query.iter().peekable();
        if iter.peek().is_none() || egui.ctx_mut().wants_pointer_input() {
            return;
        }

        let (x, y, z);
        let job = if let Some(pos) = get_cursor_pos() {
            x = pos.x;
            y = pos.y;
            z = pos.z;

            commands
                .spawn((
                    FollowerJob::MoveTo(pos),
                    Sprite3d {
                        transform: Transform::from_xyz(pos.x, pos.y + 1.0, pos.z),
                        image: flag_assets.flag_move.clone(),
                        pixels_per_metre: 500.0,
                        ..Default::default()
                    }
                    .bundle(&mut sprite_params),
                ))
                .id()
        } else {
            return;
        };

        for (tanuki_entity, _, mut transform, global_transform) in query.iter_mut() {
            *transform = global_transform.reparented_to(&GlobalTransform::from_xyz(x, y + 1.0, z));
            commands.entity(tanuki_entity).set_parent(job);
        }

        clear_sel.send(SelectionControlEvent::ClearSelection);
    }
}

fn init_default_job(player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    let player = player_query.single();
    let follow_player_job = commands
        .spawn((
            FollowerJob::Follow(player),
            SpatialBundle::default(),
            DefaultFollowerJobMarker,
        ))
        .id();

    commands.insert_resource(DefaultFollowerJob(follow_player_job));
}

/// Component which simply defines the goal of a given job.
#[derive(Component, Debug)]
pub enum FollowerJob {
    Follow(Entity),
    MoveTo(Vec3),
}

/// Resource holding the Entity which contains the [`FollowerJob`] that follower tanuki should
/// default to.
#[derive(Resource)]
pub struct DefaultFollowerJob(Entity);

/// Component marking the default job
#[derive(Component)]
pub struct DefaultFollowerJobMarker;

#[derive(Bundle)]
pub struct TanukiBundle {
    #[bundle]
    scene: SceneBundle,
    animation_marker: AnimationMarker,
    rigid_body: RigidBody,
    velocity: Velocity,
    locked_axes: LockedAxes,
    collider: Collider,
    direction: Direction,
    momentum: Momentum,
    damping: Damping,
    friction: Friction,
    gravity: GravityScale,
    tanuki: Tanuki,
    selectable: Selectable,
    thinker: ThinkerBuilder,
    motion_tracker: IsInMotion,
    moving_char_tag: MovingCharacter,
    character_speed: CharacterSpeed,
}
