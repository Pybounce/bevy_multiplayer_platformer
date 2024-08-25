use bevy::{input::keyboard::KeyboardInput, prelude::*};
use controller::EditorController;
use item_icon::*;
use crate::common::{mouse::MouseData, states::{AppState, DespawnOnStateExit}};

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
            handle_save
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
        let mouse_pos = editor_con.world_to_grid_pos(mouse_data.position.extend(0.0));
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