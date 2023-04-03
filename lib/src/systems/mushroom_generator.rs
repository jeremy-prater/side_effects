use crate::components::{camera::MainCamera, mushroom::Mushroom};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

const MUSHROOM_X_SPAWN_RANGE: i32 = 100;
const MUSHROOM_Z_SPAWN_RANGE: i32 = 100;
const MUSHROOM_RADIUS_SPAWN_STEP: usize = 10;
const MUSHROOM_RENDER_SCALE: f32 = 0.30;
const MUSHROOM_NOISE_SCALE: f64 = 250.0;

const MUSHROOM_GEN_LOW: f64 = 0.2;
const MUSHROOM_GEN_HIGH: f64 = 0.5;

const MAX_MUSHROOMS: usize = 256;

pub fn init_mushrooms(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&MainCamera>,
) {
}

pub fn spawn_mushroom(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
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
                x as f64 / MUSHROOM_NOISE_SCALE,
                z as f64 / MUSHROOM_NOISE_SCALE,
            ]);

            if value >= MUSHROOM_GEN_LOW && value <= MUSHROOM_GEN_HIGH {
                commands
                    .spawn(SceneBundle {
                        scene: mushroom_model.clone(),
                        transform: Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32))
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
