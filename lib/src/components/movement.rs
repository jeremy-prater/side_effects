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
}
