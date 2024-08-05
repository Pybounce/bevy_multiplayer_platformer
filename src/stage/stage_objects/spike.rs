use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, CollisionGroups, Group, RigidBody};

use crate::{obstacles::InstantKiller, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE}};

use super::tiles::{PhysicalTileBundle, TileBundle};

#[derive(Component)]
pub struct Spike;



pub struct SpikeFactory;

impl SpikeFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) {
        
        commands.spawn(
            TileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap)
        ).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(0.4, 0.4),
                TransformBundle::from(Transform::from_xyz(0.0, -(0.1), 0.0)),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                Spike
            ));
        });

    }
}
