use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

pub mod systems;
pub mod utils;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin).add_plugin(KayakWidgets);
        // .add_system(handle_radial_menu.in_set(OnUpdate(GameState::InGame)));
    }
}
