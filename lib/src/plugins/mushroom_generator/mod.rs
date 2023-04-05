pub mod resource;
pub mod system;

use crate::states::game_state::GameState;
use bevy::prelude::*;
use system::*;

pub struct MushroomGeneratorPlugin;

impl Plugin for MushroomGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_mushrooms.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (spawn_mushroom,)
                    .chain()
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}
