use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use kayak_ui::prelude::*;

#[derive(Component)]
pub struct RadialMenu {
    pub world_pos: Vect,
}
