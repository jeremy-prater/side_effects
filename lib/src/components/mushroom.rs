use bevy::prelude::*;

#[derive(Default)]
pub enum MushroomEffect {
    // These are the side-effects :)
    // There are good and bad ones
    #[default]
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
pub struct ActiveMushroomEffect {
    pub effect: MushroomEffect,
    pub turns_left: u32,
}

#[derive(Component, Default)]
pub struct Mushroom {
    pub hp: u32,
    pub effect: MushroomEffect,
}
