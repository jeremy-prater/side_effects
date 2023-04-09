pub mod component;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy_ninepatch::NinePatchPlugin;
use system::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NinePatchPlugin::<()>::default())
            .add_system(game_ui_startup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(game_ui_system.in_set(OnUpdate(GameState::InGame)))
            .add_system(ui_button_handler.in_set(OnUpdate(GameState::InGame)));
    }
}
