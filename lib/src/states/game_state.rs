// Our Game State
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    GameResults,
}

pub fn debug_game_state_changes(state: Res<State<GameState>>) {
    if state.is_changed() {
        info!("GameState :: Game state change to {:?}!", state);
    }
}
