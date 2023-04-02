use bevy::prelude::*;

pub enum MushroomEffect {
    // These are the side-effects :)
    // There are good and bad ones
    NoSideEffect,

    // Negative effects
    Death,
    LoseOneActionSlot,
    LoseOneTurn,

    // Positive effects
    FullHealth,
    GainOneActionSlot,
}

#[derive(Component)]
pub struct Mushroom {
    pub hp: u32,
    pub effect: MushroomEffect,
}
