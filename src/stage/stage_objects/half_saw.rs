
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{common::animated_sprite::AnimatedSprite, obstacles::InstantKiller, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE_HALF}};

use super::{spike::SpikeFactory, tiles::{PhysicalTileBundle, TileBundle}, StageObject};


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
        
        let mut bundle = HalfSawBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_2, Group::ALL)),
            half_saw_marker: HalfSaw,
            instant_killer: InstantKiller,
            animated_sprite : AnimatedSprite::new(50, atlas_rects)
        };

        bundle.physical_tile_bundle.collider = Collider::cuboid(0.45, 0.225);

        return bundle;

    }
}

pub struct SawFactory;

impl SawFactory {
    pub fn spawn_half(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rects: Vec<Rect>, rotation: f32) {
        
        commands.spawn((
            TileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap),
            AnimatedSprite::new(50, atlas_rects)
        )).with_children(|parent| {
            parent.spawn((
                Collider::ball(TILE_SIZE_HALF * 0.9),
                TransformBundle::from(Transform::from_xyz(0.0, -TILE_SIZE_HALF, 0.0)),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                HalfSaw,
                StageObject { stage_id: stage_creator.stage.id }
            ));
        });

    }
}