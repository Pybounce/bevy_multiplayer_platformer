use bevy::prelude::*;

use crate::{local_player::LocalPlayer, stage::stage_builder::CurrentStageData};





pub fn check_player_out_of_bounds(
    mut query: Query<&mut Transform, With<LocalPlayer>>,
    stage_data: Res<CurrentStageData>
) {
    for mut t in &mut query {
        if !stage_data.bounds.contains(t.translation.truncate()) {
            t.translation = stage_data.spawn_translation;
        }
    }
}


