use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::{obstacles::InstantKiller, stage::stage_builder::stage_creator::StageCreator};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct Spike;

#[derive(Bundle)]
pub struct SpikeBundle {
    pub physical_tile_bundle: PhysicalTileBundle,
    pub spike_marker: Spike,
    pub instant_killer: InstantKiller
}

impl SpikeBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        
        SpikeBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_2, Group::ALL)),
            spike_marker: Spike,
            instant_killer: InstantKiller,
        }

    }
}