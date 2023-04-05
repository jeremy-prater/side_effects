use super::resource::TreeDatabase;
use crate::components::player::Player;
use crate::components::tree::Tree;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::f32::consts::PI;

const TREE_X_SPAWN_RANGE: i32 = 250;
const TREE_Z_SPAWN_RANGE: i32 = 250;
const TREE_RADIUS_SPAWN_STEP: usize = 25;
const TREE_RENDER_SCALE: f32 = 0.70;
const TREE_NOISE_SCALE: f64 = 1.0 / 250.0;

const TREE_GEN_LOW: f32 = 0.3;
const TREE_GEN_HIGH: f32 = 0.7;

const TREE_ROTATE: f32 = 0.05;
const TREE_X_JITTER: f32 = 10.0;
const TREE_Z_JITTER: f32 = 10.0;

const MAX_TREES: usize = 2048;

pub fn init_trees(mut commands: Commands) {
    commands.insert_resource(TreeDatabase {
        tree_locations: HashSet::new(),
    });
}

pub fn spawn_trees(
    mut commands: Commands,
    mut trees_db: ResMut<TreeDatabase>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, &Player>,
    query: Query<&Tree>,
    despawner: Query<(Entity, &Transform), With<Tree>>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return;
    }

    let num_trees = query.iter().len();
    let mut generate_count = MAX_TREES - num_trees;

    if generate_count == 0 {
        // No trees to grow
        return;
    }

    let origin = player.unwrap().translation;
    let trees_model: Handle<Scene> = asset_server.load("models/tree_1.glb#Scene0");

    let x =
        ((origin.x / TREE_RADIUS_SPAWN_STEP as f32).ceil() * TREE_RADIUS_SPAWN_STEP as f32) as i32;
    let z =
        ((origin.z / TREE_RADIUS_SPAWN_STEP as f32).ceil() * TREE_RADIUS_SPAWN_STEP as f32) as i32;

    let x_start = x - TREE_X_SPAWN_RANGE;
    let x_end = x + TREE_X_SPAWN_RANGE;

    let z_start = z - TREE_Z_SPAWN_RANGE;
    let z_end = z + TREE_Z_SPAWN_RANGE;

    let perlin = Perlin::new(0);

    for x in (x_start..x_end).step_by(TREE_RADIUS_SPAWN_STEP) {
        for z in (z_start..z_end).step_by(TREE_RADIUS_SPAWN_STEP) {
            let value =
                perlin.get([x as f64 * TREE_NOISE_SCALE, z as f64 * TREE_NOISE_SCALE]) as f32;

            if (TREE_GEN_LOW..TREE_GEN_HIGH).contains(&value)
                && !trees_db.tree_locations.contains(&(x, z))
            {
                trees_db.tree_locations.insert((x, z));
                commands
                    .spawn(SceneBundle {
                        scene: trees_model.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 + rand::thread_rng().gen_range(-TREE_X_JITTER..TREE_X_JITTER),
                            0.0,
                            z as f32 + rand::thread_rng().gen_range(-TREE_Z_JITTER..TREE_Z_JITTER),
                        ))
                        .with_rotation(Quat::from_euler(
                            EulerRot::YXZ,
                            rand::thread_rng().gen_range(0.0..=2.0) * PI,
                            rand::thread_rng().gen_range(-TREE_ROTATE..TREE_ROTATE) * PI,
                            rand::thread_rng().gen_range(-TREE_ROTATE..TREE_ROTATE) * PI,
                        ))
                        .with_scale(Vec3::from((
                            TREE_RENDER_SCALE,
                            TREE_RENDER_SCALE,
                            TREE_RENDER_SCALE,
                        ))),
                        ..default()
                    })
                    .insert(RigidBody::Fixed)
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(Collider::cylinder(1.0, (1.5 * TREE_RENDER_SCALE).max(4.0)))
                    .insert(Tree::default());

                generate_count -= 1;
                if generate_count == 0 {
                    return;
                }
            }
        }

        for (entity, transform) in despawner.iter() {
            if transform.translation.distance(origin) > (TREE_X_SPAWN_RANGE + TREE_Z_SPAWN_RANGE) as f32 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
