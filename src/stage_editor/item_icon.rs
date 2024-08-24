use bevy::{input::mouse::MouseWheel, prelude::*, transform::commands, window::PrimaryWindow};

use crate::{common::states::{AppState, DespawnOnStateExit}, stage::stage_builder::stage_creator::TILE_SIZE};

use super::controller::EditorController;

#[derive(Component)]
pub struct ItemIcon;

pub fn add_item_icon(
    mut commands: Commands,
    query: Query<Entity, With<ItemIcon>>,
    editor_con: Res<EditorController>,
    asset_server: Res<AssetServer>
) {
    let mut first_item = true;
    for e in &query {
        if first_item {
            first_item = false;
            continue;
        }

        commands.entity(e).despawn();
    }

    if first_item == true {
        //no item exists
        let object_tilemap_handle: Handle<Image> = asset_server.load("object_tilemap.png");

        commands.spawn(SpriteBundle {
            texture: object_tilemap_handle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(editor_con.get_item_icon_atlas_rect()),
                ..default()
            },
            ..default()
        })
        .insert(ItemIcon)
        .insert(DespawnOnStateExit::App(AppState::StageEditor));
    }
}

pub fn display_item_icon(
    mut editor_con: ResMut<EditorController>
) {

}


pub fn move_item_icon(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut item_icon_query: Query<&mut Transform, With<ItemIcon>>,
    editor_con: Res<EditorController>,

) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok(mut t) = item_icon_query.get_single_mut() {
            t.translation = editor_con.world_to_grid_pos(world_position.extend(t.translation.z));
        }
    }
}

pub fn handle_current_item_change(
    mut editor_con: ResMut<EditorController>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut current_item_q: Query<&mut Sprite, With<ItemIcon>>
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        match mouse_wheel_event.y > 0.0 {
            true => editor_con.cycle_next_item(),
            false => editor_con.cycle_prev_item(),
        }
    }
    if let Ok(mut s) = current_item_q.get_single_mut() {
        s.rect = Some(editor_con.get_item_icon_atlas_rect());
    }
}

