use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use oxidized_navigation::{NavMeshSettings, OxidizedNavigationPlugin};

pub mod animation;
pub mod audio;
pub mod camera;
pub mod follower;
pub mod game_ui;
pub mod loading;
pub mod main_menu;
pub mod mouse_capture;
pub mod movement;
pub mod mushroom_generator;
pub mod player;
pub mod selection;
pub mod sun;
pub mod terrain;
pub mod tree_generator;

pub use animation::*;
pub use audio::*;
pub use camera::*;
pub use follower::*;
pub use game_ui::*;
pub use game_ui::*;
pub use loading::*;
pub use main_menu::*;
pub use mouse_capture::*;
pub use movement::*;
pub use mushroom_generator::*;
pub use player::*;
pub use selection::*;
pub use sun::*;
pub use terrain::*;
pub use tree_generator::*;

pub use crate::states::game_state::{debug_game_state_changes, GameState};

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(EguiPlugin)
            .insert_resource(NavMeshSettings {
                cell_width: 0.25,
                cell_height: 0.125,
                tile_width: 1000,
                world_half_extents: 250.0,
                world_bottom_bound: -100.0,
                max_traversable_slope_radians: (0.1f32).to_radians(),
                walkable_height: 20,
                walkable_radius: 1,
                step_height: 3,
                min_region_area: 100,
                merge_region_area: 500,
                max_contour_simplification_error: 1.1,
                max_edge_length: 80,
            })
            .add_plugin(OxidizedNavigationPlugin)
            .add_plugin(Sprite3dPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(FollowerPlugin)
            .add_plugin(DebugFollowerPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(TreeGeneratorPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MushroomGeneratorPlugin)
            .add_plugin(SelectionPlugin)
            .add_plugin(DebugSelectionInputPlugin)
            .add_plugin(SunPlugin)
            .add_plugin(GameAudioPlugin)
            .add_plugin(GameUIPlugin)
            .add_system(debug_game_state_changes);
        // .add_system(cursor_grab_system);
    }
}
