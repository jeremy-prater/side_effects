use std::{f32::consts::PI, time::Duration};

use super::component::*;
use super::resource::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

const DAY_LENGTH: Duration = Duration::from_secs(60 * 5);

pub fn spawn_sun(mut commands: Commands) {
    let directional_light = DirectionalLight {
        color: Color::BLACK,
        shadows_enabled: true,
        illuminance: 0.0,
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

    commands.insert_resource(AtmosphereCounter {
        count: 0,
    });
}

pub fn update_sun(
    mut sun: Query<(&mut Sun, &mut DirectionalLight, &mut Transform)>,
    mut atmosphere: AtmosphereMut<Nishita>,
    mut atmo_counter: ResMut<AtmosphereCounter>,
    time: Res<Time>,
) {
    let (mut sun, mut light, mut transform) = sun.get_single_mut().unwrap();
    let sun_gradient = colorgrad::magma();

    sun.time_of_day += time.delta();

    if sun.time_of_day > DAY_LENGTH {
        sun.time_of_day = Duration::ZERO;
    }

    let percent_of_day = sun.time_of_day.as_secs_f32() / DAY_LENGTH.as_secs_f32();

    // This is the curve for sun rise / sun set
    // https://www.desmos.com/calculator/81yim8indl
    let gradient_pos = f32::sin(percent_of_day * PI).powf(1.0 / 3.0);

    let new_color = sun_gradient.at(gradient_pos.into()).to_linear_rgba();

    let new_color = Color::rgb(new_color.0 as f32, new_color.1 as f32, new_color.2 as f32);
    light.color = new_color;
    light.illuminance = 7000.0 + (percent_of_day * 3000.0);

    // Curve for sunlight angle
    // https://www.desmos.com/calculator/2njavqj7rg
    let sun_angle_x = 10.0 - (percent_of_day * 20.0);
    let sun_angle_y = f32::sin(percent_of_day * PI).powf(2.0 / 1.0) * 20.0;

    let sun_position = Vec3::from_slice(&[sun_angle_x, sun_angle_y, 1.0]);

    *transform = Transform::from_xyz(sun_position.x, sun_position.y, sun_position.z);
    transform.look_at(Vec3::ZERO, Vec3::Y);

    if atmo_counter.next() {
        atmosphere.sun_position = sun_position;
        atmosphere.sun_intensity = 22.0 * gradient_pos;
    }
}
