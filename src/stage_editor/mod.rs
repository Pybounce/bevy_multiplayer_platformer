use bevy::prelude::*;
use controller::EditorController;
use item_icon::*;
use crate::common::states::{AppState, DespawnOnStateExit};

mod enums;
mod controller;
mod item_icon;

pub struct StageEditorPlugin;

impl Plugin for StageEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageEditor), build_stage_editor)
        .add_systems(OnExit(AppState::StageEditor), teardown_stage_editor)
        .add_systems(Update, handle_current_item_change.run_if(in_state(AppState::StageEditor)))
        .add_systems(Update, (add_item_icon, display_item_icon, move_item_icon).run_if(in_state(AppState::StageEditor)));
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

