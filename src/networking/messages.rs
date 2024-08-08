use serde::{Deserialize, Serialize};

use crate::player::look_state::PlayerLookState;




#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct NewLocationMessage {
    pub code: usize,
    pub translation_x: f32,
    pub translation_y: f32,
    pub velocity_x: f32, 
    pub velocity_y: f32,
    pub look_state: PlayerLookState
}


