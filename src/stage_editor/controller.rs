
use bevy::{prelude::*, scene::ron};

use crate::stage::stage_builder::{stage_asset::{GroundTile, Spike, Stage}, stage_creator::TILE_SIZE};

use super::enums::{EditorItem, EditorItemIconAtlasIndices};

const EDITOR_TILEMAP_SIZE: f32 = 16.0;

#[derive(Resource)]
pub struct EditorController {
    current_item: EditorItem,
    tile_size: f32,
    stage: Stage
}


impl EditorController {
    pub fn cycle_next_item(&mut self) {
        if self.can_cycle_item() {
            self.current_item = self.current_item.cycle_next();
        }
    }
    pub fn cycle_prev_item(&mut self) {
        if self.can_cycle_item() {
            self.current_item = self.current_item.cycle_prev();
        }
    }
    pub fn get_item_icon_atlas_rect(&self) -> Rect {
        let (index, tile_size) = match self.current_item {
            EditorItem::Ground => (15.0, 16.0),
            EditorItem::Spike => (4.0, 16.0),
        };

        let upper_left = Vec2::new(index % EDITOR_TILEMAP_SIZE, (index / EDITOR_TILEMAP_SIZE).trunc()) * tile_size;
        let lower_right = upper_left + tile_size;
        Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y)
    }
    pub fn should_display_item_icon(&self) -> bool {
        true
    }
    /// Returns the grid position in world space <br/>
    /// In other words, snaps the world pos to the grid and returns that snapped world pos
    pub fn world_to_grid_world_pos(&self, world_pos: Vec3) -> Vec3 {
        let grid_pos = self.world_to_grid_pos(world_pos);
        Vec3::new(
            (grid_pos.x as f32 + (world_pos.x.signum() * 0.5)) * self.tile_size, 
            (grid_pos.y as f32 + (world_pos.y.signum() * 0.5)) * self.tile_size, 
            world_pos.z)
    }
    /// Returns the grid position
    pub fn world_to_grid_pos(&self, world_pos: Vec3) -> IVec2 {
        IVec2::new(
            (world_pos.x / self.tile_size).trunc() as i32, 
            (world_pos.y /self.tile_size).trunc()as i32) 
    }

    pub fn try_place(&mut self, grid_pos: IVec2) -> bool {
        if !self.can_place(grid_pos) { return false; }
        match self.current_item {
            EditorItem::Ground => {
                self.stage.ground_tiles.push(GroundTile {
                    grid_pos: Vec2::new(grid_pos.x as f32, grid_pos.y as f32),
                    tilemap_index: 0
                });
                return true;
            },
            EditorItem::Spike => {
                self.stage.spikes.push(Spike {
                    grid_pos: Vec2::new(grid_pos.x as f32, grid_pos.y as f32),
                    rotation: 0.0
                });
                return true;
            },
        }
    }
    pub fn can_place(&self, grid_pos: IVec2) -> bool {
        true
    }
    pub fn try_save(&self) -> bool {
        if !self.can_save() { return false; }
        
        let mut bytes: Vec<u8> = vec![];
        ron::ser::to_writer(&mut bytes, &self.stage).unwrap();
        let name = String::from("assets/stage_".to_owned() + &self.stage.id.to_string() + ".stage");
        let path = std::path::Path::new(&name);     
        let mut file = std::fs::File::create(&path).expect("yeet1");       
     
        use std::io::Write;
        file.write_all(&bytes).expect("yeet2"); 
        return true;
    }
}

impl EditorController {
    fn can_cycle_item(&self) -> bool {
        true
    }
    fn can_save(&self) -> bool {
        true
    }
}

impl Default for EditorController {
    fn default() -> Self {
        Self { 
            current_item: EditorItem::default(),
            tile_size: TILE_SIZE,
            stage: Stage::new(100, IVec2::new(30, 30))
         }
    }
}