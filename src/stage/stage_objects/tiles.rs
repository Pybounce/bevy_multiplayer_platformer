
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{stage::{stage_builder::stage_creator::StageCreator, stage_objects::StageObject}, stage_1::Ground};


const TILE_SIZE: f32 = 32.0;

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite_bundle: SpriteBundle,
    stage_marker: StageObject
}

#[derive(Bundle)]
pub struct PhysicalTileBundle {
    tile_bundle: TileBundle,
    rigidbody: RigidBody,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    gravity_scale: GravityScale,
    active_events: ActiveEvents,
}

#[derive(Bundle)]
pub struct GroundTileBundle {
    physical_tile_bundle: PhysicalTileBundle,
    ground_marker: Ground
}


impl TileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {

        TileBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
                    translation: Vec3::new(grid_pos.x * TILE_SIZE, grid_pos.y * TILE_SIZE, 0.0),
                    ..default()
                },
                texture: stage_creator.colour_palettes.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    rect: Some(atlas_rect),
                    ..default()
                },
                ..default()
            },
            stage_marker: StageObject { stage_id: stage_creator.stage.id },
        }
        
    }
}

impl PhysicalTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        PhysicalTileBundle {
            tile_bundle: TileBundle::new(stage_creator, grid_pos, atlas_rect),
            rigidbody: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
            restitution: Restitution::coefficient(0.0),
            friction: Friction::coefficient(0.0),
            gravity_scale: GravityScale(0.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl GroundTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        GroundTileBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect),
            ground_marker: Ground,
        }
    }
}