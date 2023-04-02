use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationMarker {
    pub collection: String,
    pub starting_clip: String,
}

impl AnimationMarker {
    pub fn new(collection: &str, starting_clip: &str) -> Self {
        AnimationMarker {
            collection: collection.to_string(),
            starting_clip: starting_clip.to_string(),
        }
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub parent_entity_id: Entity,
    pub animation_collection_name: String,
    pub current_clip: String,
}
