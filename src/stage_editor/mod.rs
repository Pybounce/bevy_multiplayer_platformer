use bevy::{prelude::*, ui::FocusPolicy};

use crate::common::states::{AppState, DespawnOnStateExit};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
};


pub struct StageEditorPlugin;

impl Plugin for StageEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StageEditor), build_stage_editor)
        .add_systems(Update, hovering);
    }
}

fn build_stage_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn(Text2dBundle {
        text: Text::from_section("Stage Editor", TextStyle::default()),
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::StageEditor));


}

fn hovering(
    mut query: Query<(&mut BackgroundColor, &Interaction)>
) {
    for (mut bg, i) in &mut query {
        match i {
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.0, 1.0, 0.0);
            },
            Interaction::None => {
                bg.0 = Color::srgb(1.0, 0.0, 0.0);
            },
            _ => ()
        }
    }
}

//ok so you have a box with overflow on clip y
// then you have the list container inside that
// the list container itself contains a list of elements (list items)
// but the bottom of the list container is overflowed so we don't see it
// then we move the list container (ie every item in the list) up a bit, and that creates scrolling
    // since it is now clipping the top y and bottom y etc
