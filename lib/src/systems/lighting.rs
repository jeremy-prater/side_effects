use crate::components::lighting::Sun;
use bevy::prelude::*;

pub fn spawn_sun(mut commands: Commands) {
    let directional_light = DirectionalLight {
        color: Color::ORANGE_RED,
        ..default()
    };

    let mut transform = Transform::from_xyz(0.0, 20.0, 0.0);
    transform.rotate_x(20.0_f32.to_radians());

    commands
        .spawn(DirectionalLightBundle {
            directional_light,
            transform,
            ..default()
        })
        .insert(Sun::default());
}
