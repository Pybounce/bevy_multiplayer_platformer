use bevy::prelude::*;

use crate::common::states::{GameState, StageTransitionData};

use super::stage_goal::GoalReached;

/* 

pub fn load_next_stage_if_goal_reached


plugin for stages

load stage event
load next stage event that looks for stage data, and sends to menu if all stages done */


#[derive(Resource)]
pub struct StageData {
    pub stage_id: usize,
    pub respawn_translation: Vec3
}

pub fn next_staged_if_goal_reached(
    stage_data: Res<StageData>,
    mut stage_transition_data: ResMut<StageTransitionData>,
    mut event_reader: EventReader<GoalReached>,
    mut game_state: ResMut<NextState<GameState>>,

) {

    for event in event_reader.read() {
        if event.stage_id == stage_data.stage_id {
            stage_transition_data.target_stage_id = stage_data.stage_id + 1;
            game_state.set(GameState::Transitioning);
            break;
        }
    }

}