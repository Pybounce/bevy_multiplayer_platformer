use bevy::{input::keyboard::KeyboardInput, math::VectorSpace, prelude::*};
use controller::EditorController;
use item_icon::*;
use crate::{camera::PixelPerfectTranslation, common::{mouse::MouseData, states::{AppState, DespawnOnStateExit}}};

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
            (handle_current_item_change, add_item_icon, display_item_icon, move_item_icon),
            handle_placement,
            handle_save,
            move_camera
        ).run_if(in_state(AppState::StageEditor)));
}
}

fn build_stage_editor(
    mut commands: Commands
) {

    commands.insert_resource(EditorController::default());
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
    mouse_data: Res<MouseData>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mouse_pos = editor_con.world_to_grid_pos(mouse_data.world_position.extend(0.0));
        editor_con.try_place(mouse_pos);
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