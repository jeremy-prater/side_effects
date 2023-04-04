use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(500.0, 1.0, 500.0))),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(250.0, 0.5, 250.0))
        .insert(RigidBody::Fixed);
}