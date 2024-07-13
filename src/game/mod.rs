use bevy::prelude::*;
use player::player_states::spawn_player_on_stage_built;
use stage_goal::{check_goal_reached, next_staged_if_goal_reached, GoalReached};

use crate::common::states::{AppState, GameState};

pub mod player;
pub mod stage_goal;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_systems(Update, (check_goal_reached, next_staged_if_goal_reached).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Playing)))
        .add_systems(Update, spawn_player_on_stage_built);
    }
}



