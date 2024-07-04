
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;

use super::stage_manager::StageData;

#[derive(Event)]
pub struct GoalReached {
    pub stage_id: usize
}

#[derive(Component)]
pub struct StageGoal;

pub fn check_goal_reached(
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    goal_query: Query<(), With<StageGoal>>,
    mut event_writer: EventWriter<GoalReached>,
    stage_data: Res<StageData>
) {
    for colliding_entities in &player_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = goal_query.get(colliding_entity) {
                event_writer.send(GoalReached { stage_id: stage_data.stage_id });
            }
        }
    }
}