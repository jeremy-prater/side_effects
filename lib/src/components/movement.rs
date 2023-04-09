use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterSpeed {
    accel_timer: Timer,
    decel_timer: Timer,
    base_speed: f32,
    current_speed: f32,
    base_top_speed: f32,
    top_speed: f32,
    acceleration: f32,
}

impl CharacterSpeed {
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

impl Default for CharacterSpeed {
    fn default() -> Self {
        CharacterSpeed {
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

#[derive(Component, Default)]
pub struct Direction(Vec3);

impl Direction {
    pub fn get(&self) -> Vec3 {
        self.0
    }
    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn is_moving(&self) -> bool {
        self.0 != Vec3::ZERO
    }
}

#[derive(Component)]
pub struct MovingCharacter;

#[derive(Component, Default)]
pub struct Momentum(f32);

impl Momentum {
    pub fn has_momentum(&self) -> bool {
        self.0 != 0.0
    }

    pub fn reset(&mut self) {
        self.0 = 0.0;
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, momentum: f32) {
        self.0 = momentum;
    }

    pub fn add(&mut self, momentum: f32) {
        self.0 += momentum;
    }
}
