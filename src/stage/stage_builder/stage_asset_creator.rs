

use bevy::{
    asset::ron,
    prelude::*,
};

use crate::stage::stage_builder::stage_asset::{GroundTile, Spike, Stage, Checkpoint};

pub fn save_stage() {
    save_stage_0();
    save_stage_1();
    save_stage_2();
    save_stage_3();
}

pub fn save_stage_0() {

    let grid_width = 17;
    let grid_height = 20;

    let layout = String::from("
    ⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛
    ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟩⬜
    ⬜⬜⬜⬜🟥🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
    ⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
    ⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬜⬜⬜⬛
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥🟥⬜⬜⬜⬜
    ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜
    ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜
    ⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜
    ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬛⬜🟪⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
    ⬛⬛⬛⬛🟥🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛");
    stage_from_grid(layout, grid_width, grid_height, 0);
}

pub fn save_stage_1() {
    
    let grid_width = 40;
    let grid_height = 15;

    let layout = String::from("
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟪⬜⬜⬜⬛
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛
       ⬜⬜⬜⬜🟩⬜⬜⬜⬜⬜⬜⬜🟥🟥🟥🟥⬜⬜⬜⬜⬜⬜⬜🟥🟥🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛
       ⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛
       ⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛
       ⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬛⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥");

    stage_from_grid(layout, grid_width, grid_height, 1);
}

pub fn save_stage_2() {
    
    let grid_width = 53;
    let grid_height = 15;

    let layout = String::from("
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜🟩⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜🟨⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜🟪⬜⬜
       ⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛
       🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥🟥
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       🟥🟥🟥🟥🟥🟥🟥⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥⬛⬛🟥🟥🟥🟥🟥🟥🟥🟥🟥");

    stage_from_grid(layout, grid_width, grid_height, 2);
}

pub fn save_stage_3() {

    let grid_width = 31;
    let grid_height = 19;

    let layout = String::from("
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟩⬜⬛⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬜🟪⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥
       ⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜🟥🟥⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜
       ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜🟥
       ⬜⬜⬜⬜⬜⬜⬛⬛🟥🟥⬜⬜⬛⬛⬛⬛⬜⬜⬜🟥⬛⬛⬜⬜⬜⬜⬜🟥🟥🟥⬛
       🟥🟥⬜⬜🟥🟥⬛⬛⬛⬛🟥⬜⬜⬜⬜⬜⬜⬜🟥⬛⬛⬛⬜⬜⬜🟥🟥⬛⬛⬛⬛
       ⬛⬛🟥🟥⬛⬛⬛⬛⬛⬛⬛🟥🟥🟥🟥🟥🟥⬛⬛⬛⬛⬛🟥🟥🟥⬛⬛⬛⬛⬛⬛");

    stage_from_grid(layout, grid_width, grid_height, 3);
}

fn clean_layout(layout: &String) -> String {
    layout.replace("\n", "").replace(" ", "")
}

fn stage_from_grid(mut layout: String, width: usize, height: usize, id: usize) {
    layout = clean_layout(&layout);
    let mut ground_tiles: Vec<GroundTile> = vec![];
    let mut spikes: Vec<Spike> = vec![];
    let mut checkpoints: Vec<Checkpoint> = vec![];
    let mut goal_grid_pos: Vec2 = Vec2::default();
    let mut spawn_grid_pos: Vec2 = Vec2::default();

    for i in 0..layout.chars().count() {
        let tile = layout.chars().nth(i).unwrap();
        let x = i % width;
        let y = height - 1 - (i / width);

        if tile == '⬜' { continue; }
        if tile == '🟪' { 
            //goal
            goal_grid_pos = Vec2::new(x as f32, y as f32);
        }
        else if tile == '🟥' { 
            //spike
            spikes.push(Spike {grid_pos: Vec2::new(x as f32, y as f32)});
        }
        else if tile == '🟩' {
            //spawn
            spawn_grid_pos = Vec2::new(x as f32, y as f32);
        }
        else if tile == '🟨' {
            //checkpoint
            checkpoints.push(Checkpoint {grid_pos: Vec2::new(x as f32, y as f32)});
        }
        else if tile == '⬛' {
            ground_tiles.push(GroundTile {
                grid_pos: Vec2::new(x as f32, y as f32),
                tilemap_index: 0
            });
        }
        else {
            error!("WHY NO WHYYYY: {}", tile);
        }

    }

    let stage = Stage {
        id: id,
        spawn_grid_pos,
        ground_tiles: ground_tiles,
        spikes: spikes,
        grid_width: width,
        grid_height: height,
        goal_grid_pos,
        checkpoints: checkpoints,
    };

    let mut bytes: Vec<u8> = vec![];
    ron::ser::to_writer(&mut bytes, &stage).unwrap();
    let name = String::from("assets/stage_".to_owned() + &id.to_string() + ".stage");
    let path = std::path::Path::new(&name);     
    let mut file = std::fs::File::create(&path).expect("yeet1");       
 
    use std::io::Write;
    file.write_all(&bytes).expect("yeet2"); 
}



fn get_surrounding_tile_bitmask(layout: &String, index: usize, width: usize, height: usize) -> u8 {
    let i_index = index as isize;
    let i_width = width as isize;

    let is_grounds: Vec<bool> = vec![
    is_layout_index_ground(layout, i_index - i_width),  //up
    is_layout_index_ground(layout, i_index - i_width + 1),  //up right
    is_layout_index_ground(layout, i_index + 1),    //right
    is_layout_index_ground(layout, i_index + i_width + 1),  //down right
    is_layout_index_ground(layout, i_index + i_width),  //down
    is_layout_index_ground(layout, i_index + i_width - 1),  //down left
    is_layout_index_ground(layout, i_index - 1),    //left
    is_layout_index_ground(layout, i_index - i_width - 1)]; //up left

    let mut bitmask: u8 = 0;
    let mut current_bit: u8 = 1;
    for is_ground in is_grounds {
        if is_ground {
            bitmask |= current_bit;
        }
        current_bit <<= 1;
    }
    return bitmask;
}

fn is_layout_index_ground(layout: &String, index: isize) -> bool {
    if index < 0 || index >= layout.chars().count() as isize { 
        return false
    }
    return match layout.chars().nth(index as usize) {
        Some(x) => x == '⬛',
        None => false,
    }

}

fn map_surrounding_ground_bitmask_to_tilemap_index(bitmask: u8) -> usize {
    //todo - tiles with dark dirt on full top layer will never be used (index 9 for example)
    match bitmask {
        0 => 56,
        1 => 22,
        2 => 56,
        3 => 22,
        4 => 54,
        5 => 12,
        6 => 54,
        7 => 18,
        8 => 56,
        9 => 56,

        10 => 56,
        11 => 22,
        12 => 54,
        13 => 12,
        14 => 54,
        15 => 18,
        16 => 57,
        17 => 26,
        18 => 57,
        19 => 26,

        20 => 51,
        21 => 8,
        22 => 51,
        23 => 43,
        24 => 57,
        25 => 26,
        26 => 57,
        27 => 24,
        28 => 49,
        29 => 4,

        30 => 19,
        31 => 15,
        32 => 56,
        33 => 22,
        34 => 56,
        35 => 22,
        36 => 54,
        37 => 12,
        38 => 54,
        39 => 18,

        40 => 56,
        41 => 22,
        42 => 56,
        43 => 56,
        44 => 54,
        45 => 12,
        46 => 54,
        47 => 18,
        48 => 57,
        49 => 26,

        50 => 57,
        51 => 26,
        52 => 51,
        53 => 8,
        54 => 51,
        55 => 43,
        56 => 57,
        57 => 26,
        58 => 57,
        59 => 26,

        60 => 49,
        61 => 4,
        62 => 49,
        63 => 15,
        64 => 53,
        65 => 11,
        66 => 53,
        67 => 11,
        68 => 48,
        69 => 7,

        70 => 48,
        71 => 3,
        72 => 53,
        73 => 0,
        74 => 0,
        75 => 0,
        76 => 0,
        77 => 0,
        78 => 0,
        79 => 0,
        _ => todo!()
    }
}







