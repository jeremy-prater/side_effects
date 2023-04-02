use bevy::prelude::*;

pub struct AnimationTransitionEvent {
    pub entity_id: Entity,
    pub animation_name: String,
}
