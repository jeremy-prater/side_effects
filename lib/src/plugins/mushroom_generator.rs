use crate::states::game_state::GameState;
use crate::systems::mushroom_generator::{init_mushrooms, spawn_mushroom};
use bevy::prelude::*;
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
