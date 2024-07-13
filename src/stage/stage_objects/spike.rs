use bevy::prelude::*;

use crate::stage::stage_builder::stage_creator::StageCreator;

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct Spike;

#[derive(Bundle)]
pub struct SpikeBundle {
    physical_tile_bundle: PhysicalTileBundle,
    spike_marker: Spike
}

impl SpikeBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        SpikeBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect),
            spike_marker: Spike,
        }
    }
}