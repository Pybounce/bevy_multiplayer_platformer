use serde::{Deserialize, Serialize};




#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct NewLocationMessage {
    pub code: usize,
    pub translation_x: f32,
    pub translation_y: f32
}


