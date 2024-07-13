use crate::stage::{self, stage_objects::{goal::GoalBundle, spike::SpikeBundle, tiles::{GroundTileBundle, TileBundle}}};

use super::{stage_asset::Stage};
use bevy::prelude::*;

const TILE_SIZE: f32 = 32.0;

pub struct StageCreator<'a> {
    pub stage: &'a Stage, 
    pub colour_palettes: &'a Handle<Image>
}

pub enum ColourPaletteAtlasIndex {
    Background,
    Ground,
    Obstacle,
    Goal,
    _Misc
}

impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_perimeter(self, commands) 
        && build_ground(self, commands)
        && build_goal(self, commands)
        && build_background(self, commands)
        && build_spikes(self, commands)
        && build_far_background(self, commands)

    }


}

fn build_perimeter(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for x in 0..stage_creator.stage.grid_width + 2{
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, -1.0);
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, stage_creator.stage.grid_height as f32);

    }
    for y in 0..stage_creator.stage.grid_height + 2{
        build_ground_tile(commands, stage_creator, -1.0, y as f32 - 1.0);
        build_ground_tile(commands, stage_creator, stage_creator.stage.grid_width as f32, y as f32 - 1.0);

    }
    return true;
}

fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        build_ground_tile(commands, stage_creator, tile.grid_pos.x, tile.grid_pos.y);
    }
    return true;
}

fn build_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let sprite_rect_x = (ColourPaletteAtlasIndex::Background as usize % 5) as f32;
    let sprite_rect_y = (ColourPaletteAtlasIndex::Background as usize / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);

    let mut background = TileBundle::new(
        stage_creator, 
        Vec2::new((stage_creator.stage.grid_width as f32 - 1.0) / 2.0, 
        (stage_creator.stage.grid_height as f32 - 1.0) / 2.0), 
        sprite_rect);
    background.sprite_bundle.transform.translation.z = -10.0;
    background.sprite_bundle.transform.scale = Vec3::new(
        stage_creator.stage.grid_width as f32 * TILE_SIZE,
        stage_creator.stage.grid_height as f32 * TILE_SIZE,
        1.0);
    commands.spawn(background);

    return true;
}

fn build_far_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let sprite_rect_x = (ColourPaletteAtlasIndex::Ground as usize % 5) as f32;
    let sprite_rect_y = (ColourPaletteAtlasIndex::Ground as usize / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);

    let mut background = TileBundle::new(
        stage_creator, 
        Vec2::new((stage_creator.stage.grid_width as f32 - 1.0) / 2.0, 
        (stage_creator.stage.grid_height as f32 - 1.0) / 2.0), 
        sprite_rect);
    background.sprite_bundle.transform.translation.z = -20.0;
    background.sprite_bundle.transform.scale = Vec3::new(
        stage_creator.stage.grid_width as f32 * TILE_SIZE * 10.0,
        stage_creator.stage.grid_height as f32 * TILE_SIZE * 10.0,
        1.0);
    commands.spawn(background);

    return true;
}

fn build_goal(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let sprite_rect_x = (ColourPaletteAtlasIndex::Goal as usize % 5) as f32;
    let sprite_rect_y = (ColourPaletteAtlasIndex::Goal as usize / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);
    
    commands.spawn(GoalBundle::new(
        &stage_creator, 
        stage_creator.stage.goal_grid_pos, 
        sprite_rect));
    return true;
}

fn build_spikes(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    let sprite_rect_x = (ColourPaletteAtlasIndex::Obstacle as usize % 5) as f32;
    let sprite_rect_y = (ColourPaletteAtlasIndex::Obstacle as usize / 5) as f32;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 1.0, sprite_rect_y + 1.0);
    

    for spike in &stage_creator.stage.spikes {
        commands.spawn(SpikeBundle::new(
            stage_creator, 
            spike.grid_pos, 
            sprite_rect));
    }

    return true;
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


