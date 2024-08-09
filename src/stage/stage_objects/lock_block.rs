use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::stage::stage_builder::stage_creator::StageCreator;

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct LockBlock;



pub struct LockBlockFactory;

impl LockBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, rotation: f32) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, rotation, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            LockBlock
        ));
    }
}
