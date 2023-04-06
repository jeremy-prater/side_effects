use bevy::prelude::*;

pub mod animation;
pub mod camera;
pub mod lighting;
pub mod loading;
pub mod main_menu;
pub mod mouse_capture;
pub mod movement;
pub mod mushroom_generator;
pub mod player;
pub mod terrain;
pub mod tree_generator;

pub use animation::*;
pub use camera::*;
pub use lighting::*;
pub use loading::*;
pub use main_menu::*;
pub use mouse_capture::*;
pub use movement::*;
pub use mushroom_generator::*;
pub use player::*;
pub use terrain::*;
pub use tree_generator::*;

pub use crate::states::game_state::{debug_game_state_changes, GameState};

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(TreeGeneratorPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MushroomGeneratorPlugin)
            .add_plugin(LightingPlugin)
            .add_system(debug_game_state_changes);
            // .add_system(cursor_grab_system);
    }
}
