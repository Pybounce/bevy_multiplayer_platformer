use bevy::prelude::*;
use stage_goal::{check_goal_reached, next_staged_if_goal_reached, GoalReached};

use crate::common::states::{AppState, GameState};

pub mod player;
pub mod stage_goal;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_systems(Update, (check_goal_reached, next_staged_if_goal_reached).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Playing)));
        //.add_systems(OnEnter(StageState::Loading), (save_stage, despawn_stage, load_stage_handles).chain())
        //.add_systems(Update, spawn_stage_vec.run_if(in_state(StageState::Loading)))
        //.add_systems(OnEnter(StageState::Loaded), reset_players)
        //.add_systems(OnExit(StageState::Loaded), despawn_stage)
        //.add_systems(Update, (check_grounded, respawn_dead_players, check_goal_reached, next_staged_if_goal_reached)
        //    .run_if(in_state(AppState::Game))
        //    .run_if(in_state(GameState::Playing)));
    }
}



