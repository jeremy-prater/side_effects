// pub mod resource;
pub mod system;
pub mod resource;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
use system::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(KayakContextPlugin).add_plugin(KayakWidgets);
        // .add_system(setup_game_ui.in_schedule(OnEnter(GameState::InGame)));
    }
}
