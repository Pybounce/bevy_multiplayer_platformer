
use bevy::prelude::*;

use crate::stage::stage_builder::stage_creator::TILE_SIZE;

use super::enums::{EditorItem, EditorItemIconAtlasIndices};

const EDITOR_TILEMAP_SIZE: f32 = 16.0;

#[derive(Resource)]
pub struct EditorController {
    current_item: EditorItem,
    grid_size: IVec2,
    tile_size: f32
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
    pub fn world_to_grid_pos(&self, world_pos: Vec3) -> Vec3 {
        Vec3::new(
            ((world_pos.x / self.tile_size).trunc() + (world_pos.x.signum() * 0.5)) * self.tile_size, 
            ((world_pos.y /self.tile_size).trunc() + (world_pos.y.signum() * 0.5)) * self.tile_size, 
            world_pos.z)
    }
}

impl EditorController {
    fn can_cycle_item(&self) -> bool {
        true
    }
}

impl Default for EditorController {
    fn default() -> Self {
        Self { 
            current_item: EditorItem::default(),
            grid_size: IVec2::new(30, 30),
            tile_size: TILE_SIZE
         }
    }
}