use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerSpeed {
    accel_timer: Timer,
    decel_timer: Timer,
    base_speed: f32,
    current_speed: f32,
    base_top_speed: f32,
    top_speed: f32,
    acceleration: f32,
}

impl PlayerSpeed {
    pub fn reset(&mut self) {
        self.current_speed = self.base_speed;
        self.top_speed = self.base_top_speed;
        self.accel_timer.reset();
    }

    pub fn accelerate(&mut self, delta: std::time::Duration, seconds: f32) {
        self.accel_timer.tick(delta);
        if self.accel_timer.finished() {
            if self.current_speed + 0.3 <= self.top_speed {
                self.current_speed = self.current_speed
                    + (self.top_speed - self.current_speed) * (seconds * self.acceleration);
            } else {
                self.current_speed = self.top_speed;
            }
        }
    }

    pub fn current(&self) -> f32 {
        self.current_speed
    }

    pub fn set(&mut self, speed: f32) {
        self.top_speed = speed;
        self.current_speed = speed;
    }
}

impl Default for PlayerSpeed {
    fn default() -> Self {
        PlayerSpeed {
            accel_timer: Timer::from_seconds(0.3, TimerMode::Once),
            decel_timer: Timer::from_seconds(0.5, TimerMode::Once),
            base_speed: 7.5,
            current_speed: 7.5,
            top_speed: 15.0,
            base_top_speed: 15.0,
            acceleration: 1.0,
        }
    }
}
