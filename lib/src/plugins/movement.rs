use crate::states::game_state::GameState;
use crate::systems::movement::*;
use bevy::prelude::*;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                set_player_direction,
                rotate_player_to_direction,
                handle_player_speed,
                apply_momentum,
            )
                .chain()
                .in_set(OnUpdate(GameState::InGame)),
        );
    }
}
