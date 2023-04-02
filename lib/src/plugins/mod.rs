use bevy::prelude::*;

pub mod asset_loader;
pub mod camera;
pub mod mushroom_generator;
pub mod player;
pub mod terrain;
pub mod main_menu;

pub use asset_loader::*;
pub use camera::*;
pub use mushroom_generator::*;
pub use player::*;
pub use terrain::*;
pub use main_menu::*;

pub use crate::states::game_state::{debug_game_state_changes, GameState};

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(MainMenuPlugin)
            .add_system(debug_game_state_changes);
    }
}
