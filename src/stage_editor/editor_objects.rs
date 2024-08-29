use bevy::prelude::*;


pub enum EditorStageObject {
    Spike { entity: Entity, rotation: f32 },
    Ground { entity: Entity },
    Spawn { entity: Entity },
    Spring { entity: Entity, rotation: f32 },
    PhantomBlock { entity: Entity },
    HalfSaw { entity: Entity, rotation: f32 },
    Key { entity: Entity },
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
            EditorStageObject::Key { entity } => *entity,
        }
    }
}


// For future me
//  Make a hashmap [grid_pos : EditorStageObject] in the EditorController
//  Remove Stage from EditorController
//  Instead, have Stage built from the hashmap on save
//  Hashmap means you can easily add and remove new objects, and tell if/what object exists in any given tile