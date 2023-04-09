use crate::states::game_state::GameState;
use crate::systems::movement::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_systems(
                (
                    set_player_direction,
                    animate_character_movement,
                    rotate_character_to_direction,
                    handle_player_speed,
                    apply_momentum,
                )
                    .chain()
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}
