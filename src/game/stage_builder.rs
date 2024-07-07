
use bevy::{prelude::*, render::render_resource::Texture};
use bevy_rapier2d::prelude::*;
use crate::{common::states::{AppState, GameState, StageState, StageTransitionData}, stage_1::Ground};

use super::{stage_asset_loader::Stage, stage_goal::StageGoal, stage_manager::StageData};


#[derive(Component)]
pub struct StagePiece;

fn spawn_tile(x: f32, y: f32, commands: &mut Commands, tex_handle: &Handle<Image>) {
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(32.0, 32.0, 1.0),
            translation: Vec3::new(x * 32.0, y * 32.0, 0.0),
            ..default()
        },
        texture: tex_handle.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            rect: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
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
            scale: Vec3::new(32.0, 32.0, 1.0),
            translation: Vec3::new(x * 32.0, y * 32.0, 0.0),
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
    mut stage_state: ResMut<NextState<StageState>>,
    stage_handles: Res<StageAssetLoadingHandles>,
    stage_assets: Res<Assets<Stage>>,
    asset_server: Res<AssetServer>
) {

    match asset_server.load_state(&stage_handles.stage_handle) {
        bevy::asset::LoadState::NotLoaded => {
            app_state.set(AppState::StageSelect);
            game_state.set(GameState::NA);
            stage_state.set(StageState::NA);
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed => {
            app_state.set(AppState::StageSelect);
            game_state.set(GameState::NA);
            stage_state.set(StageState::NA);
        },
    }

    let stage_asset = stage_assets.get(&stage_handles.stage_handle);
    let texture: Handle<Image> = asset_server.load("test_sprite_sheet.png");

    match stage_asset {
        Some(data) => {
            let mut spawn = data.spawn_translation;
            for i in 0..data.tiles.len() {
                if data.tiles[i] == 0 { continue; }
                let x = i % data.tiles_width;
                let y = i / data.tiles_height;
                if data.tiles[i] == 1 {
                    spawn_tile(x as f32, -(y as f32), &mut commands, &texture);
                }
                else if data.tiles[i] == 2 {
                    spawn_goal(x as f32, -(y as f32), &mut commands);
                }
                else if data.tiles[i] == 3 {
                    spawn = Vec3::new(x as f32 * 32.0, y as f32 * -32.0, 0.0)
                }
            }

            commands.insert_resource(
                StageData {
                    stage_id: stage_transition_data.target_stage_id, 
                    respawn_translation: spawn
                });
                game_state.set(GameState::Playing);
                stage_state.set(StageState::Loaded);
            },
        None => {
            app_state.set(AppState::StageSelect);
            game_state.set(GameState::NA);
            stage_state.set(StageState::NA);
        },
    }

    
}

#[derive(Resource, Default)]
pub struct StageAssetLoadingHandles {
    stage_handle: Handle<Stage>
}

pub fn load_stage_handles(
    asset_server: Res<AssetServer>,
    mut stage_handles: ResMut<StageAssetLoadingHandles>,
    stage_transition_data: Res<StageTransitionData>,
) {
    //let a = asset_server.load
    stage_handles.stage_handle = asset_server.load(format!("stage_{}.stage", stage_transition_data.target_stage_id));
}


pub fn despawn_stage(
    query: Query<Entity, With<StagePiece>>,
    mut commands: Commands
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
