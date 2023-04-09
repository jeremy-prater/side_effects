use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub hp: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player { hp: 100.0 }
    }
}
