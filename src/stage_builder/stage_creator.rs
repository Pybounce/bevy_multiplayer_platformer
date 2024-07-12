use crate::stage_1::Ground;

use super::{stage_asset::Stage, tiles::GroundTileBundle, StageMarker};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const TILE_SIZE: f32 = 32.0;

pub struct StageCreator<'a> {
    pub stage: &'a Stage, 
    pub colour_palettes: &'a Handle<Image>
}

pub enum ColourPaletteAtlasIndex {
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
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, -1.0);
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, stage_creator.stage.grid_height as f32);

    }
    for y in 0..stage_creator.stage.grid_height + 2{
        build_ground_tile(commands, stage_creator, -1.0, y as f32);
        build_ground_tile(commands, stage_creator, stage_creator.stage.grid_width as f32, y as f32);

    }
    return true;
}

fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        let mut pos: Vec2 = tile.grid_pos;
        pos.y = (stage_creator.stage.grid_height as f32) - 1.0 - pos.y;

        //build_tile(pos.x, pos.y, PaletteColours::Ground as usize, commands, stage_creator.colour_palettes, 1);
        build_ground_tile(commands, stage_creator, pos.x, pos.y);
    }
    return true;
}

fn build_background(stage_creator: &StageCreator) -> bool {
    todo!();
}

fn set_camera_background(texture_handle: &Handle<Image>) {
    todo!();
}

fn build_ground_tile(commands: &mut Commands, stage_creator: &StageCreator, grid_x: f32, grid_y: f32) {

    let sprite_rect_x = (ColourPaletteAtlasIndex::Ground as usize % 5) as f32;
    let sprite_rect_y = (ColourPaletteAtlasIndex::Ground as usize / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);


    commands.spawn(GroundTileBundle::new(
        stage_creator, 
        Vec2::new(grid_x, grid_y), 
        sprite_rect));

}


