
use bevy::{prelude::*, render::render_resource::Texture};
use bevy_rapier2d::prelude::*;
use crate::{common::states::{AppState, GameState, StageState, StageTransitionData}, stage_1::Ground};

use super::{stage_asset_loader::Stage, stage_goal::StageGoal, stage_manager::StageData};

const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct StagePiece;

fn spawn_tile(x: f32, y: f32, commands: &mut Commands, tex_handle: &Handle<Image>, atlas_index: u8) {
    let sprite_rect_x = (atlas_index % 16) as f32 * 16.0;
    let sprite_rect_y = (atlas_index / 16) as f32 * 16.0;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 16.0, sprite_rect_y + 16.0);
    
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
            translation: Vec3::new(x * TILE_SIZE, y * TILE_SIZE, 0.0),
            ..default()
        },
        texture: tex_handle.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            rect: Some(sprite_rect),
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

fn spawn_background_tile(x: f32, y: f32, commands: &mut Commands, tex_handle: &Handle<Image>, atlas_index: u8) {
    let sprite_rect_x = (atlas_index % 16) as f32 * 16.0;
    let sprite_rect_y = (atlas_index / 16) as f32 * 16.0;
    let sprite_rect = Rect::new(sprite_rect_x, sprite_rect_y, sprite_rect_x + 16.0, sprite_rect_y + 16.0);
    
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
            translation: Vec3::new(x * TILE_SIZE, y * TILE_SIZE, -10.0),
            ..default()
        },
        texture: tex_handle.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            rect: Some(sprite_rect),
            ..default()
        },
        ..default()
    })
    .insert(StagePiece);
}

fn spawn_goal(x: f32, y: f32, commands: &mut Commands) {
    commands
    .spawn(StageGoal)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
            translation: Vec3::new(x * TILE_SIZE, y * TILE_SIZE, 0.0),
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
                let x = i % data.tiles_width;
                let y = i / data.tiles_height;
                if data.tiles[i] == 0 {
                    spawn_background_tile(x as f32, -(y as f32), &mut commands, &texture, 0);
                }
                else if data.tiles[i] == 3 {
                    spawn_tile(x as f32, -(y as f32), &mut commands, &texture, 1);
                }
                if data.tiles[i] == 4 {
                    spawn_tile(x as f32, -(y as f32), &mut commands, &texture, 2);
                }
                if data.tiles[i] == 5 {
                    spawn_tile(x as f32, -(y as f32), &mut commands, &texture, 3);
                }
                if data.tiles[i] == 6 {
                    spawn_tile(x as f32, -(y as f32), &mut commands, &texture, 4);
                }
                else if data.tiles[i] == 1 {
                    spawn_goal(x as f32, -(y as f32), &mut commands);
                }
                else if data.tiles[i] == 2 {
                    spawn = Vec3::new(x as f32 * TILE_SIZE, y as f32 * -TILE_SIZE, 0.0)
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
