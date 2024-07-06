

use bevy::{
    prelude::*, window::{ close_on_esc, PresentMode }, winit::{ UpdateMode, WinitSettings }
};

mod local_player;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use common::states::StatesPlugin;
use game::GamePlugin;
use local_player::{ spawn_local_player, LocalPlayer };

mod networking;
use networking::{networked_players::{remove_disconnected_players, spawn_new_players}, GameNetworkingPlugin};

mod stage_select;
use player::{horizontal_movement_controller::{move_airbourne_horizontal_controller, move_ground_horizontal_controller}, jump_controller::{begin_player_jump, can_jump, check_jump_fall_states, maintain_player_jump, update_last_grounded}};
use stage_select::StageSelectPlugin;

mod common;

mod game;
mod player;

pub mod stage_1;

fn main() {
    let winit_settings = WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    };
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "Legend of the Octo-Parakeet".into(),
            name: Some("Legend of the Octo-Parakeet".into()),
            resolution: (800.0, 450.0).into(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    
    App::new()
        .insert_resource(winit_settings)
        .add_plugins(DefaultPlugins.set(window_settings).set(ImagePlugin::default_nearest()))
        .add_plugins(StatesPlugin)
        .add_plugins(StageSelectPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameNetworkingPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (spawn_camera, spawn_local_player))
        .add_systems(Update, (move_camera, close_on_esc, spawn_new_players, remove_disconnected_players))
        .add_systems(Update, (move_airbourne_horizontal_controller, move_ground_horizontal_controller, update_last_grounded, maintain_player_jump, begin_player_jump, can_jump, check_jump_fall_states))
        .insert_resource(Msaa::Sample8)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        });
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation = player_transform.translation;
}
