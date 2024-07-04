use bevy::prelude::*;

use crate::common::states::{AppState, GameState, StageState, StageTransitionData};


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
    mut stage_state: ResMut<NextState<StageState>>,
    mut commands: Commands
) {
    if input.just_released(KeyCode::Space) {
        commands.insert_resource(StageTransitionData {target_stage_id: 0});
        app_state.set(AppState::Game);
        stage_state.set(StageState::Loading);
    }
}
