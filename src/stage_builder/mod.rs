use bevy::prelude::*;
use events::{read_stage_build_events, read_stage_load_events, BuildStageEvent, LoadStageEvent};
use stage_asset::{Stage, StageLoader};
use systems::unload_old_stage;

mod events;
mod stage_asset;
mod systems;

pub struct StageBuilderPlugin;

impl Plugin for StageBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<LoadStageEvent>()
        .add_event::<BuildStageEvent>()
        .init_state::<StageBuilderState>()
        .init_asset::<Stage>()
        .init_asset_loader::<StageLoader>()
        .init_resource::<StageBuilderData>()
        .add_systems(Update, (read_stage_load_events, read_stage_build_events).chain())
        .add_systems(OnEnter(StageBuilderState::Building), unload_old_stage);
    }
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
enum StageBuilderState {
    #[default]
    NotBuilding,
    Building,
}



#[derive(Resource, Default)]
struct StageBuilderData {
    stage_id: usize,
    stage_handle: Handle<Stage>
}

#[derive(Component)]
struct StagePiece {
    pub stage_id: usize
}