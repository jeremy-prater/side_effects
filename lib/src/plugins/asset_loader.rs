use crate::states::game_state::GameState;
use crate::systems::loading;
use bevy::prelude::*;
use iyes_progress::prelude::*;

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
                (loading::setup_loading_ui, loading::load_game_assets)
                    .chain()
                    .in_schedule(OnEnter(GameState::Loading)),
            )
            .add_system(loading::teardown_loading_ui.in_schedule(OnExit(GameState::Loading)));
    }
}
