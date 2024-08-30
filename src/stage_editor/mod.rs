use bevy::prelude::*;
use controller::{EditorController, GROUND_TILEMAP_SIZE};
use item_icon::*;
use crate::{camera::PixelPerfectTranslation, common::{mouse::MouseData, states::{AppState, DespawnOnStateExit}}, stage::stage_builder::stage_creator::TILE_SIZE};

mod enums;
mod controller;
mod item_icon;

pub struct StageEditorPlugin;

impl Plugin for StageEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageEditor), build_stage_editor)
        .add_systems(OnExit(AppState::StageEditor), teardown_stage_editor)
        .add_systems(Update, (
            (handle_current_item_change, add_item_icon, display_item_icon, move_item_icon, update_ground_atlas_indices),
            (handle_rotate, handle_placement, handle_grid_object_removals),
            handle_save,
            move_camera
        ).run_if(in_state(AppState::StageEditor)));
}
}

fn build_stage_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let object_atlas: Handle<Image> = asset_server.load("object_tilemap.png");
    let ground_atlas: Handle<Image> = asset_server.load("tilemap.png");
    commands.insert_resource(EditorController::new(&object_atlas, &ground_atlas));

    commands.spawn(Text2dBundle {
        text: Text::from_section("Stage Editor", TextStyle::default()),
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::StageEditor));

}

fn teardown_stage_editor(
    mut commands: Commands
) {
    commands.remove_resource::<EditorController>();
}


fn handle_placement(
    buttons: Res<ButtonInput<MouseButton>>,
    mut editor_con: ResMut<EditorController>,
    mouse_data: Res<MouseData>,
    mut commands: Commands,
    mut current_item_q: Query<Entity, With<ItemIcon>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(e) = current_item_q.get_single_mut() {
            let mouse_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
            if editor_con.try_place(mouse_pos, e.clone()) {
                commands.entity(e).remove::<ItemIcon>();
            }
        }

    }
}

fn handle_grid_object_removals(
    buttons: Res<ButtonInput<MouseButton>>,
    mut editor_con: ResMut<EditorController>,
    mouse_data: Res<MouseData>,
    mut commands: Commands
) {
    if buttons.just_pressed(MouseButton::Right) {
        let mouse_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
        editor_con.try_remove(mouse_pos, &mut commands);
    }
}

fn handle_save(
    input: Res<ButtonInput<KeyCode>>,
    mut editor_con: ResMut<EditorController>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        editor_con.try_save();
    }
}

fn handle_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut editor_con: ResMut<EditorController>
) {
    if input.just_pressed(KeyCode::KeyR) {
        editor_con.try_rotate();
    }
}

//TODO: Potentially move to moving the cam via clicking mouse3
fn move_camera(
    mut query: Query<&mut PixelPerfectTranslation, With<Camera>>,
    mouse_data: Res<MouseData>,
    time: Res<Time>
) {
    const CAMERA_MOVE_DEADZONE: f32 = 0.1;
    const CAMERA_MOVE_SPEED: f32 = 64.0;

    let mut direction = Vec3::ZERO;    
    if mouse_data.window_position_normalised.x >= 1.0 - CAMERA_MOVE_DEADZONE {
        direction += Vec3::X;
    }
    else if mouse_data.window_position_normalised.x <= CAMERA_MOVE_DEADZONE {
        direction -= Vec3::X;
    }
    if mouse_data.window_position_normalised.y <= CAMERA_MOVE_DEADZONE {
        direction += Vec3::Y;
    }
    else if mouse_data.window_position_normalised.y >= 1.0 - CAMERA_MOVE_DEADZONE {
        direction -= Vec3::Y;
    }

    for mut ppt in &mut query {
        ppt.translation += direction * CAMERA_MOVE_SPEED * time.delta_seconds();
    }
}


fn update_ground_atlas_indices(
    mut stage_entities_q: Query<&mut Sprite, Without<ItemIcon>>, 
    editor_con: Res<EditorController>,
    mouse_data: Res<MouseData>,
    mut item_icon_query: Query<&mut Sprite, With<ItemIcon>>,

) {
    let current_grid_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
    let adjacent_grid_positions = get_not_clockwise_adjacent_grid_positions_but_2_layers_hardcoded_because_thats_the_neutron_style(current_grid_pos);

    let mut ground_icon_grid_pos_opt: Option<IVec2> = None;

    match editor_con.current_item {
        enums::EditorItem::Ground => {
            if let Ok(mut s) = item_icon_query.get_single_mut() {
                let atlas_index = get_ground_atlas_index(&editor_con, current_grid_pos, None) as f32;
                let upper_left = Vec2::new(atlas_index % GROUND_TILEMAP_SIZE, (atlas_index / GROUND_TILEMAP_SIZE).trunc()) * TILE_SIZE;
                let lower_right = upper_left + TILE_SIZE;
                let atlas_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);
                s.rect = Some(atlas_rect);
                ground_icon_grid_pos_opt = Some(current_grid_pos);
            }
        },
        _ => (),
    }

    for adjacent_grid_pos in &adjacent_grid_positions {
        if let Some(stage_object) = editor_con.stage_grid.get(adjacent_grid_pos) {
            match stage_object {
                enums::EditorStageObject::Ground { entity } => {
                    if let Ok(mut s) = stage_entities_q.get_mut(*entity) {
                        let atlas_index = get_ground_atlas_index(&editor_con, *adjacent_grid_pos, ground_icon_grid_pos_opt) as f32;
                        let upper_left = Vec2::new(atlas_index % GROUND_TILEMAP_SIZE, (atlas_index / GROUND_TILEMAP_SIZE).trunc()) * TILE_SIZE;
                        let lower_right = upper_left + TILE_SIZE;
                        let atlas_rect = Rect::new(upper_left.x, upper_left.y, lower_right.x, lower_right.y);
                        s.rect = Some(atlas_rect);
                    }
                }
                _ => (),
            }
        }
    }


}

fn get_ground_atlas_index(
    editor_con: &Res<EditorController>,
    grid_pos: IVec2,
    ground_icon_grid_pos_opt: Option<IVec2>
) -> usize {
    let adjacent_grid_positions = get_clockwise_adjacent_grid_positions(grid_pos);

    let mut bitmask: u8 = 0;
    let mut current_bit: u8 = 1;

    for adjacent_grid_pos in &adjacent_grid_positions {
        if let Some(ground_icon_grid_pos) = ground_icon_grid_pos_opt {
            if ground_icon_grid_pos == *adjacent_grid_pos {
                bitmask |= current_bit;
                current_bit <<= 1;
                continue;
            }
        }
        if let Some(stage_object) = editor_con.stage_grid.get(adjacent_grid_pos) {
            match stage_object {
                enums::EditorStageObject::Ground { entity: _ } => bitmask |= current_bit,
                _ => (),
            }
        };

        current_bit <<= 1;
    }

    return map_surrounding_ground_bitmask_to_atlas_index(bitmask);

}

fn get_clockwise_adjacent_grid_positions(grid_pos: IVec2) -> Vec<IVec2> {
    return vec![
        IVec2::new(grid_pos.x, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 1),
    ];
}
fn get_not_clockwise_adjacent_grid_positions_but_2_layers_hardcoded_because_thats_the_neutron_style(grid_pos: IVec2) -> Vec<IVec2> {
    return vec![
        IVec2::new(grid_pos.x, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 1, grid_pos.y),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 1, grid_pos.y),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 1),

        IVec2::new(grid_pos.x, grid_pos.y + 2),
        IVec2::new(grid_pos.x + 1, grid_pos.y + 2),
        IVec2::new(grid_pos.x + 2, grid_pos.y + 2),
        IVec2::new(grid_pos.x - 1, grid_pos.y + 2),
        IVec2::new(grid_pos.x - 2, grid_pos.y) + 2,
        IVec2::new(grid_pos.x - 2, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 2, grid_pos.y + 1),
        IVec2::new(grid_pos.x + 2, grid_pos.y),
        IVec2::new(grid_pos.x - 2, grid_pos.y),
        IVec2::new(grid_pos.x + 2, grid_pos.y - 1),
        IVec2::new(grid_pos.x - 2, grid_pos.y - 1),
        IVec2::new(grid_pos.x, grid_pos.y - 2),
        IVec2::new(grid_pos.x + 1, grid_pos.y - 2),
        IVec2::new(grid_pos.x + 2, grid_pos.y - 2),
        IVec2::new(grid_pos.x - 1, grid_pos.y - 2),
        IVec2::new(grid_pos.x - 2, grid_pos.y) - 2,
    ];
}

fn map_surrounding_ground_bitmask_to_atlas_index(bitmask: u8) -> usize {
    let indices = [0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 13, 14, 13, 14, 15, 16, 15, 17, 13, 14, 13, 14, 15, 16, 15, 17, 18, 19, 18, 19, 20, 21, 20, 22, 18, 19, 18, 19, 23, 24, 23, 25, 13, 14, 13, 14, 15, 16, 15, 17, 13, 14, 13, 14, 15, 16, 15, 17, 26, 27, 26, 27, 28, 29, 28, 30, 26, 27, 26, 27, 31, 32, 31, 33, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 0, 1, 0, 1, 2, 3, 2, 4, 0, 1, 0, 1, 2, 3, 2, 4, 5, 6, 5, 6, 7, 8, 7, 9, 5, 6, 5, 6, 10, 11, 10, 12, 13, 34, 13, 34, 15, 35, 15, 36, 13, 34, 13, 34, 15, 35, 15, 36, 18, 37, 18, 37, 20, 38, 20, 39, 18, 37, 18, 37, 23, 40, 23, 41, 13, 34, 13, 34, 15, 35, 15, 36, 13, 34, 13, 34, 15, 35, 15, 36, 26, 42, 26, 42, 28, 43, 28, 44, 26, 42, 26, 42, 31, 45, 31, 46];
    return indices[bitmask as usize];
}