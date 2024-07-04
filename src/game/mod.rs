use bevy::prelude::*;
use player::player_states::respawn_dead_players;
use stage_builder::{despawn_stage, spawn_stage_vec};

use crate::{common::states::{AppState, GameState}, stage_1::check_grounded};
mod stage_builder;
pub mod player;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Transitioning), (despawn_stage, spawn_stage_vec).chain()
    )//.run_if(in_state(AppState::Game)))
        .add_systems(Update, (check_grounded, respawn_dead_players)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Playing)));
    }
}



