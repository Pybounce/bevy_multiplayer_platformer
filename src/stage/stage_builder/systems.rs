use bevy::prelude::*;

use crate::stage::stage_objects::StageObject;

use super::{events::{StageBuildCompleteEvent, StageBuildFailedEvent}, stage_asset::Stage, stage_creator::StageCreator, CurrentStageData, StageBuilderData};


pub fn unload_old_stage(
    stage_piece_query: Query<(Entity, &StageObject)>,
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage_builder_data: Res<StageBuilderData>,
    stage_assets: Res<Assets<Stage>>,
    mut current_stage_data: ResMut<CurrentStageData>,
    mut complete_event_writer: EventWriter<StageBuildCompleteEvent>,
    mut failed_event_writer: EventWriter<StageBuildFailedEvent>
) {
    match asset_server.load_state(&stage_builder_data.stage_handle) {
        bevy::asset::LoadState::NotLoaded => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
    }

    let stage_asset = stage_assets.get(&stage_builder_data.stage_handle);
    let texture_handle: Handle<Image> = asset_server.load("colour_palettes.png");

    match stage_asset {
        Some(stage) => {
            let stage_creator = StageCreator::new(&stage, &texture_handle);
            if stage_creator.build(&mut commands) {
                current_stage_data.stage_id = stage.id;
                current_stage_data.spawn_translation = stage.spawn_translation;
                complete_event_writer.send(StageBuildCompleteEvent { stage_id: stage_builder_data.stage_id });
            }
            else {
                failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            }
        },
        None => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
    }
}
