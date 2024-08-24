
use bevy::prelude::*;

use super::enums::EditorItem;


#[derive(Resource)]
pub struct EditorController {
    current_item: EditorItem
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
}

impl EditorController {
    fn can_cycle_item(&self) -> bool {
        true
    }
}

impl Default for EditorController {
    fn default() -> Self {
        Self { 
            current_item: EditorItem::default()
         }
    }
}