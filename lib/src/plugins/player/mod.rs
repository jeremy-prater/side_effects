pub mod component;
pub mod system;

use self::component::Player;
use crate::{states::game_state::GameState, components::tanuki::Tanuki};
use bevy::prelude::*;
use system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::InGame)))
            .add_system(player_effects.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::despawn_with::<Player>.in_schedule(OnExit(GameState::InGame)))
            .add_system(crate::despawn_with::<Tanuki>.in_schedule(OnExit(GameState::InGame)));
    }
}
