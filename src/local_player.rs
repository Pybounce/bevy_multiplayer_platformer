use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::stage_1::{Groundable, Grounded};

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 2.0, 0.0);
const PLAYER_MAX_SPEED: Vec2 = Vec2::new(200.0, 1000.0);
const PLAYER_ACCELERATION: f32 = 800.0;
const PLAYER_HORIZONTAL_FRICTION: f32 = 200.0;
const PLAYER_JUMP_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct LocalPlayer {
    move_up_key: KeyCode,
    move_down_key: KeyCode,
    move_right_key: KeyCode,
    move_left_key: KeyCode,
    acceleration: f32,
    horizontal_friction: f32,
    max_speed: Vec2,
    jump_speed: f32
}

pub fn spawn_local_player(mut commands: Commands) {
    commands
        .spawn(LocalPlayer {
            move_up_key: KeyCode::KeyW,
            move_down_key: KeyCode::KeyS,
            move_right_key: KeyCode::KeyD,
            move_left_key: KeyCode::KeyA,
            acceleration: PLAYER_ACCELERATION,
            horizontal_friction: PLAYER_HORIZONTAL_FRICTION,
            max_speed: PLAYER_MAX_SPEED,
            jump_speed: PLAYER_JUMP_SPEED
        })
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
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.0))
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(GravityScale(2.0))
        .insert(Groundable)
        .insert(CollidingEntities::default());
}

pub fn move_player(
    mut query: Query<(&mut Velocity, &LocalPlayer, Option<&Grounded>)>, 
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut v, p, grounded_opt) in &mut query.iter_mut() {



        let mut new_direction = Vec2::new(0.0, 0.0);


        if input.pressed(p.move_right_key) {
            new_direction += Vec2::new(1.0, 0.0);
        }
        if input.pressed(p.move_left_key) {
            new_direction -= Vec2::new(1.0, 0.0);
        }
        if new_direction.length() > 0.00001 {
            v.linvel.x +=
                new_direction.normalize().x * p.acceleration * time.delta_seconds();
        } else if v.linvel.x.abs() > 0.00001 {
            v.linvel.x -= (p.horizontal_friction * time.delta_seconds()) * v.linvel.x.signum();
        }

        if let Some(_) = grounded_opt {
            if input.pressed(p.move_up_key) {
                 v.linvel.y = p.jump_speed;
             }
         }


        v.linvel.x = v.linvel.x.abs().min(p.max_speed.x) * v.linvel.x.signum();
        v.linvel.y = v.linvel.y.abs().min(p.max_speed.y) * v.linvel.y.signum();

    }
}

