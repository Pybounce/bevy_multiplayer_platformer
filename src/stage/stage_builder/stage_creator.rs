use crate::{common::checkpoint::CheckpointBundle, player::spawner::LocalPlayerSpawner, stage::stage_objects::{goal::GoalBundle, spike::SpikeBundle, tiles::{GroundTileBundle, TileBundle}}};

use super::stage_asset::Stage;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

pub const TILE_SIZE: f32 = 32.0;

pub struct StageCreator<'a> {
    pub stage: &'a Stage, 
    pub colour_palettes: &'a Handle<Image>,
    pub tilemap: &'a Handle<Image>
}

pub enum ColourPaletteAtlasIndex {
    Background,
    Ground,
    Obstacle,
    Goal,
    _Misc
}

impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>, tilemap: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes,
            tilemap
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_ground(self, commands)
        //build_perimeter(self, commands) 
        && build_goal(self, commands)
        //&& build_background(self, commands)
        && build_spikes(self, commands)
        //&& build_far_background(self, commands)
        && build_player_spawner(self, commands)
        && build_checkpoints(self, commands)

    }


}

fn build_player_spawner(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    commands.spawn(LocalPlayerSpawner {
        spawn_time: 0.0,
        translation: (stage_creator.stage.spawn_grid_pos * TILE_SIZE).extend(0.0),
    });
    return true;
}

fn build_perimeter(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for x in 0..stage_creator.stage.grid_width + 2{
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, -1.0, 0);
        build_ground_tile(commands, stage_creator, x as f32 - 1.0, stage_creator.stage.grid_height as f32, 0);

    }
    for y in 0..stage_creator.stage.grid_height + 2{
        build_ground_tile(commands, stage_creator, -1.0, y as f32 - 1.0, 0);
        build_ground_tile(commands, stage_creator, stage_creator.stage.grid_width as f32, y as f32 - 1.0, 0);

    }
    return true;
}

fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        build_ground_tile(commands, stage_creator, tile.grid_pos.x, tile.grid_pos.y, tile.tilemap_index);
    }
    return true;
}

fn build_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = colour_palette_rect_from_index(ColourPaletteAtlasIndex::Background);

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

    let sprite_rect = colour_palette_rect_from_index(ColourPaletteAtlasIndex::Ground);

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

    let sprite_rect = colour_palette_rect_from_index(ColourPaletteAtlasIndex::Goal);
    
    commands.spawn(GoalBundle::new(
        &stage_creator, 
        stage_creator.stage.goal_grid_pos, 
        sprite_rect)).try_insert(CollisionGroups::new(Group::GROUP_3, Group::ALL));
    return true;
}

fn build_spikes(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = colour_palette_rect_from_index(ColourPaletteAtlasIndex::Obstacle);

    for spike in &stage_creator.stage.spikes {
        commands.spawn(SpikeBundle::new(
            stage_creator, 
            spike.grid_pos, 
            sprite_rect)).try_insert(CollisionGroups::new(Group::GROUP_2, Group::ALL));
    }

    return true;
}

fn build_checkpoints(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = colour_palette_rect_from_index(ColourPaletteAtlasIndex::_Misc);

    for checkpoint in &stage_creator.stage.checkpoints {
        commands.spawn(CheckpointBundle::new(
            stage_creator, 
            checkpoint.grid_pos, 
            sprite_rect));
    }

    return true;
}

fn build_ground_tile(commands: &mut Commands, stage_creator: &StageCreator, grid_x: f32, grid_y: f32, atlas_index: usize) {
    let tilemap_size = 8;
    let tilemap_tile_size = 16.0;
    let upper_left = Vec2::new((atlas_index as f32 % tilemap_size as f32) as f32 * tilemap_tile_size, (atlas_index / tilemap_size) as f32 * tilemap_tile_size);
    let lower_right = Vec2::new(upper_left.x + tilemap_tile_size , upper_left.y + tilemap_tile_size);
    let sprite_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);
    error!("rect upper {}", upper_left);
    error!("rect lower {}", lower_right);
    commands.spawn(GroundTileBundle::new(
        stage_creator, 
        Vec2::new(grid_x, grid_y), 
        sprite_rect));

}


fn colour_palette_rect_from_index(index: ColourPaletteAtlasIndex) -> Rect {

    let i_index = index as i32;
    let padding = 1.0;
    let upper_left = IVec2::new(i_index % 5, i_index / 5).as_vec2();
    let lower_right = Vec2::new(upper_left.x, upper_left.y);

    return Rect::new(
        index_to_padded_index(upper_left.x, padding), 
        index_to_padded_index(upper_left.y, padding), 
        index_to_padded_index(lower_right.x, padding) + 0.0, 
        index_to_padded_index(lower_right.y, padding) + 0.0, 
    );
}

fn index_to_padded_index(index: f32, padding: f32) -> f32 {
    padding + (index * ((2.0 * padding) + 1.0))
}