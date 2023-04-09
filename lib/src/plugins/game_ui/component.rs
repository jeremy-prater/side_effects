use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AttackButton(pub bool);

#[derive(Component, Default)]

pub struct InspectButton(pub bool);

#[derive(Component, Default)]

pub struct MoveButton(pub bool);

#[derive(Component, Default)]
pub struct PickButton(pub bool);

#[derive(Component)]
pub struct HealthText;
