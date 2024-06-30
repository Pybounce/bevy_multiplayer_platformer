
use std::time::Duration;

use bevy::{
    prelude::*, time::common_conditions::on_timer, window::{ close_on_esc, PresentMode }, winit::{ UpdateMode, WinitSettings }
};

mod player;
use player::{ spawn_player, move_player };

mod networking_exmaple;
use networking_exmaple::{ receive_messages, send_message, start_socket };
fn main() {
    let winit_settings = WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    };
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "Legend of the Octo-Parakeet".into(),
            name: Some("Legend of the Octo-Parakeet".into()),
            resolution: (1600.0, 900.0).into(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    
    App::new()
        .insert_resource(winit_settings)
        .add_plugins(DefaultPlugins.set(window_settings))
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, (move_player, close_on_esc))
        .add_systems(Startup, start_socket)
        .add_systems(Update, receive_messages)
        .add_systems(
            Update,
            send_message.run_if(on_timer(Duration::from_secs(5))),
        )
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