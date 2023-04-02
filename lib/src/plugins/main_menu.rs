use crate::states::game_state::GameState;
use crate::systems::main_menu;
use bevy::prelude::*;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu::setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(main_menu::main_menu_ui_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_systems(
                (
                    crate::despawn_with::<crate::components::main_menu::MainMenu>,
                    crate::despawn_with::<crate::components::main_menu::Camera>,
                )
                    .chain()
                    .in_schedule(OnExit(GameState::MainMenu)),
            );
    }
}
