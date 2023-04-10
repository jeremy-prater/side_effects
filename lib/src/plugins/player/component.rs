use crate::plugins::mushroom_generator::component::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub hp: f32,
    pub active_effects: Vec<ActiveMushroomEffect>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            hp: 100.0,
            active_effects: Vec::new(),
        }
    }
}
