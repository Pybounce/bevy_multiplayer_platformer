

use bevy::{
    prelude::*, window::{ close_on_esc, PresentMode }, winit::{ UpdateMode, WinitSettings }
};

mod local_player;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use common::{death_marker::despawn_death_marked, states::StatesPlugin};
use game::GamePlugin;
use local_player::LocalPlayer;

mod networking;
use networking::{networked_players::{remove_disconnected_players, spawn_new_players}, GameNetworkingPlugin};

mod stage_select;
use player::{common::check_player_out_of_bounds, death::trigger_dead_local_player_respawn, horizontal_movement_controller::{move_airbourne_horizontal_controller, move_ground_horizontal_controller}, jump_controller::{begin_player_jump, can_jump, check_jump_fall_states, maintain_player_jump, update_last_grounded}, spawner::spawn_local_players};
use stage_1::check_grounded;
use stage::stage_builder::StageBuilderPlugin;
use stage_select::StageSelectPlugin;

mod common;

mod game;
mod player;
mod stage;

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
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(window_settings).set(ImagePlugin::default_nearest()))
        .add_plugins(StatesPlugin)
        .add_plugins(StageBuilderPlugin)
        .add_plugins(StageSelectPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameNetworkingPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (move_camera, close_on_esc, spawn_new_players, remove_disconnected_players))
        .add_systems(Update, (trigger_dead_local_player_respawn, spawn_local_players, check_grounded, check_player_out_of_bounds, move_airbourne_horizontal_controller, move_ground_horizontal_controller, update_last_grounded, maintain_player_jump, begin_player_jump, can_jump, check_jump_fall_states, despawn_death_marked))
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
    let player_transform = player_query.get_single();
    match player_transform {
        Ok(t) => camera_transform.translation = t.translation,
        Err(_) => (),
    }
    
}
