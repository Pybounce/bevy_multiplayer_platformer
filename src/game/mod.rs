use bevy::prelude::*;
use player::player_states::{reset_players, respawn_dead_players};
use stage_asset_loader::StageLoaderPLugin;
use stage_builder::{despawn_stage, load_stage_handles, spawn_stage_vec, StageAssetLoadingHandles};
use stage_goal::{check_goal_reached, GoalReached};
use stage_manager::next_staged_if_goal_reached;

use crate::{common::states::{AppState, GameState, StageState}, stage_1::check_grounded};
mod stage_builder;
pub mod player;
pub mod stage_goal;
pub mod stage_manager;
mod stage_asset_loader;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(StageLoaderPLugin)
        .add_event::<GoalReached>()
        .init_resource::<StageAssetLoadingHandles>();
        //.add_systems(OnEnter(StageState::Loading), (save_stage, despawn_stage, load_stage_handles).chain())
        //.add_systems(Update, spawn_stage_vec.run_if(in_state(StageState::Loading)))
        //.add_systems(OnEnter(StageState::Loaded), reset_players)
        //.add_systems(OnExit(StageState::Loaded), despawn_stage)
        //.add_systems(Update, (check_grounded, respawn_dead_players, check_goal_reached, next_staged_if_goal_reached)
        //    .run_if(in_state(AppState::Game))
        //    .run_if(in_state(GameState::Playing)));
    }
}



