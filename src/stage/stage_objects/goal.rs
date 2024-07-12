use bevy::prelude::*;

use crate::{game::stage_goal::StageGoal, stage::stage_builder::stage_creator::StageCreator};

use super::tiles::PhysicalTileBundle;


#[derive(Bundle)]
pub struct GoalBundle {
    physical_tile_bundle: PhysicalTileBundle,
    goal_marker: StageGoal
}

impl GoalBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        GoalBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect),
            goal_marker: StageGoal,
        }
    }
}