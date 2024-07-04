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

//TODO: Make this more resiliant
// Right now this is run directly after the stage building
// And it assumes that the stage was built
// If something went wrong, like the stage file didn't exist...
// The StageData would be null and crash the program
// Consider raising a player reset event or just adding an Option<StageData>
pub fn reset_players(
    stage_data: Res<StageData>,
    mut query: Query<&mut Transform, With<LocalPlayer>>
) {
    for mut t in &mut query {
        t.translation = stage_data.respawn_translation;
    }
}