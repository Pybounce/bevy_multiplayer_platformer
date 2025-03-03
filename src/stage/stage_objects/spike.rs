use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{obstacles::InstantKiller, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}};

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
                Collider::cuboid(TILE_SIZE_HALF * 0.2, TILE_SIZE_HALF * 0.8),
                TransformBundle::from(Transform::from_xyz(-(TILE_SIZE_HALF * 0.5), -(TILE_SIZE_HALF * 0.2), 0.0)),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                Spike,
                StageObject { stage_id: stage_creator.stage.id }
            ));
            parent.spawn((
                Collider::cuboid(TILE_SIZE_HALF * 0.2, TILE_SIZE_HALF * 0.8),
                TransformBundle::from(Transform::from_xyz((TILE_SIZE_HALF * 0.5), -(TILE_SIZE_HALF * 0.2), 0.0)),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                Spike,
                StageObject { stage_id: stage_creator.stage.id }
            ));
        });

    }
    pub fn spawn_editor_icon(commands: &mut Commands, grid_pos: IVec2, rotation: f32, atlas: &Handle<Image>, atlas_rect: Rect) -> Entity {
        commands.spawn(
            SpriteBundle {
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    translation: Vec3::new((grid_pos.x as f32 * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y as f32 * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
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
        ).id()
    }
}
