use bevy::prelude::*;

#[derive(Default)]
pub enum LightingTimePeriod {
    #[default]
    Dawn,
    Noon,
    Dusk,
    Night,
}

#[derive(Component, Default)]
pub struct Sun {
    pub time_of_day: LightingTimePeriod,
}
