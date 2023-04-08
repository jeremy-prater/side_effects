//! This module contains behavior related to the "selection" of [`Entity`] instances with the mouse,
//! which is primarily intended to be used in selecting which "follower tanuki" are to be ordered to
//! do a given task.
//!
//! # Interface
//!
//! [`Selectable`] -- A marker component designating an entity that may be selected. In order for
//! selections to function as intended, possible selections must all have this component.
//!
//! [`Selected`] -- Another marker component which is intended to designate members of the active
//! selection. This is managed automatically using the [`SelectionPlugin`].
//!
//! [`SelectionControlEvent`] -- An event type which must be emit in order to control the selection
//! process.
//!
//! # How to use
//!
//!   1. Marke relevant entities to [`Selectable`]
//!   2. Emit the desired [`SelectionControlEvent`]s based on player input
//!   3. Query for [`Entity`] instances [`With`] the [`Selected`] component
//!
//! # Improvements
//
//!  * Box selection eg by dragging the GlobalTransform), With<MainCamera>>,

use bevy::{prelude::*, scene::SceneInstance, utils::HashSet, window::PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_mod_outline::{
    AutoGenerateOutlineNormalsPlugin, OutlineBundle, OutlinePlugin, OutlineVolume,
};
use bevy_rapier3d::prelude::*;

use super::{camera::component::MainCamera, GameState};

/// See the module docs for more information.
pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveSelection(HashSet::new()))
            .add_event::<SelectionControlEvent>()
            .add_system(handle_selections.in_set(OnUpdate(GameState::InGame)));
    }
}

pub struct DebugSelectionInputPlugin;
impl Plugin for DebugSelectionInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OutlinePlugin)
            .add_plugin(AutoGenerateOutlineNormalsPlugin)
            .add_system(debug_replace_extend_input.in_set(OnUpdate(GameState::InGame)))
            .add_system(debug_highlight.in_set(OnUpdate(GameState::InGame)));
    }
}

fn debug_replace_extend_input(
    mut input_events: EventWriter<SelectionControlEvent>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl) {
            input_events.send(SelectionControlEvent::ExtendSelectionWithHovered);
        } else {
            input_events.send(SelectionControlEvent::ReplaceSelectionWithHovered(true));
        }
    }
}

fn debug_highlight(
    scene_query: Query<&SceneInstance>,
    outline_query: Query<Entity, With<OutlineVolume>>,
    scene_manager: Res<SceneSpawner>,
    active_selection: Res<ActiveSelection>,
    mut commands: Commands,
) {
    if active_selection.is_changed() {
        for outlined in outline_query.iter() {
            commands.entity(outlined).remove::<OutlineBundle>();
        }

        for sel in active_selection.0.iter() {
            if let Ok(scene) = scene_query.get(*sel) {
                for entity in scene_manager.iter_instance_entities(**scene) {
                    commands.entity(entity).insert(OutlineBundle {
                        outline: OutlineVolume {
                            visible: true,
                            width: 3.0,
                            colour: Color::FUCHSIA,
                        },
                        ..Default::default()
                    });
                }
            }

            commands.entity(*sel).insert(OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    width: 3.0,
                    colour: Color::FUCHSIA,
                },
                ..Default::default()
            });
        }
    }
}

#[derive(Resource)]
struct ActiveSelection(HashSet<Entity>);

/// Whether to allow an [`Entity`] to be automatically be (un)assigned the [`Selected`] component
/// upon the mouse hovering over that [`Entity`] and a relevant [`SelectionControlEvent`] being
/// fired.
#[derive(Component)]
pub struct Selectable;

/// Whether an [`Entity`] is in the active selection.
///
/// XXX: Please do not add or remove this copmonent manually. It is managed by the plugin's systems.
/// If you need to assume "direct control", use [`SelectionControlEvent::DirectInsert`] event or its
/// counterpart [`SelectionControlEvent::DirectRemove`].
#[derive(Component)]
pub struct Selected;

/// The events used to control the [`SelectionPlugin`]. Meant to be a thin mapping from user input
/// to the control options, such as to allow any control scheme.
pub enum SelectionControlEvent {
    /// Remove [`Selected`] from all [`Entity`] instances that currently have it
    ClearSelection,

    /// Remove [`Selected`] from all [`Entity`] instances that currently have it, then adding the
    /// component to the currently hovered [`Selectable`], effectively replacing the old selection.
    ///
    /// If no [`Selectable`] is under the mouse, and the inner boolean is false, this is a NOOP.
    ReplaceSelectionWithHovered(bool),

    /// Add [`Selected`] to the currently hovered [`Selectable`]. This does not affect other
    /// [`Entity`] insances that are [`Selected`], so it can be thought of as "extending" the
    /// selection with the hovered [`Entity`].
    ExtendSelectionWithHovered,

    /// Remove [`Selected`] from the currently hovered [`Selectable`]. This does not affect another
    /// [`Entity`]0insances, so it can be thought of as "deselecting" the one, hovered entity.
    RemoveHoveredFromSelection,

    /// Directly tracks a new [`Entity`] marking it with [`Selected`].
    ///
    /// NOTE: This does not perform any check that the given [`Entity`] is selectable.
    DirectInsert(Entity),

    /// Directly untracks an [`Entity`] should it have been [`Selected`], removing [`Selected`] in
    /// the process.
    DirectRemove(Entity),
}

#[allow(clippy::too_many_arguments)]
fn handle_selections(
    mut input_events: EventReader<SelectionControlEvent>,

    mut egui_contexts: EguiContexts,

    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    is_selectable: Query<&Selectable>,

    rapier_contex: Res<RapierContext>,
    mut active_selection: ResMut<ActiveSelection>,

    mut commands: Commands,
) {
    let mut hovered = || {
        if egui_contexts.ctx_mut().wants_pointer_input() {
            return None;
        }

        let window = window.single();

        let cursor_pos = window.cursor_position()?;
        let (camera, camera_transform) = camera.single();

        let ray = camera.viewport_to_world(camera_transform, cursor_pos)?;

        Some(
            rapier_contex
                .cast_ray(
                    ray.origin,
                    ray.direction,
                    f32::MAX,
                    true,
                    QueryFilter::exclude_kinematic().exclude_sensors(),
                )?
                .0,
        )
    };

    let drain = |active_selection: &mut ResMut<ActiveSelection>, commands: &mut Commands| {
        active_selection.0.drain().for_each(|e| {
            commands.entity(e).remove::<Selected>();
        });
    };

    let extend = |entity: Entity,
                  active_selection: &mut ResMut<ActiveSelection>,
                  commands: &mut Commands| {
        active_selection.0.insert(entity);
        commands.entity(entity).insert(Selected);
    };

    let remove_individual = |entity: Entity,
                             active_selection: &mut ResMut<ActiveSelection>,
                             commands: &mut Commands| {
        if active_selection.0.remove(&entity) {
            commands.entity(entity).remove::<Selected>();
        }
    };

    for input_event in input_events.iter() {
        match input_event {
            SelectionControlEvent::ClearSelection => drain(&mut active_selection, &mut commands),

            SelectionControlEvent::ReplaceSelectionWithHovered(deselect_on_none) => {
                if let Some(entity) = hovered() {
                    if is_selectable.get(entity).is_err() {
                        if *deselect_on_none {
                            drain(&mut active_selection, &mut commands);
                        }

                        return;
                    }

                    drain(&mut active_selection, &mut commands);
                    extend(entity, &mut active_selection, &mut commands);
                } else if *deselect_on_none {
                    drain(&mut active_selection, &mut commands);
                }
            }

            SelectionControlEvent::ExtendSelectionWithHovered => {
                if let Some(entity) = hovered() {
                    if is_selectable.get(entity).is_err() {
                        return;
                    }

                    extend(entity, &mut active_selection, &mut commands);
                }
            }

            SelectionControlEvent::RemoveHoveredFromSelection => {
                if let Some(entity) = hovered() {
                    if is_selectable.get(entity).is_err() {
                        return;
                    }

                    remove_individual(entity, &mut active_selection, &mut commands);
                }
            }

            SelectionControlEvent::DirectInsert(entity) => {
                extend(*entity, &mut active_selection, &mut commands);
            }

            SelectionControlEvent::DirectRemove(entity) => {
                remove_individual(*entity, &mut active_selection, &mut commands);
            }
        }
    }
}
