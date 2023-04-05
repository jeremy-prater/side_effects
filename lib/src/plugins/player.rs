use crate::resources::player::PlayerSpeed;
use crate::states::game_state::GameState;
use crate::systems::player::spawn_player;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::InGame)));
    }
}
