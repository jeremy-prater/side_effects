pub mod resource;
pub mod system;
pub mod component;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

pub struct TreeGeneratorPlugin;

impl Plugin for TreeGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_trees.in_schedule(OnEnter(GameState::InGame)))
            .add_system(spawn_trees.in_set(OnUpdate(GameState::InGame)));
    }
}
