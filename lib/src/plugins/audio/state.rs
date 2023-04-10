// Our Game State
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AudioState {
    #[default]
    MenuIntro,
    MenuLoop,
    AmbientIntro,
    AmbientLoop,
    GoodIntro,
    GoodLoop,
    BadIntro,
    BadLoop,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AudioEvent {
    #[default]
    Ambient,
    Good,
    Bad,
}
