use bevy::prelude::*;

pub mod tiles;
pub mod goal;
pub mod spike;
pub mod half_saw;
pub mod spring;
pub mod lock_block;
pub mod key;
pub mod interval_block;

#[derive(Component)]
pub struct StageObject {
    pub stage_id: usize
}