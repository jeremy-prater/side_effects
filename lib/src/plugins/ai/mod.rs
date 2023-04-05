use bevy::prelude::*;
use big_brain::prelude::*;

pub mod components;
pub mod systems;

use components::{
    MoveAbsoluteScorer, MoveRelativeScorer, MoveToAbsolute, MoveToRelative, TanukiJob,
};
use systems::{
    init_unassigned_tanuki_group, move_to_absolute_execution, move_to_absolute_scoring,
    move_to_relative_execution, move_to_relative_scoring,
};

use super::GameState;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_system(init_unassigned_tanuki_group.in_schedule(OnEnter(GameState::InGame)))
            .add_system(move_to_relative_execution.in_set(BigBrainSet::Actions))
            .add_system(move_to_absolute_execution.in_set(BigBrainSet::Actions))
            .add_system(move_to_relative_scoring.in_set(BigBrainSet::Scorers))
            .add_system(move_to_absolute_scoring.in_set(BigBrainSet::Scorers));
    }
}

/// A bundle meant to be given to groups of tanuki assigned on the same job. Ie. the entitiy whose
/// childern are the individual tanuki each of which are assigned to the same [`TanukiJob`].
///
/// An entity with this component is expected to have [`Children`] which are individual tanuki
/// followers. This entity is expected to have a [`SpatialBundle`].
#[derive(Bundle)]
pub struct FollowerJobBundle {
    pub job: TanukiJob,
}

/// A bundle meant to be given to individual tanuki.
#[derive(Bundle)]
pub struct TanukiAiBundle {
    thinker: ThinkerBuilder,
}

impl Default for TanukiAiBundle {
    fn default() -> Self {
        TanukiAiBundle {
            thinker: Thinker::build()
                .label("tanuki_ai")
                .picker(Highest)
                .when(MoveRelativeScorer, MoveToRelative { speed: 2.0 })
                .when(MoveAbsoluteScorer, MoveToAbsolute { speed: 2.0 }),
        }
    }
}
