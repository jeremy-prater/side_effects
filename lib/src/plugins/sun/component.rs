use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Sun {
    pub time_of_day: Duration,
}
