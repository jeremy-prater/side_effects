use crate::events::animation::AnimationTransitionEvent;
use crate::resources::animation::AnimationLibrary;
use crate::systems::animation::*;
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationLibrary::default())
            .add_event::<AnimationTransitionEvent>()
            .add_startup_system(setup_animation_library)
            .add_system(read_animation_events)
            .add_system(transfer_animations)
            .add_system(assign_animation_controllers);
    }
}
