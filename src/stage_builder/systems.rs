use bevy::prelude::*;

use super::{StageBuilderData, StagePiece};



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

pub fn try_build_stage() {

}