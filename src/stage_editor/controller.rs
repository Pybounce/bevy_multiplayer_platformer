
use bevy::{prelude::*, scene::ron, utils::hashbrown::HashMap};

use crate::{common::limited_stack::LimitedStack, stage::stage_builder::{stage_asset::{GroundTile, HalfSaw, IntervalBlock, Key, LockBlock, PhantomBlock, Spike, Spring, Stage}, stage_creator::TILE_SIZE}};

use super::{enums::*, events::*};

const EDITOR_TILEMAP_SIZE: f32 = 16.0;
pub const GROUND_TILEMAP_SIZE: f32 = 7.0;

#[derive(Resource)]
pub struct EditorController {
    pub current_item: EditorItem,
    tile_size: f32,
    /// Tracks whether or not the latest stage updates have been saved
    saved: bool,
    pub object_atlas: Handle<Image>,
    pub ground_atlas: Handle<Image>,
    pub stage_grid: HashMap<IVec2, EditorStageObject>,
    grid_size: IVec2,
    pub rotation: f32,
    event_groups: LimitedStack<EditorEventGroup>
}

///API
impl EditorController {
    pub fn new(object_atlas: &Handle<Image>, ground_atlas: &Handle<Image>) -> Self {
        
        Self { 
            current_item: EditorItem::default(),
            tile_size: TILE_SIZE,
            saved: false,
            object_atlas: object_atlas.clone(),
            ground_atlas: ground_atlas.clone(),
            grid_size: IVec2::splat(30),
            stage_grid: HashMap::new(),
            rotation: 0.0,
            event_groups: LimitedStack::new(50)
         }
    }

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
    pub fn cycle_next_item_variant(&mut self) {
        if self.can_cycle_item_variant() {
            self.current_item = self.current_item.cycle_next_variant();
        }
    }
    pub fn cycle_prev_item_variant(&mut self) {
        if self.can_cycle_item_variant() {
            self.current_item = self.current_item.cycle_prev_variant();
        }
    }
    pub fn get_item_icon_atlas_rect(&self) -> Rect {
        let (index, tile_size) = match self.current_item {
            EditorItem::Ground => (15.0, 16.0),
            EditorItem::Spike => (4.0, 16.0),
            EditorItem::Spawn => (18.0, 16.0),
            EditorItem::Spring => (5.0, 16.0),
            EditorItem::PhantomBlock => (21.0, 16.0),
            EditorItem::HalfSaw => (0.0, 16.0),
            EditorItem::Key(variant) => {
                match variant {
                    KeyVariant::One => (255.0, 16.0),
                    KeyVariant::Two => (239.0, 16.0),
                    KeyVariant::Three => (223.0, 16.0),
                }
            },
            EditorItem::LockBlock(variant) => {
                match variant {
                    LockBlockVariant::One => (254.0, 16.0),
                    LockBlockVariant::Two => (238.0, 16.0),
                    LockBlockVariant::Three => (222.0, 16.0),
                }
            },
            EditorItem::IntervalBlock(variant) => {
                match variant {
                    IntervalBlockVariant::On => (253.0, 16.0),
                    IntervalBlockVariant::Off => (237.0, 16.0),
                }
            },
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

    pub fn try_place(&mut self, grid_pos: IVec2, entity: Entity) -> bool {
        if !self.can_place(grid_pos) { return false; }
        match self.current_item {
            EditorItem::Ground => {
                self.stage_grid.insert(grid_pos, EditorStageObject::Ground { entity } );
            },
            EditorItem::Spike => {
                self.stage_grid.insert(grid_pos, EditorStageObject::Spike { entity: entity, rotation: self.rotation });
            },
            EditorItem::Spawn => {
                self.stage_grid.insert(grid_pos, EditorStageObject::Spawn { entity } );
            },
            EditorItem::Spring => {
                self.stage_grid.insert(grid_pos, EditorStageObject::Spring { entity: entity, rotation: self.rotation });
            },
            EditorItem::PhantomBlock => {
                self.stage_grid.insert(grid_pos, EditorStageObject::PhantomBlock { entity: entity });
            },
            EditorItem::HalfSaw => {
                self.stage_grid.insert(grid_pos, EditorStageObject::HalfSaw { entity: entity, rotation: self.rotation });
            },
            EditorItem::Key(variant) => {
                match variant {
                    KeyVariant::One => self.stage_grid.insert(grid_pos, EditorStageObject::Key { entity: entity, trigger_id: 1 }),
                    KeyVariant::Two => self.stage_grid.insert(grid_pos, EditorStageObject::Key { entity: entity, trigger_id: 2 }),
                    KeyVariant::Three => self.stage_grid.insert(grid_pos, EditorStageObject::Key { entity: entity, trigger_id: 3 }),
                };
            },
            EditorItem::LockBlock(variant) => {
                match variant {
                    LockBlockVariant::One => self.stage_grid.insert(grid_pos, EditorStageObject::LockBlock { entity: entity, trigger_id: 1 }),
                    LockBlockVariant::Two => self.stage_grid.insert(grid_pos, EditorStageObject::LockBlock { entity: entity, trigger_id: 2 }),
                    LockBlockVariant::Three => self.stage_grid.insert(grid_pos, EditorStageObject::LockBlock { entity: entity, trigger_id: 3 }),
                };
            },
            EditorItem::IntervalBlock(variant) => {
                match variant {
                    IntervalBlockVariant::On => self.stage_grid.insert(grid_pos, EditorStageObject::IntervalBlock { entity: entity, is_active: true }),
                    IntervalBlockVariant::Off => self.stage_grid.insert(grid_pos, EditorStageObject::IntervalBlock { entity: entity, is_active:  false }),
                };
            },
        }
        self.saved = false;
        return true;
    }
    
    pub fn can_place(&self, grid_pos: IVec2) -> bool {
        grid_pos.x > 0 && 
        grid_pos.x < self.grid_size.x as i32 - 1 &&
        grid_pos.y > 0 && 
        grid_pos.y < self.grid_size.y as i32 - 1 &&
        !self.stage_grid.contains_key(&grid_pos)
    }

    pub fn try_remove(&mut self, grid_pos: IVec2, commands: &mut Commands) -> bool{
        if !self.can_remove(grid_pos) { return false; }

        if let Some((_entry_key, entry_val)) = self.stage_grid.remove_entry(&grid_pos) {
            commands.entity(entry_val.entity()).despawn_recursive();
        }
        return true;
    }
    
    pub fn try_save(&mut self) -> bool {

        if !self.can_save() { return false; }

        let stage = self.build_stage();

        let mut bytes: Vec<u8> = vec![];
        ron::ser::to_writer(&mut bytes, &stage).unwrap();
        let name = String::from("assets/stage_".to_owned() + &stage.id.to_string() + ".stage");
        let path = std::path::Path::new(&name);     
        let mut file = std::fs::File::create(&path).expect("yeet1");       
        
        use std::io::Write;
        file.write_all(&bytes).expect("yeet2");
        self.saved = true;
        return true;
    }
    pub fn try_rotate(&mut self) -> bool {

        if !self.can_rotate() { return false; }
        self.rotation -= std::f32::consts::FRAC_PI_2;
        if self.rotation <= 0.0 {
            self.rotation = std::f32::consts::PI * 2.0;
        }

        return true;
    }
    pub fn can_remove(&self, grid_pos: IVec2) -> bool {
        grid_pos.x > 0 && 
        grid_pos.x < self.grid_size.x as i32 - 1 &&
        grid_pos.y > 0 && 
        grid_pos.y < self.grid_size.y as i32 - 1 &&
        self.stage_grid.contains_key(&grid_pos)
    }
}

/// Helper Functions
impl EditorController {
    fn get_spawn_grid_pos(&self) -> Option<IVec2> {
        for (key, val) in self.stage_grid.iter() {
            match val {
                EditorStageObject::Spawn { entity: _ } => {
                    return Some(*key);
                },
                _ => ()
            }
        }
        return None;
    }
    fn can_cycle_item(&self) -> bool {
        true
    }
    fn can_cycle_item_variant(&self) -> bool {
        true
    }
    fn can_save(&self) -> bool {
        true
    }
    fn can_rotate(&self) -> bool {
        true
    }
    fn build_stage(&self) -> Stage {
        let mut stage: Stage = Stage::new(4, self.grid_size);
        for (grid_pos, stage_editor_obj) in &self.stage_grid {
            match stage_editor_obj {
                EditorStageObject::Spike { entity: _, rotation } => {
                    stage.spikes.push(Spike {
                        grid_pos: grid_pos.as_vec2(),
                        rotation: *rotation,
                    });
                },
                EditorStageObject::Ground { entity: _ } => {
                    stage.ground_tiles.push(GroundTile {
                        grid_pos: grid_pos.as_vec2(),
                        tilemap_index: 0,   //TODO
                    });
                },
                EditorStageObject::Spawn { entity: _ } => stage.spawn_grid_pos = grid_pos.as_vec2(),
                EditorStageObject::Spring { entity: _, rotation } => {
                    stage.springs.push(Spring {
                        grid_pos: grid_pos.as_vec2(),
                        rotation: *rotation,
                    });
                }
                EditorStageObject::PhantomBlock { entity: _ } => {
                    stage.phantom_blocks.push(PhantomBlock {
                        grid_pos: grid_pos.as_vec2(),
                    });
                },
                EditorStageObject::HalfSaw { entity: _, rotation } => {
                    stage.half_saws.push(HalfSaw {
                        grid_pos: grid_pos.as_vec2(),
                        rotation: *rotation,
                        movement_path_opt: None
                    });
                },
                EditorStageObject::Key { entity: _ , trigger_id} => {
                    stage.keys.push(Key {
                        grid_pos: grid_pos.as_vec2(),
                        trigger_id: *trigger_id,
                    });
                },
                EditorStageObject::LockBlock { entity: _, trigger_id } => {
                    stage.lock_blocks.push(LockBlock {
                        grid_pos: grid_pos.as_vec2(),
                        trigger_id: *trigger_id,
                    });
                },
                EditorStageObject::IntervalBlock { entity: _, is_active } => {
                    stage.interval_blocks.push(IntervalBlock {
                        grid_pos: grid_pos.as_vec2(),
                        is_active: *is_active
                    });
                },
            }
        }
        return stage;
    }
}

/// Events
impl EditorController {
    fn create_removal_event(&self, grid_pos: IVec2) -> Option<EditorEvent> {
        if let Some(stage_object) = self.stage_grid.get(&grid_pos) {
            match stage_object {
                EditorStageObject::Spike { entity: _, rotation } => {
                    return EditorEvent::RemoveSpike { grid_pos, rotation: *rotation }.into();
                },
                EditorStageObject::Ground { entity: _ } => {
                    return EditorEvent::RemoveGround { grid_pos: grid_pos }.into();
                },
                EditorStageObject::Spawn { entity: _ } => {
                    return EditorEvent::RemoveSpawn { grid_pos: grid_pos }.into();
                },
                EditorStageObject::Spring { entity: _, rotation } => {
                    return EditorEvent::RemoveSpring { grid_pos, rotation: *rotation }.into();
                },
                EditorStageObject::PhantomBlock { entity } => todo!(),
                EditorStageObject::HalfSaw { entity, rotation } => todo!(),
                EditorStageObject::Key { entity, trigger_id } => todo!(),
                EditorStageObject::LockBlock { entity, trigger_id } => todo!(),
                EditorStageObject::IntervalBlock { entity, is_active } => todo!(),
            }
        }
        return None;
    }
    fn add_removal_event_to_group(&self, group: &mut EditorEventGroup, grid_pos: IVec2) 
    {
        if let Some(removal_event) = self.create_removal_event(grid_pos) {
            group.events.push_front(removal_event);
        }
    }
    fn insert_ground_at(&mut self, grid_pos: IVec2) {
        let mut event_group = EditorEventGroup::new();
        self.add_removal_event_to_group(&mut event_group, grid_pos);
        event_group.events.push_front(EditorEvent::InsertGround { grid_pos });
        self.event_groups.push(event_group);
    }
    fn insert_spike_at(&mut self, grid_pos: IVec2, rotation: f32) {
        let mut event_group = EditorEventGroup::new();
        self.add_removal_event_to_group(&mut event_group, grid_pos);
        event_group.events.push_front(EditorEvent::InsertSpike { grid_pos, rotation });
        self.event_groups.push(event_group);
    }
    fn insert_spawn_at(&mut self, grid_pos: IVec2) {
        let mut event_group = EditorEventGroup::new();
        if let Some(spawn_pos) = self.get_spawn_grid_pos() {
            if !spawn_pos == grid_pos { 
                self.add_removal_event_to_group(&mut event_group, spawn_pos);
            }
        }
        self.add_removal_event_to_group(&mut event_group, grid_pos);

        event_group.events.push_front(EditorEvent::InsertSpawn { grid_pos });
        self.event_groups.push(event_group);
    }
    fn insert_spring_at(&mut self, grid_pos: IVec2, rotation: f32) {
        let mut event_group = EditorEventGroup::new();
        self.add_removal_event_to_group(&mut event_group, grid_pos);
        event_group.events.push_front(EditorEvent::InsertSpring { grid_pos, rotation });
        self.event_groups.push(event_group);
    }
    fn insert_phantom_block_at(&mut self, grid_pos: IVec2) {
        let mut event_group = EditorEventGroup::new();
        self.add_removal_event_to_group(&mut event_group, grid_pos);
        event_group.events.push_front(EditorEvent::InsertPhantomBlock { grid_pos });
        self.event_groups.push(event_group);
    }
}