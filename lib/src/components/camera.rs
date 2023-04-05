use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct MainCameraTarget;

#[derive(Component)]
pub struct MainCamera {
    pub target: Option<Entity>,
    pub focus: Vec3,
    distance: f32,
    pitch: f32,
    yaw: f32,
    pub easing: f32,
}

impl MainCamera {
    const MIN_DISTANCE: f32 = 5.0;
    const MAX_DISTANCE: f32 = 25.0;
    const MAX_PITCH: f32 = 89.9 / 180.0 * PI;
    const MIN_PITCH: f32 = 5.0 / 180.0 * PI;
    const MAX_YAW: f32 = PI;
    const MIN_YAW: f32 = -Self::MAX_YAW;

    pub fn new(
        target: Option<Entity>,
        mut distance: f32,
        pitch: f32,
        yaw: f32,
        easing: f32,
    ) -> Self {
        distance = distance
            .max(f32::EPSILON)
            .max(Self::MIN_DISTANCE)
            .min(Self::MAX_DISTANCE);
        let focus = Vec3::default();
        Self {
            target,
            focus,
            distance,
            pitch,
            yaw,
            easing,
        }
    }
    pub fn set_focus(&mut self, focus: Vec3) -> &mut Self {
        self.focus = focus;
        self
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
    pub fn set_distance(&mut self, distance: f32) -> &mut Self {
        self.distance = distance
            .max(f32::EPSILON)
            .max(Self::MIN_DISTANCE)
            .min(Self::MAX_DISTANCE);
        self
    }
    pub fn add_distance(&mut self, distance: f32) -> &mut Self {
        self.set_distance(self.distance() + distance);
        self
    }
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
    pub fn set_pitch(&mut self, pitch: f32) -> &mut Self {
        self.pitch = pitch.max(Self::MIN_PITCH).min(Self::MAX_PITCH);
        self
    }
    pub fn add_pitch(&mut self, pitch: f32) -> &mut Self {
        self.set_pitch(self.pitch() + pitch);
        self
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
    }
    pub fn set_yaw(&mut self, yaw: f32) -> &mut Self {
        self.yaw = Self::wrap(yaw, Self::MIN_YAW, Self::MAX_YAW);
        self
    }
    pub fn add_yaw(&mut self, yaw: f32) -> &mut Self {
        self.set_yaw(self.yaw() + yaw);
        self
    }
    pub fn position(&self) -> Vec3 {
        self.focus + Self::calculate_relative_position(self.pitch, self.yaw, self.distance)
    }
    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }
    fn wrap(num: f32, min: f32, max: f32) -> f32 {
        if num < min {
            Self::wrap(max - (min - num), min, max)
        } else if num > max {
            Self::wrap(min - (max - num), min, max)
        } else {
            num
        }
    }
    fn calculate_relative_position(pitch: f32, yaw: f32, distance: f32) -> Vec3 {
        let point = Vec3::new(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        );
        assert!(point.is_normalized());
        point * distance
    }
}
