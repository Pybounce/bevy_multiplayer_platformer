

use bevy::{
    asset::ron,
    prelude::*,
};

use crate::stage::stage_builder::stage_asset::{Checkpoint, GroundTile, HalfSaw, Spike, Stage};

pub fn save_stage() {
    save_stage_0();
    save_stage_1();
    save_stage_2();
    save_stage_3();
    save_stage_4();
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
    ⬜⬜⬜⬜⬜⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜🔴⬜
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
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜🟪⬜⬜🔴⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛
       ⬛⬜⬜⬜🟩⬜⬜⬜⬜⬜⬜⬜🔴🔴🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛
       ⬛⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛
       ⬛⬜🔴⬛⬛⬛🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛
       ⬛⬜🔴⬛⬛⬛🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛");

    stage_from_grid(layout, grid_width, grid_height, 1);
}

pub fn save_stage_2() {
    
    let grid_width = 53;
    let grid_height = 15;

    let layout = String::from("
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛🟩⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜🟨⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜🟪⬜⬛
       ⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛
       ⬛🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥🟥⬜⬛⬛⬜🟥🟥🟥🟥🟥🟥🟥⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛");

    stage_from_grid(layout, grid_width, grid_height, 2);
}

pub fn save_stage_3() {

    let grid_width = 31;
    let grid_height = 19;

    let layout = String::from("
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟩⬜⬛⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬜🟪⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜🔴🔴⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬛⬛⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬛⬛⬛⬛⬜⬜⬜⬜⬜⬜⬛
       ⬛⬜⬜⬜⬜⬜⬛⬛🔴🔴⬜⬜⬛⬛⬛⬛⬜⬜⬜🔴⬛⬛⬜⬜⬜⬜⬜🔴🔴🔴⬛
       ⬛🔴🔴🔴🔴🔴⬛⬛⬛⬛🔴🔴🔴🔴🔴🔴🔴🔴🔴⬛⬛⬛🔴🔴🔴🔴🔴⬛⬛⬛⬛
       ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛");

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

fn stage_from_grid(mut layout: String, mut width: usize, mut height: usize, id: usize) {
    layout = clean_layout(&layout);
    let mut ground_tiles: Vec<GroundTile> = vec![];
    let mut spikes: Vec<Spike> = vec![];
    let mut half_saws: Vec<HalfSaw> = vec![];
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
        else if tile == '🔴' { 
            //spike
            half_saws.push(HalfSaw {grid_pos: Vec2::new(x as f32, y as f32), rotation: get_rotation(&layout, i, width) });
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
        half_saws: half_saws
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

fn get_rotation(layout: &String, index: usize, width: usize) -> f32 {
    let i_index = index as isize;
    let i_width = width as isize;
    if is_layout_index_ground(layout, i_index + i_width) {
        //down
        return 0.0;
    }
    else if is_layout_index_ground(layout, i_index - i_width) {
        //up
        return std::f32::consts::PI;
    }
    else if is_layout_index_ground(layout, i_index + 1) {
        //right
        return std::f32::consts::PI / 2.0;
    }
    else if is_layout_index_ground(layout, i_index - 1) {
        //left
        return std::f32::consts::PI * 1.5;
    }
    return 0.0;
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
        return true
    }
    return match layout.chars().nth(index as usize) {
        Some(x) => x == '⬛',
        None => false,
    }

}

fn map_surrounding_ground_bitmask_to_tilemap_index(bitmask: u8) -> usize {
    let indices = [0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 13, 14, 13, 14, 15, 16, 15, 17, 13, 14, 13, 14, 15, 16, 15, 17, 18, 19, 18, 19, 20, 21, 20, 22, 18, 19, 18, 19, 23, 24, 23, 25, 13, 14, 13, 14, 15, 16, 15, 17, 13, 14, 13, 14, 15, 16, 15, 17, 26, 27, 26, 27, 28, 29, 28, 30, 26, 27, 26, 27, 31, 32, 31, 33, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 13, 34, 13, 34, 15, 35, 15, 36, 13, 34, 13, 34, 15, 35, 15, 36, 18, 37, 18, 37, 20, 38, 20, 39, 18, 37, 18, 37, 23, 40, 23, 41, 13, 34, 13, 34, 15, 35, 15, 36, 13, 34, 13, 34, 15, 35, 15, 36, 26, 42, 26, 42, 28, 43, 28, 44, 26, 42, 26, 42, 31, 45, 31, 46];
    return indices[bitmask as usize];
}







