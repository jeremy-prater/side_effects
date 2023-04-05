//! This module is dedicated to [`Component`]s which are used in running the AI of both follower
//! tanuki as well as for entities with AI outside of the user's control.

use bevy::prelude::*;
use big_brain::prelude::*;

/// With tankui, it is important to understand that they are given "jobs" (this word is chosen to
/// avoid overloading "command" or "task") in groups. Ie each job may have any natural number of
/// tanuki assigned to it. This is modeled with the Bevy hierarchies which use the [`Parent`] and
/// [`Children`] components.
///
/// As such, we need an entity to be the [`Parent`] of the tanuki entities. Such an entity will be
/// marked by this component, which denotes the variant of job that the entity represents.
///
/// In working with the AI systems, tanuki will consider their parent job when making decisions.
#[derive(Component)]
pub enum TanukiJob {
    /// Command to move to a point relative to the player's postion.
    MoveRelative(Vec3),

    /// Command to move to a global position irregardless of the player's position. However there
    /// will be a maximum range that this group may be from the player.
    MoveAbsolute(Vec3),

    /// Command to search for mushrooms around an absolute point on the map. Like `MoveRelative`,
    /// there is a maximum range that the point may be placed from the player.
    SearchMushrooms(Vec3),

    /// Command to attack the included entitiy.
    Attack(Entity),

    /// Command to interact with the included entity in some capacity.
    Interact(Entity),
}

/// A marker trait for the [`TanukiJob`]-containing entity that denotes the "default group" of
/// tankui. This is the group of unassigned tanuki which simply follow the player.
#[derive(Component)]
pub struct UnassignedTanukiJob;

/// This component is a "scorer" which serves to provide a priority of a given action with values
/// between 0.0 and 1.0.
///
/// This component provides the score for whether the [`MoveToRelative`] should be the active action
/// by setting the score 0.7 if the actor's parent has a [`TanukiJob::MoveRelative`], and 0.0
/// otherwise.
///
/// 0.7 is chosen instead of 1.0 to allow "instincts" such as avoiding predators.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct MoveRelativeScorer;

/// This component is the "move to point (x, y, z) relative to the player" action as used by any
/// pathfinding AI.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToRelative {
    pub speed: f32,
}

/// This component is a "scorer" which serves to provide a priority of a given action with values
/// between 0.0 and 1.0.
///
/// This component provides the score for whether the [`MoveToAbsolute`] should be the active action
/// by setting the score 0.7 if the actor's parent has a [`TanukiJob::MoveAbsolute`], and 0.0
/// otherwise.
///
/// 0.7 is chosen instead of 1.0 to allow "instincts" such as avoiding predators.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct MoveAbsoluteScorer;

/// This component is the "move to point (x, y, z)" action as used by any pathfinding AI.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToAbsolute {
    pub speed: f32,
}
