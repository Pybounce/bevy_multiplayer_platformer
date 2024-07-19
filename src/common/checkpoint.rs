use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{local_player::LocalPlayer, player::death::Respawnable, stage::stage_builder::{stage_creator::{StageCreator, TILE_SIZE}, CurrentStageData}};

#[derive(Component)]
pub struct Checkpoint;

#[derive(Bundle)]
pub struct CheckpointBundle {
    pub checkpoint_marker: Checkpoint,
    pub sprite_bundle: SpriteBundle,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub sensor_marker: Sensor,
    pub active_events: ActiveEvents
}

impl CheckpointBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> CheckpointBundle {
        CheckpointBundle {
            checkpoint_marker: Checkpoint,
            rigidbody: RigidBody::Fixed,
            collider: Collider::ball(0.5),
            sensor_marker: Sensor,
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
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub fn check_checkpoint_reached(
    checkpoint_query: Query<(Entity, &Transform), With<Checkpoint>>,
    mut player_query: Query<(&mut Respawnable, &CollidingEntities), With<LocalPlayer>>,
    mut stage_data: Option<ResMut<CurrentStageData>>,
    mut commands: Commands
) {
    if let Some(stage_data) = &mut stage_data {
        for (mut r, ce) in &mut player_query {
            for colliding_entity in ce.iter() {
                if let Ok((e, t)) = checkpoint_query.get(colliding_entity) {
                    r.translation = t.translation;
                    stage_data.spawn_translation = t.translation;
                    commands.entity(e).despawn();
                }
            }
        }
    }

}