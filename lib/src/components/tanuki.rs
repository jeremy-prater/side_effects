use bevy::prelude::*;
use crate::components::mushroom::MushroomEffect;

pub enum TanukiActions {
    Idle,
    Move,
    PickMushroom,
    UseMushroom,
    // ATTACK?
    // DEFEND?
}

#[derive(Component)]
pub struct Tanuki {
    pub age: u32,
    pub hp: u32,
    pub current_effects: Vec<MushroomEffect>,
    pub blocked_effects: Vec<MushroomEffect>,
    pub next_action: TanukiActions
}
