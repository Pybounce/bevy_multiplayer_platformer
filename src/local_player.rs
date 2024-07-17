use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::death::Killable, ground::Groundable, player::{death::Respawnable, horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController}, jump_controller::JumpController, wall_jump_controller::{WallJumpController, WallStickable}}, stage::stage_objects::StageObject, wall::{Wall, Wallable}};

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 2.0, 0.0);
const PLAYER_MAX_SPEED: Vec2 = Vec2::new(300.0, 1000.0);
const PLAYER_ACCELERATION: f32 = 2000.0;
const PLAYER_HORIZONTAL_FRICTION: f32 = 600.0;

const PLAYER_JUMP_SPEED: f32 = 300.0;
const PLAYER_JUMP_DURATION: f64 = 0.3;
const PLAYER_WALL_JUMP_IN_FORCE: Vec2 = Vec2::new(250.0, 300.0);
const PLAYER_WALL_JUMP_OUT_FORCE: Vec2 = Vec2::new(250.0, 300.0);
const PLAYER_WALL_FRICTION_COEFFICIENT: f32 = 0.03;

const PLAYER_RESPAWN_DELAY: f64 = 0.5;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    local_player_market: LocalPlayer,
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    ccd: Ccd,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    velocity: Velocity,
    gravity_scale: GravityScale,
    groundable_marker: Groundable,
    wallable_marker: Wallable,
    colliding_entities: CollidingEntities,
    jump_controller: JumpController,
    wall_jump_controller: WallJumpController,
    wall_stickable: WallStickable,
    grounded_horizontal_movement_controller: GroundedHorizontalMovementController,
    airbourne_horizontal_movement_controller: AirbourneHorizontalMovementController,
    respawnable: Respawnable,
    stage_object: StageObject,
    killable: Killable
}
impl LocalPlayerBundle {
    pub fn new(pos: Vec3, stage_id: usize) -> Self {
        let mut p = LocalPlayerBundle::default();
        p.sprite_bundle.transform.translation = pos;
        p.respawnable.translation = pos;
        p.stage_object.stage_id = stage_id;
        return p;
    }
}
impl Default for LocalPlayerBundle {
    fn default() -> Self {
        LocalPlayerBundle {
            local_player_market: LocalPlayer,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            collider: Collider::ball(0.5),
            restitution: Restitution::coefficient(0.0),
            friction: Friction::coefficient(0.0),
            velocity: Velocity::linear(Vec2::ZERO),
            gravity_scale: GravityScale(2.0),
            groundable_marker: Groundable,
            colliding_entities: CollidingEntities::default(),
            jump_controller: JumpController {
                key: KeyCode::KeyW,
                force: PLAYER_JUMP_SPEED,
                duration: PLAYER_JUMP_DURATION,
                last_jump_pressed_time: 0.0,
                last_jump_released_time: 0.0,
                last_grounded: 0.0,
                coyote_time: 0.3,
            },
            wall_jump_controller: WallJumpController {
                force_in: PLAYER_WALL_JUMP_IN_FORCE,
                force_out: PLAYER_WALL_JUMP_OUT_FORCE,
                friction_coefficient: PLAYER_WALL_FRICTION_COEFFICIENT,
            },
            wall_stickable: WallStickable {
                wall_stick_time: 1.0,
            },
            grounded_horizontal_movement_controller: GroundedHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION,
                resistance: PLAYER_HORIZONTAL_FRICTION,
                max_speed: PLAYER_MAX_SPEED.x,
            },
            airbourne_horizontal_movement_controller: AirbourneHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION / 2.0,
                resistance: PLAYER_HORIZONTAL_FRICTION / 2.0,
                max_speed: PLAYER_MAX_SPEED.x,
            },
            respawnable: Respawnable {
                translation: Vec3::default(),
                delay_in_seconds: PLAYER_RESPAWN_DELAY,
            },
            stage_object: StageObject { stage_id: usize::max_value() },
            killable: Killable,
            wallable_marker: Wallable,
        }
    }
}


