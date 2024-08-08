
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{common::animated_sprite::{Animating, SpriteAnimator}, obstacles::InstantKiller, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE_HALF}};

use super::{tiles::TileBundle, StageObject};


#[derive(Component)]
pub struct HalfSaw;

pub struct SawFactory;

impl SawFactory {
    pub fn spawn_half(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rects: Vec<Rect>, rotation: f32) {
        
        commands.spawn((
            TileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap),
            SpriteAnimator::new(50, atlas_rects),
            Animating
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