
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{common::{animated_sprite::SpriteAnimator, offset_mover::OffsetMover, rotator::Rotator}, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}}};

use super::{tiles::TileBundle, StageObject};


#[derive(Component)]
pub struct HalfSaw;

pub struct SawFactory;

impl SawFactory {
    pub fn spawn_half(commands: &mut Commands, stage_creator: &StageCreator, saw_asset: &stage_asset::HalfSaw) {
        
        let mut e = commands.spawn((
            MaterialMesh2dBundle {
                mesh: stage_creator.tile_mesh_handle.clone(),
                transform: Transform {
                    translation: (Quat::from_rotation_z(saw_asset.rotation) * Vec3::new(0.0, -TILE_SIZE_HALF, 0.0)) + Vec3::new(saw_asset.grid_pos.x * TILE_SIZE, saw_asset.grid_pos.y * TILE_SIZE, -5.0), 
                    rotation: Quat::from_rotation_z(saw_asset.rotation),
                    ..default()
                },
                material: stage_creator.saw_mat_handle.clone(),
                ..default()
            },
            StageObject { stage_id: stage_creator.stage.id },
            Rotator {
                speed: 10.0
            },
            Collider::ball(TILE_SIZE_HALF * 0.9),
            CollisionGroups::new(Group::GROUP_2, Group::ALL),
            ActiveEvents::COLLISION_EVENTS,
            RigidBody::Fixed,
            InstantKiller,
            HalfSaw,
        ));
        match &saw_asset.movement_path_opt {
            Some(mp) => { e.insert(OffsetMover::new_from_grid(&mp.grid_offsets, mp.speed)); },
            None => (),
        };
        

    }
}