

use bevy::{
    asset::ron,
    prelude::*,
};

use crate::stage::stage_builder::stage_asset::{GroundTile, Spike, Stage, Checkpoint};

pub fn save_stage() {
    //save_stage_0();
    //save_stage_1();
    //save_stage_2();
    //save_stage_3();
    //save_stage_4();
}

pub fn save_stage_0() {

    let grid_width = 17;
    let grid_height = 20;

    let layout = String::from("
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟩⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜
    ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬛⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜🟪⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
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

pub fn save_stage_4() {

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
    stage_from_grid(layout, grid_width, grid_height, 4);
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
                tilemap_index: get_ground_atlas_index(&layout, i, width)
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


fn get_ground_atlas_index(layout: &String, index: usize, width: usize) -> usize {
    let bitmask = get_surrounding_tile_bitmask(layout, index, width);
    let atlas_index = map_surrounding_ground_bitmask_to_tilemap_index(bitmask);

    return atlas_index;
}

fn get_surrounding_tile_bitmask(layout: &String, index: usize, width: usize) -> u8 {
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
        73 => 11,
        74 => 53,
        75 => 11,
        76 => 48,
        77 => 7,
        78 => 48,
        79 => 3,

        80 => 47,
        81 => 6,
        82 => 47,
        83 => 6,
        84 => 52,
        85 => 36,
        86 => 52,
        87 => 32,
        88 => 47,
        89 => 6,

        90 => 47,
        91 => 6,
        92 => 58,
        93 => 33,
        94 => 58,
        95 => 29,
        96 => 53,
        97 => 11,
        98 => 53,
        99 => 11,

        100 => 48,
        101 => 7,
        102 => 48,
        103 => 3,
        104 => 53,
        105 => 22,
        106 => 53,
        107 => 11,
        108 => 48,
        109 => 7,

        110 => 52,
        111 => 3,
        112 => 55,
        113 => 45,
        114 => 55,
        115 => 45,
        116 => 50,
        117 => 34,
        118 => 50,
        119 => 42,

        120 => 55,
        121 => 45,
        122 => 55,
        123 => 45,
        124 => 1,
        125 => 30,
        126 => 1,
        127 => 38,
        128 => 56,
        129 => 22,

        130 => 56,
        131 => 22,
        132 => 54,
        133 => 12,
        134 => 54,
        135 => 18,
        136 => 56,
        137 => 22,
        138 => 56,
        139 => 22,

        140 => 54,
        141 => 12,
        142 => 54,
        143 => 18,
        144 => 57,
        145 => 26,
        146 => 57,
        147 => 26,
        148 => 51,
        149 => 8,

        150 => 51,
        151 => 43,
        152 => 57,
        153 => 26,
        154 => 57,
        155 => 26,
        156 => 49,
        157 => 4,
        158 => 49,
        159 => 15,

        160 => 56,
        161 => 22,
        162 => 56,
        163 => 22,
        164 => 54,
        165 => 12,
        166 => 54,
        167 => 18,
        168 => 56,
        169 => 22,

        170 => 56,
        171 => 22,
        172 => 54,
        173 => 12,
        174 => 54,
        175 => 18,
        176 => 57,
        177 => 26,
        178 => 57,
        179 => 26,

        180 => 51,
        181 => 8,
        182 => 51,
        183 => 26,
        184 => 57,
        185 => 26,
        186 => 57,
        187 => 26,
        188 => 49,
        189 => 4,

        190 => 49,
        191 => 15,
        192 => 53,
        193 => 21,
        194 => 53,
        195 => 21,
        196 => 48,
        197 => 46,
        198 => 48,
        199 => 14,

        200 => 53,
        201 => 21,
        202 => 53,
        203 => 21,
        204 => 48,
        205 => 46,
        206 => 48,
        207 => 14,
        208 => 47,
        209 => 2,

        210 => 47,
        211 => 2,
        212 => 52,
        213 => 35,
        214 => 52,
        215 => 32,
        216 => 47,
        217 => 2,
        218 => 47,
        219 => 2,

        220 => 58,
        221 => 41,
        222 => 58,
        223 => 37,
        224 => 53,
        225 => 21,
        226 => 53,
        227 => 21,
        228 => 48,
        229 => 46,

        230 => 48,
        231 => 14,
        232 => 53,
        233 => 21,
        234 => 53,
        235 => 21,
        236 => 48,
        237 => 46,
        238 => 48,
        239 => 14,

        240 => 55,
        241 => 17,
        242 => 55,
        243 => 17,
        244 => 50,
        245 => 55,
        246 => 50,
        247 => 40,
        248 => 55,
        249 => 17,

        250 => 55,
        251 => 17,
        252 => 1,
        253 => 39,
        254 => 1,
        255 => 0,
        _ => 0
    }
}







