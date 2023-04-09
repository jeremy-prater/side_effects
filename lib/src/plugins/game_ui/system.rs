use crate::plugins::camera::component::MainCamera;
use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

pub fn setup_game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    camera: Query<Entity, With<MainCamera>>,
) {

}
