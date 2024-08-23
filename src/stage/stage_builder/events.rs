use bevy::prelude::*;

use crate::{common::states::{AppState, GameState}, sdf::{buffer_garbage::BufferBullshit, enums::SDFShapeID, ground_material::GroundMaterial, test::CustomMaterial}};

use super::{stage_creator::TILE_SIZE, StageBuilderData, StageBuilderState};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadStageEvent {
    pub stage_id: usize
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct BuildStageEvent {
    pub stage_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildCompleteEvent {
    pub stage_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildFailedEvent {
    pub stage_id: usize
}


pub fn read_stage_build_complete_events(
    mut event_reader: EventReader<StageBuildCompleteEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
) {
    for _ in event_reader.read() {
        game_state.set(GameState::Playing);
        app_state.set(AppState::Game);
        stage_builder_state.set(StageBuilderState::NotBuilding);
    }
}

pub fn read_stage_build_failed_events(
    mut event_reader: EventReader<StageBuildFailedEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
) {
    for _ in event_reader.read() {
        game_state.set(GameState::NA);
        app_state.set(AppState::StageSelect);
        stage_builder_state.set(StageBuilderState::NotBuilding);
    }
}

/// Listens for LoadStageEvent.</br>
/// Begins loading the stage asset.</br>
/// Adds handle to StageBuilderHandles
pub fn read_stage_load_events(
    mut event_reader: EventReader<LoadStageEvent>,
    mut stage_builder_data: ResMut<StageBuilderData>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for preload_event in event_reader.read() {
        stage_builder_data.stage_id = preload_event.stage_id;
        stage_builder_data.stage_handle = asset_server.load(format!("stage_{}.stage", preload_event.stage_id));
        stage_builder_data.tile_mesh_handle = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE)).into();
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
