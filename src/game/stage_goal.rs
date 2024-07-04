
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;

#[derive(Event)]
pub struct GoalReached;

#[derive(Component)]
pub struct StageGoal;

pub fn check_goal_reached(
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    goal_query: Query<(), With<StageGoal>>,
    mut event_writer: EventWriter<GoalReached>,
) {
    for colliding_entities in &player_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = goal_query.get(colliding_entity) {
                error!("GOAL WRITE");
                event_writer.send(GoalReached);
            }
        }
    }
}