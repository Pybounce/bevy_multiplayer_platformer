
use std::collections::VecDeque;

use bevy::prelude::*;

pub struct EditorEventGroup {
    pub events: VecDeque<EditorEvent>,
    pub event_status: EditorEventStatus
}

impl EditorEventGroup {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            event_status: EditorEventStatus::Pending
        }
    }
}

pub enum EditorEvent {
    InsertGround { grid_pos: IVec2 },
    RemoveGround { grid_pos: IVec2 },
    InsertSpike { grid_pos: IVec2, rotation: f32 },
    RemoveSpike { grid_pos: IVec2, rotation: f32 },
    InsertSpawn { grid_pos: IVec2 },
    RemoveSpawn { grid_pos: IVec2 },
    InsertSpring { grid_pos: IVec2, rotation: f32 },
    RemoveSpring { grid_pos: IVec2, rotation: f32 },
    InsertPhantomBlock { grid_pos: IVec2 },
    RemovePhantomBlock { grid_pos: IVec2 },
}

//TODO: Remove status and assign each event an incrementing id so consumers can track read events on their own?
pub enum EditorEventStatus {
    /// Event has not been applied to the stage editor
    Pending, 
    /// Event has been applied to the stage editor, but marked as preview to show it's only for preview edits
    Preview, 
    // Event has been applied to editor, and is not a preview (ie the player has explicitely made a change)
    Committed
}
