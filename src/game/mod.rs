use bevy::prelude::*;
use player::player_states::{reset_players, respawn_dead_players};
use stage_builder::{despawn_stage, spawn_stage_vec};
use stage_goal::{check_goal_reached, GoalReached};
use stage_manager::next_staged_if_goal_reached;

use crate::{common::states::{AppState, GameState}, stage_1::check_grounded};
mod stage_builder;
pub mod player;
pub mod stage_goal;
mod stage_manager;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_systems(OnEnter(GameState::Transitioning), (despawn_stage, spawn_stage_vec, reset_players).chain()
    )//.run_if(in_state(AppState::Game)))
        .add_systems(Update, (check_grounded, respawn_dead_players, (check_goal_reached, next_staged_if_goal_reached).chain())
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Playing)));
    }
}



