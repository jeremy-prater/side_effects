use bevy::prelude::*;

#[derive(Resource)]
pub struct MainUI(pub Entity);

impl MainUI {
    pub fn add_toast(text: String) {
        info!("{}", text);
    }
}