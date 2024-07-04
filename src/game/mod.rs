use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::states::{AppState, DespawnOnStateExit, GameState, StageData}, stage_1::{check_grounded, Ground}};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_stage_vec)
        .add_systems(Update, check_grounded
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Playing)));
    }
}




fn spawn_tile(x: f32, y: f32, commands: &mut Commands) {
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(30.0, 30.0, 1.0),
            translation: Vec3::new(x * 30.0, y * 30.0, 0.0),
            //rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 60.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(0.0))
    .insert(Friction::coefficient(0.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(DespawnOnStateExit::App(AppState::Game));
}

pub fn spawn_stage_vec(
    mut commands: Commands
) {
    let width = 9;
    let height = 9;
    let stage: Vec::<u32> = vec![
    0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 1, 1, 1, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,];


    for i in 0..stage.len() {
        if stage[i] == 0 { continue; }
        let x = i % width;
        let y = i / height;
        spawn_tile(x as f32, -(y as f32), &mut commands);
    }
}