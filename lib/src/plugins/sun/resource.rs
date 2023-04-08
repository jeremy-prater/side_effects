use bevy::prelude::*;

#[derive(Resource)]
pub struct AtmosphereCounter {
    pub count: u32,
}

const ATMOSPHERE_FRAME_SKIP: u32 = 32;

impl AtmosphereCounter {
    pub fn next(&mut self) -> bool {
        let value = self.count == 0;
        self.count += 1;
        if self.count == ATMOSPHERE_FRAME_SKIP {
            self.count = 0;
        }

        value
    }
}
