use crate::components::animation::{AnimationController, AnimationMarker};
use crate::events::animation::AnimationTransitionEvent;
use crate::resources::animation::AnimationLibrary;
use bevy::prelude::*;

pub fn setup_animation_library(
    asset_server: Res<AssetServer>,
    mut animation_library: ResMut<AnimationLibrary>,
) {
    animation_library.insert(
        "tanuki",
        "idle",
        asset_server.load("models/tanuki.glb#Animation0"),
    );
    animation_library.insert(
        "tanuki",
        "run",
        asset_server.load("models/tanuki.glb#Animation1"),
    );
}
pub fn read_animation_events(
    mut animation_transition_reader: EventReader<AnimationTransitionEvent>,
    mut animation_controller_query: Query<&mut AnimationController>,
) {
    for event in animation_transition_reader.iter() {
        for mut controller in &mut animation_controller_query {
            if controller.parent_entity_id == event.entity_id {
                if controller.current_clip != event.animation_name {
                    controller.current_clip = event.animation_name.clone();
                }
            }
        }
    }
}

pub fn transfer_animations(
    animation_library: Res<AnimationLibrary>,
    mut query: Query<
        (&AnimationController, &mut AnimationPlayer),
        Or<(Changed<AnimationController>, Added<AnimationController>)>,
    >,
) {
    for (controller, mut player) in &mut query {
        player
            .play(
                animation_library
                    .get(
                        &controller.animation_collection_name,
                        &controller.current_clip,
                    )
                    .unwrap(),
            )
            .repeat();
    }
}

pub fn assign_animation_controllers(
    mut commands: Commands,
    marker_query: Query<(Entity, &AnimationMarker)>,
    child_query: Query<&Children>,
    animation_player_query: Query<Entity, (With<AnimationPlayer>, Without<AnimationController>)>,
) {
    if !animation_player_query.is_empty() {
        println!("Attached Animation Controller");
        for (entity, marker) in &marker_query {
            for anim_entity in &animation_player_query {
                for descendant in child_query.iter_descendants(entity) {
                    if descendant == anim_entity {
                        commands.entity(anim_entity).insert(AnimationController {
                            parent_entity_id: entity,
                            animation_collection_name: marker.collection.clone(),
                            current_clip: marker.starting_clip.clone(),
                        });
                    }
                }
            }
        }
    }
}
