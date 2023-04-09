// pub mod resource;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_ui_startup.in_schedule(OnEnter(GameState::InGame)));
    }
}
