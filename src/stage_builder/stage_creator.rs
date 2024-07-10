use crate::stage_1::Ground;

use super::{stage_asset::Stage, StagePiece};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const TILE_SIZE: f32 = 32.0;

pub struct StageCreator<'a> {
    stage: &'a Stage, 
    colour_palettes: &'a Handle<Image>
}

pub enum PaletteColours {
    Background,
    Ground,
    Obstacle,
    Reward,
    Misc
}

impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_perimeter(self, commands) && build_ground(self, commands)
    }


}

fn build_perimeter(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for x in 0..stage_creator.stage.grid_width + 2{
        build_tile(x as f32 - 1.0, -1.0, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
        build_tile(x as f32 - 1.0, stage_creator.stage.grid_height as f32, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
    }
    for y in 0..stage_creator.stage.grid_height + 2{
        build_tile(-1.0, y as f32 - 1.0, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
        build_tile(stage_creator.stage.grid_width as f32, y as f32 - 1.0, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
    }
    return true;
}

fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        let mut pos: Vec2 = tile.grid_pos;
        pos.y = (stage_creator.stage.grid_height as f32) - 1.0 - pos.y;

        build_tile(pos.x, pos.y, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
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