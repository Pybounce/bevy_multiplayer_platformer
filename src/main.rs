

use bevy::{
    prelude::*, window::{ close_on_esc, PresentMode }, winit::{ UpdateMode, WinitSettings }
};

mod local_player;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use local_player::{ spawn_local_player, move_player };

mod networking;
use networking::GameNetworkingPlugin;

mod networked_players;
use networked_players::{remove_disconnected_players, spawn_new_players};

mod stage_1;
use stage_1::{spawn_stage, check_grounded};

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
        .add_plugins(DefaultPlugins.set(window_settings))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(GameNetworkingPlugin)
        .add_systems(Startup, (spawn_camera, spawn_local_player, spawn_stage))
        .add_systems(Update, (check_grounded, move_player, close_on_esc, spawn_new_players, remove_disconnected_players))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        });
}

