mod fps;
use bevy::prelude::*;

use self::fps::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps_stuff).add_systems(Update, update_fps_ui);
    }
}