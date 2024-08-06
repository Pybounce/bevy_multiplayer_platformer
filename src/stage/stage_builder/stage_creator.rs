use crate::{common::checkpoint::CheckpointBundle, player::spawner::LocalPlayerSpawner, stage::stage_objects::{goal::GoalFactory, half_saw::SawFactory, spike::SpikeFactory, tiles::{GroundTileBundle, TileBundle}, StageObject}};

use super::stage_asset::Stage;
use bevy::prelude::*;

pub const TILE_SIZE: f32 = 16.0;
pub const TILE_SIZE_HALF: f32 = TILE_SIZE / 2.0;
const TILEMAP_SIZE: usize = 7;
const TILEMAP_TILE_SIZE: f32 = 16.0;
const OBJECT_TILEMAP_SIZE: usize = 16;
const OBJECT_TILE_TILEMAP_SIZE: f32 = 16.0;

pub struct StageCreator<'a> {
    pub stage: &'a Stage, 
    pub colour_palettes: &'a Handle<Image>,
    pub tilemap: &'a Handle<Image>,
    pub object_tilemap: &'a Handle<Image>
}

pub enum ColourPaletteAtlasIndex {
    Background,
    Ground,
    Obstacle,
    Goal,
    _Misc
}

impl<'a> StageCreator<'a> {

    pub fn new(stage: &'a Stage, colour_palettes: &'a Handle<Image>, tilemap: &'a Handle<Image>, object_tilemap: &'a Handle<Image>) -> Self {
        StageCreator {
            stage,
            colour_palettes,
            tilemap,
            object_tilemap
        }
    }

    pub fn build(&self, commands: &mut Commands) -> bool {
        build_ground(self, commands)
        && build_goal(self, commands)
        && build_background(self, commands)
        && build_spikes(self, commands)
        && build_far_background(self, commands)
        && build_player_spawner(self, commands)
        && build_checkpoints(self, commands)
        && build_half_saws(self, commands)

    }


}

fn build_player_spawner(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    commands.spawn(LocalPlayerSpawner {
        spawn_time: 0.0,
        translation: (stage_creator.stage.spawn_grid_pos * TILE_SIZE).extend(0.0),
    });
    return true;
}


fn build_ground(stage_creator: &StageCreator, commands: &mut Commands) -> bool {
    for tile in &stage_creator.stage.ground_tiles {
        build_ground_tile(commands, stage_creator, tile.grid_pos.x, tile.grid_pos.y, tile.tilemap_index);
    }
    return true;
}

fn build_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let grid_pos = Vec2::new((stage_creator.stage.grid_width as f32 - 1.0) / 2.0, 
    (stage_creator.stage.grid_height as f32 - 1.0) / 2.0);
    
    commands.spawn(
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(TILE_SIZE * stage_creator.stage.grid_width as f32, TILE_SIZE * stage_creator.stage.grid_height as f32, 1.0),
                translation: Vec3::new(grid_pos.x * TILE_SIZE, grid_pos.y * TILE_SIZE, -10.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                color: Color::linear_rgb(0.76, 0.95, 1.0),
                ..default()
            },
            ..default()
        }
    )
    .insert(StageObject { stage_id: stage_creator.stage.id });

    return true;
}

fn build_far_background(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let upper_left = Vec2::new((52 as f32 % TILEMAP_SIZE as f32) as f32 * TILEMAP_TILE_SIZE, (52 / TILEMAP_SIZE) as f32 * TILEMAP_TILE_SIZE);
    let lower_right = Vec2::new(upper_left.x + TILEMAP_TILE_SIZE , upper_left.y + TILEMAP_TILE_SIZE);
    let sprite_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);

    let mut background = TileBundle::new(
        stage_creator, 
        Vec2::new((stage_creator.stage.grid_width as f32 - 1.0) / 2.0, 
        (stage_creator.stage.grid_height as f32 - 1.0) / 2.0), 
        sprite_rect, 0.0, stage_creator.tilemap);
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
    
    GoalFactory::spawn(
        commands,
        &stage_creator, 
        stage_creator.stage.goal_grid_pos, 
        sprite_rect);
        
    return true;
}

fn build_spikes(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let sprite_rect = get_object_tilemap_rect_from_index(4);

    for spike in &stage_creator.stage.spikes {

        SpikeFactory::spawn(commands, stage_creator, spike.grid_pos, sprite_rect);
    }

    return true;
}

fn build_half_saws(stage_creator: &StageCreator, commands: &mut Commands) -> bool {

    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(0),
        get_object_tilemap_rect_from_index(1),
        get_object_tilemap_rect_from_index(2),
        get_object_tilemap_rect_from_index(3),
    ];

    for half_saw in &stage_creator.stage.half_saws {
        SawFactory::spawn_half(commands, stage_creator, half_saw.grid_pos, atlas_rects.clone(), half_saw.rotation);
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

    let upper_left = Vec2::new((atlas_index as f32 % TILEMAP_SIZE as f32) as f32 * TILEMAP_TILE_SIZE, (atlas_index / TILEMAP_SIZE) as f32 * TILEMAP_TILE_SIZE);
    let lower_right = Vec2::new(upper_left.x + TILEMAP_TILE_SIZE , upper_left.y + TILEMAP_TILE_SIZE);
    let sprite_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);

    commands.spawn(GroundTileBundle::new(
        stage_creator, 
        Vec2::new(grid_x, grid_y), 
        sprite_rect));


}

fn get_object_tilemap_rect_from_index(index: usize) -> Rect {

    let upper_left = Vec2::new((index as f32 % OBJECT_TILEMAP_SIZE as f32) as f32 * OBJECT_TILE_TILEMAP_SIZE, (index / OBJECT_TILEMAP_SIZE) as f32 * OBJECT_TILE_TILEMAP_SIZE);
    let lower_right = Vec2::new(upper_left.x + OBJECT_TILE_TILEMAP_SIZE, upper_left.y + OBJECT_TILE_TILEMAP_SIZE);
    Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y)
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




