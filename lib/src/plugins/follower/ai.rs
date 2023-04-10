use std::iter::Peekable;

use bevy::prelude::*;
use big_brain::prelude::*;

use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings,
};

use super::FollowerJob;
use crate::components::movement::{CharacterSpeed, Direction};

pub struct FollowerAiPlugin;
impl Plugin for FollowerAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .insert_resource(AiFrameCounter { count: 0 })
            .add_system(follow_action.in_set(BigBrainSet::Actions))
            .add_system(follow_scoring.in_set(BigBrainSet::Scorers))
            .add_system(move_to_action.in_set(BigBrainSet::Actions))
            .add_system(move_to_scoring.in_set(BigBrainSet::Scorers));
    }
}

#[derive(Clone, Component, Copy, Debug, ScorerBuilder)]
pub struct FollowScorer;

#[derive(ActionBuilder, Clone, Component, Debug)]
pub struct FollowAction {
    pub(super) end_pos: Option<Vec3>,
    pub(super) path_cache: Peekable<std::vec::IntoIter<Vec3>>,
}

#[derive(Clone, Component, Copy, Debug, ScorerBuilder)]
pub struct MoveToScorer;

#[derive(ActionBuilder, Clone, Component, Debug)]
pub struct MoveToAction {
    pub(super) end_pos: Option<Vec3>,
    pub(super) path_cache: Peekable<std::vec::IntoIter<Vec3>>,
}

#[derive(Resource)]
pub struct AiFrameCounter {
    pub count: u8,
}
const AI_FRAME_SKIP: u8 = 32;

impl AiFrameCounter {
    pub fn next(&mut self) -> bool {
        let out = self.count == 0;
        self.count += 1;
        if self.count == AI_FRAME_SKIP {
            self.count = 0;
        }

        out
    }
}

pub fn follow_scoring(
    mut score_query: Query<(&Actor, &mut Score), With<FollowScorer>>,
    parent_query: Query<&Parent>,
    parent_job_query: Query<&FollowerJob>,
) {
    for (Actor(actor), mut score) in &mut score_query {
        if let Ok(parent_entity) = parent_query.get(*actor) {
            if let Ok(FollowerJob::Follow(_e)) = parent_job_query.get(parent_entity.get()) {
                score.set(0.7);
                continue;
            }
        }

        score.set(0.0);
    }
}

pub fn follow_action(
    mut action_query: Query<(&Actor, &mut ActionState, &mut FollowAction)>,
    mut actor_query: Query<(
        &mut CharacterSpeed,
        &mut Direction,
        &GlobalTransform,
        &Parent,
    )>,
    target_query: Query<&FollowerJob>,
    followed_transform_query: Query<&GlobalTransform>,

    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut counter: ResMut<AiFrameCounter>,
) {
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (Actor(actor), mut state, mut action) in action_query.iter_mut() {
            match *state {
                ActionState::Requested => {
                    if let Ok((_speed, _dir, start_pos, parent)) = actor_query.get(*actor) {
                        if let Ok(FollowerJob::Follow(entity)) = target_query.get(parent.get()) {
                            if let Ok(end_pos) = followed_transform_query.get(*entity) {
                                if end_pos.translation().distance(start_pos.translation()) < 6.0 {
                                    continue;
                                }

                                match find_path(
                                    &nav_mesh,
                                    &nav_mesh_settings,
                                    start_pos.translation(),
                                    end_pos.translation(),
                                    Some(10.0),
                                    None,
                                ) {
                                    Ok(path) => {
                                        match perform_string_pulling_on_path(
                                            &nav_mesh,
                                            start_pos.translation(),
                                            end_pos.translation(),
                                            &path,
                                        ) {
                                            Ok(path) => {
                                                let mut iter = path.into_iter().peekable();
                                                let _ = iter.next();
                                                action.path_cache = iter;
                                                action.end_pos = Some(end_pos.translation());
                                                *state = ActionState::Executing;
                                            }
                                            Err(e) => error!("Error with string path: {e:?}"),
                                        }
                                    }
                                    Err(e) => error!("Error with pathfinding {e:?}"),
                                }
                            }
                        }
                    }
                }

                ActionState::Executing => {
                    if let Ok((mut char_speed, mut dir, curr_pos, parent)) =
                        actor_query.get_mut(*actor)
                    {
                        if let Ok(FollowerJob::Follow(entity)) = target_query.get(parent.get()) {
                            if let Ok(cmd_end_pos) = followed_transform_query.get(*entity) {
                                if let Some(cached_end_pos) = action.end_pos {
                                    // If the job changed, we should cancel the current action and reasses
                                    if counter.next() && cmd_end_pos.translation() != cached_end_pos
                                    {
                                        *state = ActionState::Requested;
                                    }
                                    if let Some(next) = action.path_cache.peek() {
                                        dir.set(*next - curr_pos.translation());
                                        if next.distance(curr_pos.translation()) < 6.0 {
                                            let _ = action.path_cache.next();
                                        }
                                    } else {
                                        dir.set(Vec3::ZERO);
                                        char_speed.set(0.0);
                                        *state = ActionState::Success;
                                    }
                                } else {
                                    *state = ActionState::Cancelled
                                }
                            }
                        }
                    }
                }

                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }

                _ => {}
            }
        }
    }
}

pub fn move_to_scoring(
    mut score_query: Query<(&Actor, &mut Score), With<MoveToScorer>>,
    parent_query: Query<&Parent>,
    parent_job_query: Query<&FollowerJob>,
) {
    for (Actor(actor), mut score) in &mut score_query {
        if let Ok(parent_entity) = parent_query.get(*actor) {
            if let Ok(FollowerJob::MoveTo(_v)) = parent_job_query.get(parent_entity.get()) {
                score.set(0.7);
                continue;
            }
        }

        score.set(0.0);
    }
}

pub fn move_to_action(
    mut action_query: Query<(&Actor, &mut ActionState, &mut MoveToAction)>,
    mut actor_query: Query<(
        &mut CharacterSpeed,
        &mut Direction,
        &GlobalTransform,
        &Parent,
    )>,
    target_query: Query<&FollowerJob>,

    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
) {
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (Actor(actor), mut state, mut action) in action_query.iter_mut() {
            match *state {
                ActionState::Requested => {
                    if let Ok((_speed, _dir, start_pos, parent)) = actor_query.get(*actor) {
                        if let Ok(FollowerJob::MoveTo(end_pos)) = target_query.get(parent.get()) {
                            if end_pos.distance(start_pos.translation()) < 6.0 {
                                let dist = end_pos.distance(start_pos.translation());
                                continue;
                            }

                            match find_path(
                                &nav_mesh,
                                &nav_mesh_settings,
                                start_pos.translation(),
                                *end_pos,
                                None,
                                None,
                            ) {
                                Ok(path) => {
                                    match perform_string_pulling_on_path(
                                        &nav_mesh,
                                        start_pos.translation(),
                                        *end_pos,
                                        &path,
                                    ) {
                                        Ok(path) => {
                                            action.path_cache = path.into_iter().peekable();
                                            action.end_pos = Some(*end_pos);
                                            *state = ActionState::Executing;
                                        }
                                        Err(e) => error!("Error with string path: {e:?}"),
                                    }
                                }
                                Err(e) => error!("Error with pathfinding {e:?}"),
                            }
                        }
                    }
                }

                ActionState::Executing => {
                    if let Ok((mut char_speed, mut dir, curr_pos, parent)) =
                        actor_query.get_mut(*actor)
                    {
                        if let Ok(FollowerJob::MoveTo(end_pos)) = target_query.get(parent.get()) {
                            if let Some(cached_end_pos) = action.end_pos {
                                // If the job changed, we should cancel the current action and reasses
                                if *end_pos != cached_end_pos {
                                    *state = ActionState::Requested;
                                    continue;
                                } else {
                                    if let Some(next) = action.path_cache.peek() {
                                        dir.set(*next - curr_pos.translation());
                                        if next.distance(curr_pos.translation()) < 6.0 {
                                            let _ = action.path_cache.next();
                                        }
                                    } else {
                                        dir.set(Vec3::ZERO);
                                        char_speed.set(0.0);
                                        *state = ActionState::Success;
                                    }
                                }
                            } else {
                                *state = ActionState::Cancelled
                            }
                        }
                    }
                }

                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }

                _ => {}
            }
        }
    }
}
