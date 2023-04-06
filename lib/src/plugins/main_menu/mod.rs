pub mod component;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;

use component::*;
use system::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(main_menu_ui_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_systems(
                (
                    crate::despawn_with::<MainMenu>,
                    crate::despawn_with::<Camera>,
                )
                    .chain()
                    .in_schedule(OnExit(GameState::MainMenu)),
            );
    }
}
