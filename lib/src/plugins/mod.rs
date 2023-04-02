use bevy::prelude::*;

pub mod camera;
pub mod player;
pub mod terrain;

pub use camera::*;
pub use player::*;
pub use terrain::*;

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TerrainPlugin);
    }
}
