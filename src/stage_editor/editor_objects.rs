use bevy::prelude::*;


pub enum EditorStageObject {
    Spike { entity: Entity, rotation: f32 },
    Ground { entity: Entity }
}


pub trait HasEntity { 
    fn entity(&self) -> Entity;
}

impl HasEntity for EditorStageObject {
    fn entity(&self) -> Entity {
        match self {
            EditorStageObject::Spike { entity, .. } => *entity,
            EditorStageObject::Ground { entity } => *entity,
        }
    }
}



// For future me
//  Make a hashmap [grid_pos : EditorStageObject] in the EditorController
//  Remove Stage from EditorController
//  Instead, have Stage built from the hashmap on save
//  Hashmap means you can easily add and remove new objects, and tell if/what object exists in any given tile