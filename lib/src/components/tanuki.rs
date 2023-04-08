use crate::plugins::mushroom_generator::component::*;
use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

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
    pub max_hp: u32,
    pub hp: u32,
    pub current_effects: Vec<ActiveMushroomEffect>,
    pub blocked_effects: HashSet<MushroomEffect>,
    pub next_actions: VecDeque<TanukiActions>,
}
