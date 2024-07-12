

use bevy::{
    asset::ron,
    prelude::*,
};

use crate::stage_builder::stage_asset::{GroundTile, Stage};


pub fn save_stage() {
    let ground: Vec<usize> = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        1, 1, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 1, 1, 1, 1, 1, 1, 0, 1, 0,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 1,];
    let mut ground_tiles: Vec<GroundTile> = vec![];
    for i in 0..ground.len() {
        if ground[i] == 0 { continue; }
        let x = i % 10;
        let y = i / 10;
        ground_tiles.push(GroundTile {grid_pos: Vec2::new(x as f32, y as f32)});
    }

    let stage =  Stage {
        id: 0,
        spawn_translation: Vec3::default(),
        ground_tiles: ground_tiles,
        grid_width: 10,
        grid_height: 10,
    };
    let mut bytes: Vec<u8> = vec![];
    ron::ser::to_writer(&mut bytes, &stage).unwrap();



    let path = std::path::Path::new("assets/stage_0.stage");     
    let mut file = std::fs::File::create(&path).expect("yeet1");       
 
    use std::io::Write;
    file.write_all(&bytes).expect("yeet2"); 



}