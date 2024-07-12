use bevy::prelude::*;

pub mod tiles;
pub mod goal;

#[derive(Component)]
pub struct StageObject {
    pub stage_id: usize
}