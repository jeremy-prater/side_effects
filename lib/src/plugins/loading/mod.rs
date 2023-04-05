pub mod resource;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use iyes_progress::prelude::*;
use system::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add plugin for the loading screen
            .add_plugin(
                ProgressPlugin::new(GameState::Loading)
                    .continue_to(GameState::MainMenu)
                    .track_assets(),
            )
            // Load our UI assets during our loading screen
            .add_systems(
                (setup_loading_ui, load_game_assets)
                    .chain()
                    .in_schedule(OnEnter(GameState::Loading)),
            )
            .add_system(teardown_loading_ui.in_schedule(OnExit(GameState::Loading)));
    }
}
