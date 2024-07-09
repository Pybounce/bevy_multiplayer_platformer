use bevy::prelude::*;

use super::{events::{StageBuildCompleteEvent, StageBuildFailedEvent}, stage_asset::Stage, StageBuilderData, StagePiece};



pub fn unload_old_stage(
    stage_piece_query: Query<(Entity, &StagePiece)>,
    mut commands: Commands,
    stage_builder_data: Res<StageBuilderData>,
) {
    for (e, sp) in &stage_piece_query {
        if sp.stage_id != stage_builder_data.stage_id {
            commands.entity(e).despawn();
        }
    }
}

pub fn try_build_stage(
    asset_server: Res<AssetServer>,
    stage_builder_data: Res<StageBuilderData>,
    stage_assets: Res<Assets<Stage>>,
    mut complete_event_writer: EventWriter<StageBuildCompleteEvent>,
    mut failed_event_writer: EventWriter<StageBuildFailedEvent>
) {
    match asset_server.load_state(&stage_builder_data.stage_handle) {
        bevy::asset::LoadState::NotLoaded => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
    }

    let stage_asset = stage_assets.get(&stage_builder_data.stage_handle);
    let texture_handle: Handle<Image> = asset_server.load("colour_palettes.png");

    match stage_asset {
        Some(stage) => {
            build_perimeter(&stage, &texture_handle);
            build_background(&stage, &texture_handle);
            

            complete_event_writer.send(StageBuildCompleteEvent { stage_id: stage_builder_data.stage_id });
        },
        None => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
    }
}

fn build_perimeter(stage: &Stage, texture_handle: &Handle<Image>) {

}

fn build_background(stage: &Stage, texture_handle: &Handle<Image>) {

}

fn set_camera_background(texture_handle: &Handle<Image>) {

}