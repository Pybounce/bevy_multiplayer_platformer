use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::stage::stage_builder::stage_creator::StageCreator;

use super::tiles::PhysicalTileBundle;


#[derive(Component)]
pub struct StageGoal;


pub struct GoalFactory;

impl GoalFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) {
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_3, Group::ALL)),
            StageGoal
        ));
    }
}