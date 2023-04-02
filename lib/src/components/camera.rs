use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera {
    pub rotation: f32,
    pub offset: Vec3,
    pub easing: f32,
    pub look_target: Vec3,
    pub desired_position: Vec3,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            rotation: 0.0,
            offset: Vec3::new(0.0, 5.0, -10.0),
            easing: 10.0,
            look_target: Vec3::default(),
            desired_position: Vec3::default(),
        }
    }
}
