
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{common::states::{AppState, GameState, StageTransitionData}, stage_1::Ground};

use super::{stage_goal::StageGoal, stage_manager::StageData};


#[derive(Component)]
pub struct StagePiece;

struct StageBuildingData {
    tiles: Vec::<u32>,
    tiles_width: usize,
    tiles_height: usize,
    spawn_translation: Vec3
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
            color: Color::WHITE,
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
    .insert(StagePiece);
}

fn spawn_goal(x: f32, y: f32, commands: &mut Commands) {
    commands
    .spawn(StageGoal)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(30.0, 30.0, 1.0),
            translation: Vec3::new(x * 30.0, y * 30.0, 0.0),
            //rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 60.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::GOLD,
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Sensor)
    .insert(Restitution::coefficient(0.0))
    .insert(Friction::coefficient(0.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(StagePiece);
}

pub fn spawn_stage_vec(
    mut commands: Commands,
    stage_transition_data: Res<StageTransitionData>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {

    let stage_build_data = try_get_stage_data(stage_transition_data.target_stage_id);
    match stage_build_data {
        Some(data) => {
            for i in 0..data.tiles.len() {
                if data.tiles[i] == 0 { continue; }
                let x = i % data.tiles_width;
                let y = i / data.tiles_height;
                if data.tiles[i] == 1 {
                    spawn_tile(x as f32, -(y as f32), &mut commands);
                }
                else if data.tiles[i] == 2 {
                    spawn_goal(x as f32, -(y as f32), &mut commands);
                }
            }

            commands.insert_resource(
                StageData {
                    stage_id: stage_transition_data.target_stage_id, 
                    respawn_translation: data.spawn_translation
                });
            game_state.set(GameState::Playing);
            error!("STAGE BUILT");
        },
        None => {
            app_state.set(AppState::StageSelect);
            game_state.set(GameState::NA);
        },
    }

    
}

fn try_get_stage_data(stage_id: usize) -> Option<StageBuildingData> {
    warn!("stage: {}", stage_id);
    if stage_id == 0 {
        return Some(StageBuildingData {
            tiles: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2,
                1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            tiles_height: 9,
            tiles_width: 9,
            spawn_translation: Vec3::default()
        });
    }
    else if stage_id == 1 {
        return Some(StageBuildingData {
            tiles: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 2, 0, 0,
                1, 1, 1, 0, 0, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 1, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            tiles_height: 9,
            tiles_width: 9,
            spawn_translation: Vec3::default()
        });
    }
    else {
        return None;
    }
}

pub fn despawn_stage(
    query: Query<Entity, With<StagePiece>>,
    mut commands: Commands
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
