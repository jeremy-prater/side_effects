use crate::plugins::ActiveSelection;

use super::resource::*;
use bevy::prelude::*;

#[derive(Default, Debug, Clone)]
pub enum MushroomEffect {
    // These are the side-effects :)
    // There are good and bad ones
    #[default]
    NoSideEffect,

    // Negative effects
    Death,
    SlowDown,
    Sleep,

    // Positive effects
    FullHealth,
    SpeedUp,
}

#[derive(Component)]
pub struct ActiveMushroomEffect {
    pub effect: MushroomEffect,
    pub time_left: f32,
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
        mut commands: Commands,
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
