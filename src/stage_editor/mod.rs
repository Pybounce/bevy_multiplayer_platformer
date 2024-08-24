use bevy::{input::mouse::MouseWheel, prelude::*};
use editor_controller::EditorController;
use crate::common::states::{AppState, DespawnOnStateExit};

mod enums;
mod editor_controller;

pub struct StageEditorPlugin;

impl Plugin for StageEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageEditor), build_stage_editor)
        .add_systems(OnExit(AppState::StageEditor), teardown_stage_editor)
        .add_systems(Update, handle_current_item_change.run_if(in_state(AppState::StageEditor)));
    }
}

fn build_stage_editor(
    mut commands: Commands,
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

fn handle_current_item_change(
    mut editor_con: ResMut<EditorController>,
    mut mouse_wheel_events: EventReader<MouseWheel>
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        match mouse_wheel_event.y > 0.0 {
            true => editor_con.cycle_next_item(),
            false => editor_con.cycle_prev_item(),
        }
    }
}