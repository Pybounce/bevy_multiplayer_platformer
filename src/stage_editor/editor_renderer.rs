
use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::{controller::{EditorController, EDITOR_TILEMAP_SIZE}, enums::{EditorItem, IntervalBlockVariant, KeyVariant, LockBlockVariant}};

#[derive(Resource)]
pub struct EditorRenderer {
    stage_grid: HashMap<IVec2, Entity>,
    version: usize,
    full_refresh: bool
}


//Helper Methods
impl EditorRenderer {
    pub fn get_item_icon_atlas_rect(&self, editor_item: &EditorItem) -> Rect {
        let (index, tile_size) = match editor_item {
            EditorItem::Ground => (15.0, 16.0),
            EditorItem::Spike { .. } => (4.0, 16.0),
            EditorItem::Spawn => (18.0, 16.0),
            EditorItem::Spring { .. } => (5.0, 16.0),
            EditorItem::PhantomBlock => (21.0, 16.0),
            EditorItem::HalfSaw { .. } => (0.0, 16.0),
            EditorItem::Key { variant } => {
                        match variant {
                            KeyVariant::One => (255.0, 16.0),
                            KeyVariant::Two => (239.0, 16.0),
                            KeyVariant::Three => (223.0, 16.0),
                        }
                    },
            EditorItem::LockBlock { variant } => {
                        match variant {
                            LockBlockVariant::One => (254.0, 16.0),
                            LockBlockVariant::Two => (238.0, 16.0),
                            LockBlockVariant::Three => (222.0, 16.0),
                        }
                    },
            EditorItem::IntervalBlock { variant }=> {
                        match variant {
                            IntervalBlockVariant::On => (253.0, 16.0),
                            IntervalBlockVariant::Off => (237.0, 16.0),
                        }
                    },
            EditorItem::SawShooter { .. } => (27.0, 16.0),
        };

        let upper_left = Vec2::new(index % EDITOR_TILEMAP_SIZE, (index / EDITOR_TILEMAP_SIZE).trunc()) * tile_size;
        let lower_right = upper_left + tile_size;
        Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y)
    }
}

pub fn draw_editor(
    renderer_opt: Option<ResMut<EditorRenderer>>,
    editor_controller_opt: Option<Res<EditorController>>
) {
    if renderer_opt.is_none() || editor_controller_opt.is_none() {
        return;
    }
    let mut renderer = renderer_opt.unwrap();
    let editor_controller = editor_controller_opt.unwrap();

    //nothing to be updated
    if editor_controller.version == renderer.version { return; }
    
    //out of sync, renderer should never be ahead
    if editor_controller.version < renderer.version {
        renderer.full_refresh = true;
        return;
    }

    renderer.full_refresh = true;   //TEMPORARY


}

pub fn refresh_editor_renderer(    
    renderer_opt: Option<ResMut<EditorRenderer>>,
    editor_controller_opt: Option<Res<EditorController>>
) {
    if renderer_opt.is_none() || editor_controller_opt.is_none() {
        return;
    }
    let mut renderer = renderer_opt.unwrap();
    let editor_controller = editor_controller_opt.unwrap();

    if renderer.full_refresh == false { return; }


    for grid_pos in editor_controller.stage_grid.keys() {
        let editor_object = editor_controller.stage_grid[grid_pos];

    }


    renderer.version = editor_controller.version;
    renderer.full_refresh = false;
}