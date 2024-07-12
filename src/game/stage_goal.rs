
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{local_player::LocalPlayer, stage::{stage_builder::{events::{BuildStageEvent, LoadStageEvent}, CurrentStageData}, stage_objects::goal::StageGoal}};

#[derive(Event)]
pub struct GoalReached {
    pub stage_id: usize
}


pub fn check_goal_reached(
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    goal_query: Query<(), With<StageGoal>>,
    mut event_writer: EventWriter<GoalReached>,
    stage_data: Res<CurrentStageData>,
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>
) {
    for colliding_entities in &player_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = goal_query.get(colliding_entity) {
                load_event_writer.send(LoadStageEvent {stage_id: stage_data.stage_id + 1});
                build_event_writer.send(BuildStageEvent {stage_id: stage_data.stage_id + 1});
                return;
                //event_writer.send(GoalReached { stage_id: stage_data.stage_id });
            }
        }
    }
}