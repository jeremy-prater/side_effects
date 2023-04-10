pub mod component;
pub mod resource;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

use self::component::Mushroom;

pub struct MushroomGeneratorPlugin;

impl Plugin for MushroomGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_mushrooms.in_schedule(OnEnter(GameState::InGame)))
            .add_system(spawn_mushroom.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::despawn_with::<Mushroom>.in_schedule(OnExit(GameState::InGame)));
    }
}
