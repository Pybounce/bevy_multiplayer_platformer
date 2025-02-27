
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
    IntervalBlock { entity: Entity, is_active: bool },
    SawShooter { entity: Entity, rotation: f32 },
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
            EditorStageObject::IntervalBlock { entity, .. } => *entity,
            EditorStageObject::SawShooter { entity, .. } => *entity,
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
    Key(KeyVariant) = 6,
    LockBlock(LockBlockVariant) = 7,
    IntervalBlock(IntervalBlockVariant) = 8,
    SawShooter = 9,
}

impl EditorItem {
    pub fn cycle_next(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Key(KeyVariant::One),
            EditorItem::Key(_) => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spring,
            EditorItem::Spring => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::HalfSaw,
            EditorItem::HalfSaw => EditorItem::LockBlock(LockBlockVariant::One),
            EditorItem::LockBlock(_) => EditorItem::IntervalBlock(IntervalBlockVariant::On),
            EditorItem::IntervalBlock(_) => EditorItem::SawShooter,
            EditorItem::SawShooter => EditorItem::Ground,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            EditorItem::SawShooter => EditorItem::IntervalBlock(IntervalBlockVariant::On),
            EditorItem::Ground => EditorItem::SawShooter,
            EditorItem::IntervalBlock(_) => EditorItem::LockBlock(LockBlockVariant::One),
            EditorItem::LockBlock(_) => EditorItem::HalfSaw,
            EditorItem::HalfSaw => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::Spring,
            EditorItem::Spring => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Key(KeyVariant::One),
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
            EditorItem::IntervalBlock(variant) => EditorItem::IntervalBlock(variant.cycle_next()),
            EditorItem::SawShooter => EditorItem::SawShooter,
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
            EditorItem::Key(variant) => EditorItem::Key(variant.cycle_prev()),
            EditorItem::IntervalBlock(variant) => EditorItem::IntervalBlock(variant.cycle_prev()),
            EditorItem::SawShooter => EditorItem::SawShooter,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum KeyVariant {
    #[default]
    One,
    Two,
    Three,
}

impl KeyVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            KeyVariant::One => KeyVariant::Two,
            KeyVariant::Two => KeyVariant::Three,
            KeyVariant::Three => KeyVariant::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            KeyVariant::One => KeyVariant::Three,
            KeyVariant::Three => KeyVariant::Two,
            KeyVariant::Two => KeyVariant::One,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum LockBlockVariant {
    #[default]
    One,
    Two,
    Three,
}

impl LockBlockVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            LockBlockVariant::One => LockBlockVariant::Two,
            LockBlockVariant::Two => LockBlockVariant::Three,
            LockBlockVariant::Three => LockBlockVariant::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            LockBlockVariant::One => LockBlockVariant::Three,
            LockBlockVariant::Three => LockBlockVariant::Two,
            LockBlockVariant::Two => LockBlockVariant::One,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum IntervalBlockVariant {
    #[default]
    On,
    Off,
}

impl IntervalBlockVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            IntervalBlockVariant::On => IntervalBlockVariant::Off,
            IntervalBlockVariant::Off => IntervalBlockVariant::On,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            IntervalBlockVariant::On => IntervalBlockVariant::Off,
            IntervalBlockVariant::Off => IntervalBlockVariant::On,
        }
    }
}