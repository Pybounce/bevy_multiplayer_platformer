use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::stage::stage_builder::stage_creator::StageCreator;

use super::tiles::PhysicalTileBundle;


#[derive(Component)]
pub struct StageGoal;

#[derive(Bundle)]
pub struct GoalBundle {
    physical_tile_bundle: PhysicalTileBundle,
    goal_marker: StageGoal
}

impl GoalBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        GoalBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.tilemap, CollisionGroups::new(Group::GROUP_3, Group::ALL)),
            goal_marker: StageGoal,
        }
    }
}