use crate::systems::lighting::*;
use bevy::prelude::*;

use super::GameState;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_sun.in_schedule(OnEnter(GameState::InGame)));
    }
}
