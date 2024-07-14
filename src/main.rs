

use bevy::{
    prelude::*, window::{ close_on_esc, PresentMode }, winit::{ UpdateMode, WinitSettings }
};

mod local_player;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use camera::{move_camera, spawn_camera};
use common::{death::despawn_death_marked, states::StatesPlugin};
use game::GamePlugin;

mod networking;
use networking::{networked_players::{remove_disconnected_players, spawn_new_players}, GameNetworkingPlugin};

mod stage_select;
use obstacles::check_insta_kill_collisions;
use player::{common::check_player_out_of_bounds, death::trigger_dead_local_player_respawn, horizontal_movement_controller::{move_airbourne_horizontal_controller, move_ground_horizontal_controller}, jump_controller::{begin_player_jump, can_jump, check_jump_fall_states, maintain_player_jump, update_last_grounded}, spawner::spawn_local_players};
use ground::check_grounded;
use stage::stage_builder::StageBuilderPlugin;
use stage_select::StageSelectPlugin;

mod common;

mod game;
mod player;
mod stage;
mod obstacles;
mod camera;
pub mod ground;

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
        .add_systems(Update, (check_insta_kill_collisions, trigger_dead_local_player_respawn, spawn_local_players, check_grounded, check_player_out_of_bounds, move_airbourne_horizontal_controller, move_ground_horizontal_controller, update_last_grounded, maintain_player_jump, begin_player_jump, can_jump, check_jump_fall_states, despawn_death_marked))
        .run();
}

