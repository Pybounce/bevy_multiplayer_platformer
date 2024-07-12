use bevy::prelude::*;
use events::{read_stage_build_complete_events, read_stage_build_events, read_stage_build_failed_events, read_stage_load_events, BuildStageEvent, LoadStageEvent, StageBuildCompleteEvent, StageBuildFailedEvent};
use stage_asset::{Stage, StageLoader};
use stage_asset_creator::save_stage;
use systems::{try_build_stage, unload_old_stage};

pub mod events;
mod stage_asset;
mod systems;
pub mod stage_creator;
mod stage_asset_creator;

pub struct StageBuilderPlugin;

impl Plugin for StageBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<LoadStageEvent>()
        .add_event::<BuildStageEvent>()
        .add_event::<StageBuildCompleteEvent>()
        .add_event::<StageBuildFailedEvent>()
        .init_state::<StageBuilderState>()
        .init_asset::<Stage>()
        .init_asset_loader::<StageLoader>()
        .init_resource::<StageBuilderData>()
        .init_resource::<CurrentStageData>()
        .add_systems(PreUpdate, (read_stage_load_events, read_stage_build_events).chain())
        .add_systems(OnEnter(StageBuilderState::Building), (save_stage, unload_old_stage))
        .add_systems(Update, (try_build_stage).run_if(in_state(StageBuilderState::Building)))
        .add_systems(PostUpdate, (read_stage_build_complete_events, read_stage_build_failed_events));
    }
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum StageBuilderState {
    #[default]
    NotBuilding,
    Building,
}



#[derive(Resource, Default)]
pub struct StageBuilderData {
    stage_id: usize,
    stage_handle: Handle<Stage>
}

#[derive(Resource, Default)]
pub struct CurrentStageData {
    pub stage_id: usize,
    pub spawn_translation: Vec3
}