use bevy::prelude::*;

pub mod ai;
pub mod animation;
pub mod asset_loader;
pub mod camera;
pub mod lighting;
pub mod main_menu;
pub mod movement;
pub mod mushroom_generator;
pub mod player;
pub mod tanuki;
pub mod terrain;
pub mod ui;

use ai::AiPlugin;
pub use animation::*;
pub use asset_loader::*;
pub use camera::*;
pub use lighting::*;
pub use main_menu::*;
pub use movement::*;
pub use mushroom_generator::*;
pub use player::*;
pub use tanuki::*;
pub use terrain::*;
pub use ui::*;

#[cfg(debug_assertions)]
pub mod debug;

#[cfg(debug_assertions)]
pub use debug::*;

pub use crate::states::game_state::{debug_game_state_changes, GameState};

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(AiPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TanukiAiPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MushroomGeneratorPlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(UiPlugin)
            .add_system(debug_game_state_changes);

        #[cfg(debug_assertions)]
        app.add_plugin(DebugPlugin);
    }
}
