pub mod component;
pub mod resource;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use resource::*;
use system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::InGame)))
            .add_system(player_health.in_set(OnUpdate(GameState::InGame)));
    }
}
