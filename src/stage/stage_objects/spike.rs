use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}}};

use super::{tiles::TileBundle, StageObject};

#[derive(Component)]
pub struct Spike;



pub struct SpikeFactory;

impl SpikeFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, rotation: f32) {
        
        commands.spawn(
            TileBundle::new(stage_creator, grid_pos, atlas_rect, rotation, stage_creator.object_tilemap)
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
    pub fn spawn_editor_icon(commands: &mut Commands, spike_asset: &stage_asset::Spike, atlas: &Handle<Image>, atlas_rect: Rect) {
        commands.spawn(
            SpriteBundle {
                transform: Transform {
                    rotation: Quat::from_rotation_z(spike_asset.rotation),
                    translation: Vec3::new(spike_asset.grid_pos.x * TILE_SIZE, spike_asset.grid_pos.y * TILE_SIZE, 0.0),
                    ..default()
                },
                texture: atlas.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    rect: Some(atlas_rect),
                    ..default()
                },
                ..default()
            }
        );
    }
}
