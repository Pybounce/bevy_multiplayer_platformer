
use bevy::prelude::*;

pub enum EditorStageObject {
    Spike { entity: Entity, rotation: f32 },
    Ground { entity: Entity },
    Spawn { entity: Entity },
    Spring { entity: Entity, rotation: f32 },
    PhantomBlock { entity: Entity },
    HalfSaw { entity: Entity, rotation: f32 },
    Key { entity: Entity, trigger_id: usize },
    LockBlock { entity: Entity, trigger_id: usize },
}


pub trait HasEntity { 
    fn entity(&self) -> Entity;
}

impl HasEntity for EditorStageObject {
    fn entity(&self) -> Entity {
        match self {
            EditorStageObject::Spike { entity, .. } => *entity,
            EditorStageObject::Spring { entity, .. } => *entity,
            EditorStageObject::Ground { entity } => *entity,
            EditorStageObject::Spawn { entity } => *entity,
            EditorStageObject::PhantomBlock { entity } => *entity,
            EditorStageObject::HalfSaw { entity, .. } => *entity,
            EditorStageObject::Key { entity, .. } => *entity,
            EditorStageObject::LockBlock { entity, .. } => *entity,
        }
    }
}



#[derive(Default, Copy, Clone, Debug)]
#[repr(u8)]
pub enum EditorItem {
    #[default]
    Ground = 0,
    Spike = 1,
    Spawn = 2,
    Spring = 3,
    PhantomBlock = 4,
    HalfSaw = 5,
    Key(KeyVarient) = 6,
    LockBlock(LockBlockVarient) = 7,
}

impl EditorItem {
    pub fn cycle_next(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Key(KeyVarient::One),
            EditorItem::Key(_) => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spring,
            EditorItem::Spring => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::HalfSaw,
            EditorItem::HalfSaw => EditorItem::LockBlock(LockBlockVarient::One),
            EditorItem::LockBlock(_) => EditorItem::Ground,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::LockBlock(LockBlockVarient::One),
            EditorItem::LockBlock(_) => EditorItem::HalfSaw,
            EditorItem::HalfSaw => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::Spring,
            EditorItem::Spring => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Key(KeyVarient::One),
            EditorItem::Key(_) => EditorItem::Ground,
        }
    }
    pub fn cycle_next_variant(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Ground,
            EditorItem::Key(variant) => EditorItem::Key(variant.cycle_next()),
            EditorItem::Spike => EditorItem::Spike,
            EditorItem::Spawn => EditorItem::Spawn,
            EditorItem::Spring => EditorItem::Spring,
            EditorItem::PhantomBlock => EditorItem::PhantomBlock,
            EditorItem::HalfSaw => EditorItem::HalfSaw,
            EditorItem::LockBlock(variant) => EditorItem::LockBlock(variant.cycle_next()),
        }
    }
    pub fn cycle_prev_variant(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Ground,
            EditorItem::LockBlock(variant) => EditorItem::LockBlock(variant.cycle_prev()),
            EditorItem::HalfSaw => EditorItem::HalfSaw,
            EditorItem::PhantomBlock => EditorItem::PhantomBlock,
            EditorItem::Spring => EditorItem::Spring,
            EditorItem::Spawn => EditorItem::Spawn,
            EditorItem::Spike => EditorItem::Spike,
            EditorItem::Key(variant) => EditorItem::Key(variant.cycle_next()),
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum KeyVarient {
    #[default]
    One,
    Two,
    Three,
}

impl KeyVarient {
    pub fn cycle_next(&self) -> Self {
        match self {
            KeyVarient::One => KeyVarient::Two,
            KeyVarient::Two => KeyVarient::Three,
            KeyVarient::Three => KeyVarient::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            KeyVarient::One => KeyVarient::Three,
            KeyVarient::Three => KeyVarient::Two,
            KeyVarient::Two => KeyVarient::One,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum LockBlockVarient {
    #[default]
    One,
    Two,
    Three,
}

impl LockBlockVarient {
    pub fn cycle_next(&self) -> Self {
        match self {
            LockBlockVarient::One => LockBlockVarient::Two,
            LockBlockVarient::Two => LockBlockVarient::Three,
            LockBlockVarient::Three => LockBlockVarient::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            LockBlockVarient::One => LockBlockVarient::Three,
            LockBlockVarient::Three => LockBlockVarient::Two,
            LockBlockVarient::Two => LockBlockVarient::One,
        }
    }
}