use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Sun {
    pub time_of_day: Duration,
}

const START_TIME: Duration = Duration::from_secs(2 * 60);

impl Default for Sun {
    fn default() -> Self {
        Sun {
            time_of_day: START_TIME,
        }
    }
}
