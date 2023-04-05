//! This module is dedicated to the systems related to running the AIs of any in-game entitiesGlobalTransform, With<Player>> // The query to get the position of what we're relative to,

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use big_brain::prelude::*;

use crate::components::player::Player;

use super::components::{
    MoveAbsoluteScorer, MoveRelativeScorer, MoveToAbsolute, MoveToRelative, TanukiJob,
    UnassignedTanukiJob,
};

pub fn init_unassigned_tanuki_group(mut commands: Commands) {
    commands
        .spawn((UnassignedTanukiJob, TanukiJob::MoveRelative(Vec3::ZERO)))
        .insert(SpatialBundle::default())
        .insert(Visibility::Hidden);
}

/// An "action system" for any AI which uses the [`MoveToRelative`] action.
pub fn move_to_relative_execution(
    // The query to get actors who are currently doing/about to do the [`MoveToAbsolute`] action
    mut action_query: Query<(&Actor, &MoveToRelative, &mut ActionState)>,
    // The query to access the physics state of whatever entities are in this action
    mut physics_query: Query<(&GlobalTransform, &mut Velocity)>,
    // The query to get the position of what we're relative to
    player_position_query: Query<&GlobalTransform, With<Player>>,

    // The query to get the [`Parent`] of the actor in order to find the destination of the movement
    parent_query: Query<&Parent>,
    // The query to get the [`TanukiJob`] of the [`Parent`] of the actor which includes the point of
    // the destination.
    parent_job_query: Query<&TanukiJob>,
) {
    // For every [`Actor`] whse current action is [`MoveToRelative`].
    //
    // Additonally fetches their [`ActionState`] which records the progression of a given action, such
    // as to be able to record whether the action is still in progress, if it failed, or if it succeded
    for (Actor(actor), move_cmd, mut action_state) in &mut action_query {
        error!("HERE 1");
        if let Ok(parent_entity) = parent_query.get(*actor) {
            error!("HERE 2");
            if let Ok(TanukiJob::MoveRelative(target)) = parent_job_query.get(parent_entity.get()) {
                error!("HERE 3");
                if let Ok((transform, mut velocity)) = physics_query.get_mut(*actor) {
                    error!("HERE 4");
                    if let Ok(player_transform) = player_position_query.get_single() {
                        error!("HERE_5");
                        // If there is movent necessary, then move.
                        let target = player_transform.translation() + *target;
                        if transform.translation() != target {
                            // First some bookkeeping on state
                            match *action_state {
                                ActionState::Requested => *action_state = ActionState::Executing,
                                ActionState::Executing => {}
                                ActionState::Cancelled => {
                                    *action_state = ActionState::Failure;
                                    return;
                                }
                                ActionState::Init | ActionState::Failure | ActionState::Success => {
                                    return
                                }
                            };

                            // Now alter some velocity
                            let between = target - transform.translation();
                            let mov_dir = between.normalize();
                            let lin_vel = between.min(mov_dir * move_cmd.speed);

                            velocity.linvel = lin_vel;

                            continue;
                        } else {
                            // If there is no movement necessary, it's a success.
                            *action_state = ActionState::Success;
                            continue;
                        }
                    }
                }
            }
        }

        // If any of those conditions fail, then something is up with the state, so call it a failure
        *action_state = ActionState::Failure;
    }
}

/// An "action system" for any AI which uses the [`MoveToAbsolute`] action.
pub fn move_to_absolute_execution(
    // The query to get actors who are currently doing/about to do the [`MoveToAbsolute`] action
    mut action_query: Query<(&Actor, &MoveToAbsolute, &mut ActionState)>,
    // The query to access the physics state of whatever entities are in this action
    mut physics_query: Query<(&GlobalTransform, &mut Velocity)>,
    // The query to get the [`Parent`] of the actor in order to find the destination of the movement
    parent_query: Query<&Parent>,
    // The query to get the [`TanukiJob`] of the [`Parent`] of the actor which includes the point of
    // the destination.
    parent_job_query: Query<&TanukiJob>,
) {
    // For every [`Actor`] whse current action is [`MoveToAbsolute`].
    //
    // Additonally fetches their [`ActionState`] which records the progression of a given action, such
    // as to be able to record whether the action is still in progress, if it failed, or if it succeded
    for (Actor(actor), move_cmd, mut action_state) in &mut action_query {
        if let Ok(parent_entity) = parent_query.get(*actor) {
            if let Ok(TanukiJob::MoveAbsolute(target)) = parent_job_query.get(parent_entity.get()) {
                if let Ok((transform, mut velocity)) = physics_query.get_mut(*actor) {
                    // If there is movent necessary, then move.
                    if transform.translation() != *target {
                        // First some bookkeeping on state
                        match *action_state {
                            ActionState::Requested => *action_state = ActionState::Executing,
                            ActionState::Executing => {}
                            ActionState::Cancelled => {
                                *action_state = ActionState::Failure;
                                return;
                            }
                            ActionState::Init | ActionState::Failure | ActionState::Success => {
                                return
                            }
                        };

                        // Now alter some velocity
                        let between = *target - transform.translation();
                        let mov_dir = between.normalize();
                        // If the actual distance between the points is smaller than the max speed, then use
                        // the actual difference. That would lead to a gradual slowdown until the difference
                        // is small enough for the positions to be deemed equal.
                        //
                        // Not sure how nice it'll work in practice, but this is still a FIXME point.
                        let lin_vel = between.min(mov_dir * move_cmd.speed);

                        velocity.linvel = lin_vel;

                        continue;
                    } else {
                        // If there is no movement necessary, it's a success.
                        *action_state = ActionState::Success;
                        continue;
                    }
                }
            }
        }

        // If any of those conditions fail, then something is up with the state, so call it a failure
        *action_state = ActionState::Failure;
    }
}

pub fn move_to_relative_scoring(
    // The query to get actors who need a move absolute score.
    mut score_query: Query<(&Actor, &mut Score), With<MoveRelativeScorer>>,
    // The query to fetch the parent associated with the actors above
    parent_query: Query<&Parent>,
    // The query to fetch the task details from the parent above
    parent_job_query: Query<&TanukiJob>,
) {
    for (Actor(actor), mut score) in &mut score_query {
        if let Ok(parent_entity) = parent_query.get(*actor) {
            if let Ok(TanukiJob::MoveRelative(move_abs_vec)) =
                parent_job_query.get(parent_entity.get())
            {
                score.set(0.7);
                continue;
            }
        }

        score.set(0.0);
    }
}

pub fn move_to_absolute_scoring(
    // The query to get actors who need a move absolute score.
    mut score_query: Query<(&Actor, &mut Score), With<MoveAbsoluteScorer>>,
    // The query to fetch the parent associated with the actors above
    parent_query: Query<&Parent>,
    // The query to fetch the task details from the parent above
    parent_job_query: Query<&TanukiJob>,
) {
    for (Actor(actor), mut score) in &mut score_query {
        if let Ok(parent_entity) = parent_query.get(*actor) {
            if let Ok(TanukiJob::MoveAbsolute(move_abs_vec)) =
                parent_job_query.get(parent_entity.get())
            {
                score.set(0.7);
                continue;
            }
        }

        score.set(0.0);
    }
}
