use super::component::*;
use super::resource::TerrainDatabase;
use crate::components::player::Player;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use noise::{NoiseFn, Perlin};

const TERRAIN_X_SPAWN_RANGE: i32 = 1024;
const TERRAIN_Z_SPAWN_RANGE: i32 = 1024;
const TERRAIN_RADIUS_SPAWN_STEP: usize = 32;
const TERRAIN_NOISE_SCALE: f64 = 1.0 / 256.0;

pub fn init_terrain(mut commands: Commands) {
    commands.insert_resource(TerrainDatabase {
        terrain_locations: HashSet::new(),
    });
}

pub fn spawn_terrain(
    mut commands: Commands,
    mut terrain_db: ResMut<TerrainDatabase>,
    _asset_server: Res<AssetServer>,
    player_query: Query<&Transform, &Player>,
    _query: Query<&Terrain>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    despawner: Query<(Entity, &Transform), With<Terrain>>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return;
    }

    let origin = player.unwrap().translation;

    let x = ((origin.x / TERRAIN_RADIUS_SPAWN_STEP as f32).ceil()
        * TERRAIN_RADIUS_SPAWN_STEP as f32) as i32;
    let z = ((origin.z / TERRAIN_RADIUS_SPAWN_STEP as f32).ceil()
        * TERRAIN_RADIUS_SPAWN_STEP as f32) as i32;

    let x_start = x - TERRAIN_X_SPAWN_RANGE;
    let x_end = x + TERRAIN_X_SPAWN_RANGE;

    let z_start = z - TERRAIN_Z_SPAWN_RANGE;
    let z_end = z + TERRAIN_Z_SPAWN_RANGE;

    let perlin = Perlin::new(0);

    let gradient = colorgrad::greens();

    for x in (x_start..x_end).step_by(TERRAIN_RADIUS_SPAWN_STEP) {
        for z in (z_start..z_end).step_by(TERRAIN_RADIUS_SPAWN_STEP) {
            if !terrain_db.terrain_locations.contains(&(x, z)) {
                terrain_db.terrain_locations.insert((x, z));

                let value = perlin.get([
                    x as f64 * TERRAIN_NOISE_SCALE,
                    z as f64 * TERRAIN_NOISE_SCALE,
                ]);

                // Generate color

                let tile_color = gradient.at(value);
                let bevy_color: Color = Color::Rgba {
                    red: tile_color.r as f32,
                    green: tile_color.g as f32,
                    blue: tile_color.b as f32,
                    alpha: tile_color.a as f32,
                };

                // Ground
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(
                            TERRAIN_RADIUS_SPAWN_STEP as f32,
                            1.0,
                            TERRAIN_RADIUS_SPAWN_STEP as f32,
                        ))),
                        material: materials.add(StandardMaterial {
                            base_color: bevy_color,
                            perceptual_roughness : 0.9,
                            ..default()
                        }),
                        transform: Transform::from_xyz(x as f32, -1.0, z as f32),
                        ..default()
                    })
                    .insert(Collider::cuboid(
                        TERRAIN_RADIUS_SPAWN_STEP as f32,
                        0.5,
                        TERRAIN_RADIUS_SPAWN_STEP as f32,
                    ))
                    .insert(RigidBody::Fixed)
                    .insert(Terrain::default());
            }
        }
    }

    for (entity, transform) in despawner.iter() {
        if transform.translation.distance(origin)
            > (TERRAIN_X_SPAWN_RANGE + TERRAIN_Z_SPAWN_RANGE) as f32
        {
            commands.entity(entity).despawn_recursive();
            terrain_db.terrain_locations.remove(&(
                transform.translation.x as i32,
                transform.translation.z as i32,
            ));
        }
    }
}
