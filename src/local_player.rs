use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{game::player::player_states::PlayerState, player::{horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController}, jump_controller::JumpController}, stage_1::Groundable};

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 2.0, 0.0);
const PLAYER_MAX_SPEED: Vec2 = Vec2::new(300.0, 1000.0);
const PLAYER_ACCELERATION: f32 = 2000.0;
const PLAYER_HORIZONTAL_FRICTION: f32 = 600.0;
const PLAYER_JUMP_SPEED: f32 = 300.0;
const PLAYER_JUMP_DURATION: f64 = 0.3;

#[derive(Component)]
pub struct LocalPlayer;

pub fn spawn_local_player(mut commands: Commands) {
    commands
        .spawn(LocalPlayer)
        .insert(SpriteBundle {
            transform: Transform {
                scale: PLAYER_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(GravityScale(2.0))
        .insert(Groundable)
        .insert(CollidingEntities::default())
        .insert(PlayerState::Dead)
        .insert(JumpController {
            key: KeyCode::KeyW,
            force: PLAYER_JUMP_SPEED,
            duration: PLAYER_JUMP_DURATION,
            last_jump_pressed_time: 0.0,
            last_jump_released_time: 0.0,
            last_grounded: 0.0,
            coyote_time: 0.3,
        })
        .insert(GroundedHorizontalMovementController {
            left_key: KeyCode::KeyA,
            right_key: KeyCode::KeyD,
            acceleration: PLAYER_ACCELERATION,
            resistance: PLAYER_HORIZONTAL_FRICTION,
            max_speed: 300.0,
        })
        .insert(AirbourneHorizontalMovementController {
            left_key: KeyCode::KeyA,
            right_key: KeyCode::KeyD,
            acceleration: PLAYER_ACCELERATION / 2.0,
            resistance: PLAYER_HORIZONTAL_FRICTION / 2.0,
            max_speed: 300.0,
        });
}


