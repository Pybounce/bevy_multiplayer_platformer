use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{obstacles::InstantKiller, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}};

use super::{tiles::TileBundle, StageObject};

#[derive(Component)]
pub struct Spike;



pub struct SpikeFactory;

impl SpikeFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, rotation: f32) {
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: stage_creator.tile_mesh_handle.clone(),
                transform: Transform::default()
                    .with_translation(Vec3::new(grid_pos.x * TILE_SIZE, grid_pos.y * TILE_SIZE, 0.0))
                    .with_rotation(Quat::from_rotation_z(rotation)),
                material: stage_creator.spike_mat_handle.clone(),
                ..default()
            },
            StageObject { stage_id: stage_creator.stage.id })
        ).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE_HALF * 0.8, TILE_SIZE_HALF * 0.8),
                TransformBundle::from(Transform::from_xyz(0.0, -(TILE_SIZE_HALF * 0.2), 0.0)),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                Spike,
                StageObject { stage_id: stage_creator.stage.id }
            ));
        });

    }
}
