use std::time::Duration;

use super::resource::*;
use crate::plugins::ActiveSelection;
use bevy::{prelude::*};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MushroomEffect {
    // These are the side-effects :)
    // There are good and bad ones
    #[default]
    NoSideEffect,

    // // Negative effects
    // Death,
    SlowDown,
    Sleep, // AKA, trippy mushroom
    Minus5Health,
    SpawnEnemy,

    // Positive effects
    FullHealth,
    SpeedUp,
    Plus5Health,
    SpawnBuddy,
}

pub static EFFECT_WEIGHTS: &[(MushroomEffect, f32)] = &[
    (MushroomEffect::NoSideEffect, 20.0),
    (MushroomEffect::SpawnBuddy, 20.0),

    // (MushroomEffect::Death, 1.0),
    (MushroomEffect::FullHealth, 3.0),

    (MushroomEffect::Sleep, 6.0),
    (MushroomEffect::SpawnEnemy, 5.0),

    (MushroomEffect::SpeedUp, 10.0),
    (MushroomEffect::SlowDown, 10.0),

    (MushroomEffect::Minus5Health, 20.0),
    (MushroomEffect::Plus5Health, 20.0),
];

#[derive(Component, Debug)]
pub struct ActiveMushroomEffect {
    pub effect: MushroomEffect,
    pub time_left: Duration,
}

#[derive(Component, Default)]
pub struct Mushroom {
    pub hp: f32,
    pub effect: MushroomEffect,
    pub x: i32,
    pub z: i32,
}

impl Mushroom {
    pub fn pick(
        &self,
        entity: Entity,
        commands: &mut Commands,
        mut mushroom_db: ResMut<MushroomDatabase>,
        active_selection: ResMut<ActiveSelection>,
    ) -> (f32, MushroomEffect) {
        // Add to picked mushroom database
        mushroom_db
            .picked_mushroom_locations
            .insert((self.x, self.z));

        // Despawn current mushroom
        crate::plugins::selection::manually_remove(entity, active_selection);
        commands.entity(entity).despawn_recursive();

        (self.hp, self.effect.clone())
    }
}
