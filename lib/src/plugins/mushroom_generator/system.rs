use super::component::*;
use super::resource::*;
use crate::components::player::Player;
use crate::plugins::Selectable;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::f32::consts::PI;

const MUSHROOM_X_SPAWN_RANGE: i32 = 128;
const MUSHROOM_Z_SPAWN_RANGE: i32 = 128;
const MUSHROOM_RADIUS_SPAWN_STEP: usize = 8;
const MUSHROOM_RENDER_SCALE: f32 = 0.25;
const MUSHROOM_NOISE_SCALE: f64 = 1.0 / 256.0;

const MUSHROOM_GEN_LOW: f32 = 0.70;
const MUSHROOM_GEN_HIGH: f32 = 0.85;

const MUSHROOM_ROTATE: f32 = 0.05;
const MUSHROOM_X_JITTER: f32 = 4.0;
const MUSHROOM_Z_JITTER: f32 = 4.0;

const MAX_MUSHROOMS: usize = 256;

pub fn init_mushrooms(mut commands: Commands) {
    commands.insert_resource(MushroomDatabase {
        mushroom_locations: HashSet::new(),
        picked_mushroom_locations: HashSet::new(),
    });
}

pub fn spawn_mushroom(
    mut commands: Commands,
    mut mushroom_db: ResMut<MushroomDatabase>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, &Player>,
    query: Query<&Mushroom>,
    despawner: Query<(Entity, &Transform), With<Mushroom>>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return;
    }

    let num_mushrooms = query.iter().len();
    let mut generate_count = MAX_MUSHROOMS - num_mushrooms;

    if generate_count == 0 {
        // No mushrooms to grow
        return;
    }

    let origin = player.unwrap().translation;
    let mushroom_model: Handle<Scene> = asset_server.load("models/mushroom2.glb#Scene0");

    let x = ((origin.x / MUSHROOM_RADIUS_SPAWN_STEP as f32).ceil()
        * MUSHROOM_RADIUS_SPAWN_STEP as f32) as i32;
    let z = ((origin.z / MUSHROOM_RADIUS_SPAWN_STEP as f32).ceil()
        * MUSHROOM_RADIUS_SPAWN_STEP as f32) as i32;

    let x_start = x - MUSHROOM_X_SPAWN_RANGE;
    let x_end = x + MUSHROOM_X_SPAWN_RANGE;

    let z_start = z - MUSHROOM_Z_SPAWN_RANGE;
    let z_end = z + MUSHROOM_Z_SPAWN_RANGE;

    let perlin = Perlin::new(0);

    for x in (x_start..x_end).step_by(MUSHROOM_RADIUS_SPAWN_STEP) {
        for z in (z_start..z_end).step_by(MUSHROOM_RADIUS_SPAWN_STEP) {
            let value = perlin.get([
                x as f64 * MUSHROOM_NOISE_SCALE,
                z as f64 * MUSHROOM_NOISE_SCALE,
            ]) as f32;

            if (MUSHROOM_GEN_LOW..MUSHROOM_GEN_HIGH).contains(&value)
                && !mushroom_db.mushroom_locations.contains(&(x, z))
                && !mushroom_db.picked_mushroom_locations.contains(&(x, z))
            {
                mushroom_db.mushroom_locations.insert((x, z));
                commands
                    .spawn(SceneBundle {
                        scene: mushroom_model.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32
                                + rand::thread_rng()
                                    .gen_range(-MUSHROOM_X_JITTER..MUSHROOM_X_JITTER),
                            -0.5,
                            z as f32
                                + rand::thread_rng()
                                    .gen_range(-MUSHROOM_Z_JITTER..MUSHROOM_Z_JITTER),
                        ))
                        .with_rotation(Quat::from_euler(
                            EulerRot::YXZ,
                            rand::thread_rng().gen_range(0.0..=2.0) * PI,
                            rand::thread_rng().gen_range(-MUSHROOM_ROTATE..MUSHROOM_ROTATE) * PI,
                            rand::thread_rng().gen_range(-MUSHROOM_ROTATE..MUSHROOM_ROTATE) * PI,
                        ))
                        .with_scale(Vec3::from((
                            MUSHROOM_RENDER_SCALE,
                            MUSHROOM_RENDER_SCALE,
                            MUSHROOM_RENDER_SCALE,
                        ))),
                        ..default()
                    })
                    .insert(RigidBody::Fixed)
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(Collider::cylinder(
                        3.0,
                        (4.0 * MUSHROOM_RENDER_SCALE).max(3.0),
                    ))
                    .insert(Selectable)
                    .insert(Mushroom::default());

                generate_count -= 1;
                if generate_count == 0 {
                    return;
                }
            }
        }
    }

    for (entity, transform) in despawner.iter() {
        if transform.translation.distance(origin)
            > (MUSHROOM_X_SPAWN_RANGE + MUSHROOM_Z_SPAWN_RANGE) as f32
        {
            commands.entity(entity).despawn_recursive();
            mushroom_db.mushroom_locations.remove(&(
                transform.translation.x as i32,
                transform.translation.z as i32,
            ));
        }
    }
}
