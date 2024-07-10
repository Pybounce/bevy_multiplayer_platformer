use crate::stage_1::Ground;

use super::{stage_asset::Stage, StagePiece};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub struct StageCreator<'a> {
    stage: &'a Stage, 
    colour_palettes: &'a Handle<Image>
}


impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_perimeter(self, commands)// && build_background(self)
    }


}

fn build_perimeter(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for x in 0..stage_creator.stage.tiles_width {
        build_tile(x as f32, -1.0, 0, commands, stage_creator.colour_palettes, 1);
        build_tile(x as f32, stage_creator.stage.tiles_height as f32, 0, commands, stage_creator.colour_palettes, 1);
    }

    return true;
}

fn build_background(stage_creator: &StageCreator) -> bool {
    todo!();
}

fn set_camera_background(texture_handle: &Handle<Image>) {
    todo!();
}


fn build_tile(x: f32, y: f32, atlas_index: usize, commands: &mut Commands, tex_handle: &Handle<Image>, stage_id: usize) {
    let TILE_SIZE = 32.0;

    let sprite_rect_x = (atlas_index % 5) as f32;
    let sprite_rect_y = (atlas_index / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);
    
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
            translation: Vec3::new(x * TILE_SIZE, y * TILE_SIZE, 0.0),
            ..default()
        },
        texture: tex_handle.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            rect: Some(sprite_rect),
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(0.0))
    .insert(Friction::coefficient(0.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(StagePiece {stage_id});
}