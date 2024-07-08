use bevy::prelude::*;

use super::{StageBuilderData, StageBuilderState};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadStageEvent {
    pub stage_id: usize
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct BuildStageEvent {
    pub stage_id: usize
}




/// Listens for LoadStageEvent.</br>
/// Begins loading the stage asset.</br>
/// Adds handle to StageBuilderHandles
pub fn read_stage_load_events(
    mut event_reader: EventReader<LoadStageEvent>,
    mut stage_builder_data: ResMut<StageBuilderData>,
    asset_server: Res<AssetServer>,
) {
    for preload_event in event_reader.read() {
        stage_builder_data.stage_id = preload_event.stage_id;
        stage_builder_data.stage_handle = asset_server.load(format!("stage_{}.stage", preload_event.stage_id));
    }
}

/// REQUIRES STAGE LOAD EVENT RAISED </br>
/// Listens for BuildStageEvent. </br>
/// Sets the StageBuilderState to building.
/// (which in turn begins the building of the stage)
pub fn read_stage_build_events(
    mut event_reader: EventReader<BuildStageEvent>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,

) {
    for _ in event_reader.read() {
        stage_builder_state.set(StageBuilderState::Building);
    }
}
