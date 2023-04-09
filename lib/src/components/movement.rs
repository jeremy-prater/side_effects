use bevy::prelude::*;

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
