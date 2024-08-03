
use bevy::prelude::*;

use crate::{common::animated_sprite::AnimatedSprite, obstacles::InstantKiller, stage::stage_builder::stage_creator::StageCreator};

use super::tiles::PhysicalTileBundle;


#[derive(Component)]
pub struct HalfSaw;

#[derive(Bundle)]
pub struct HalfSawBundle {
    physical_tile_bundle: PhysicalTileBundle,
    half_saw_marker: HalfSaw,
    instant_killer: InstantKiller,
    animated_sprite: AnimatedSprite
}

impl HalfSawBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rects: Vec<Rect>, rotation: f32) -> Self {
        HalfSawBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap),
            half_saw_marker: HalfSaw,
            instant_killer: InstantKiller,
            animated_sprite : AnimatedSprite::new(500, atlas_rects)
        }
    }
}