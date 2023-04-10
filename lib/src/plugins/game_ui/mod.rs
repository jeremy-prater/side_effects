pub mod component;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

use self::component::GameUI;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_ui_startup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(game_ui_system.in_set(OnUpdate(GameState::InGame)))
            .add_system(ui_button_handler.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::despawn_with::<GameUI>.in_schedule(OnExit(GameState::InGame)));
    }
}
