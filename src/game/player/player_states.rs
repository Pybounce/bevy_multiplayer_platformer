use bevy::prelude::*;

use crate::{local_player::LocalPlayer, stage::stage_builder::{events::StageBuildCompleteEvent, CurrentStageData}};



#[derive(Component, Default)]
pub enum PlayerState {
    Alive,
    #[default]
    Dead
}

pub fn respawn_dead_players(
    stage_data: Res<CurrentStageData>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &PlayerState, Entity)>
) {
    for (mut t, ps, e) in &mut query {
        match ps {
            PlayerState::Alive => continue,
            PlayerState::Dead => {
                t.translation = stage_data.spawn_translation;
                commands.entity(e).try_insert(PlayerState::Alive);
            },
        }
    }
}

