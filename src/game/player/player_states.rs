use bevy::prelude::*;

use crate::{game::stage_manager::StageData, local_player::LocalPlayer};



#[derive(Component, Default)]
pub enum PlayerState {
    Alive,
    #[default]
    Dead
}

pub fn respawn_dead_players(
    stage_data: Res<StageData>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &PlayerState, Entity)>
) {
    for (mut t, ps, e) in &mut query {
        match ps {
            PlayerState::Alive => continue,
            PlayerState::Dead => {
                t.translation = stage_data.respawn_translation;
                commands.entity(e).try_insert(PlayerState::Alive);
            },
        }
    }
}

pub fn reset_players(
    stage_data: Res<StageData>,
    mut query: Query<&mut Transform, With<LocalPlayer>>
) {
    for mut t in &mut query {
        t.translation = stage_data.respawn_translation;
    }
}