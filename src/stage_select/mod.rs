use bevy::prelude::*;

use crate::{common::states::AppState, stage::stage_builder::events::{BuildStageEvent, LoadStageEvent}};


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
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>
) {
    if input.just_released(KeyCode::Space) {
        load_event_writer.send(LoadStageEvent {stage_id: 0});
        build_event_writer.send(BuildStageEvent {stage_id: 0});
    }
}
