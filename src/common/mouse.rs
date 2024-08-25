
use bevy::{prelude::*, window::PrimaryWindow};


#[derive(Resource, Default)]
pub struct MouseData {
    pub position: Vec2
}


pub fn update_mouse_data(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_data: ResMut<MouseData>,

) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_data.position = world_position;
    }
}