use bevy::prelude::*;

use crate::common::states::{AppState, GameState, StageData};


pub struct StageSelectPlugin;

impl Plugin for StageSelectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageSelect), display_stage_select)
        .add_systems(Update, try_enter_stage.run_if(in_state(AppState::StageSelect)));
    }
}




pub fn display_stage_select() {

}

pub fn try_enter_stage(
    input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands
) {
    if input.just_released(KeyCode::Space) {
        commands.insert_resource(StageData {stage_id: 0, respawn_translation: Vec3::default()});
        app_state.set(AppState::Game);
        game_state.set(GameState::Transitioning);
    }
}