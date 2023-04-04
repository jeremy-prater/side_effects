use std::f32::consts::PI;

use crate::components::{camera::MainCamera, mushroom::Mushroom};
use crate::resources::mushroom_db::MushroomDatabase;
use bevy::prelude::*;
use bevy::utils::HashSet;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

const MUSHROOM_X_SPAWN_RANGE: i32 = 100;
const MUSHROOM_Z_SPAWN_RANGE: i32 = 100;
const MUSHROOM_RADIUS_SPAWN_STEP: usize = 10;
const MUSHROOM_RENDER_SCALE: f32 = 0.30;
const MUSHROOM_NOISE_SCALE: f64 = 1.0 / 250.0;

const MUSHROOM_GEN_LOW: f32 = 0.0;
const MUSHROOM_GEN_HIGH: f32 = 0.6;

const MUSHROOM_ROTATE: f32 = 0.05;
const MUSHROOM_X_JITTER: f32 = 2.0;
const MUSHROOM_Z_JITTER: f32 = 2.0;

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
    camera_query: Query<&MainCamera>,
    query: Query<&Mushroom>,
) {
    let camera = camera_query.get_single();
    if camera.is_err() {
        return;
    }

    let num_mushrooms = query.iter().len();
    let mut generate_count = MAX_MUSHROOMS - num_mushrooms;

    if generate_count == 0 {
        // No mushrooms to grow
        return;
    }

    let origin = camera_query.get_single().unwrap().offset;
    let mushroom_model: Handle<Scene> = asset_server.load("models/mushroom2.glb#Scene0");

    let x_start = (origin.x as i32 - MUSHROOM_X_SPAWN_RANGE) / 2;
    let x_end = (origin.x as i32 + MUSHROOM_X_SPAWN_RANGE) / 2;

    let z_start = (origin.z as i32 - MUSHROOM_Z_SPAWN_RANGE) / 2;
    let z_end = (origin.z as i32 + MUSHROOM_Z_SPAWN_RANGE) / 2;

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
                            0.0,
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
                    .insert(Mushroom::default());

                generate_count -= 1;
                if generate_count == 0 {
                    return;
                }
            }
        }

        // TODO : Query mushrooms too far from camera and despawn
    }
}
