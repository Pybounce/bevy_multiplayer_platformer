use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground::Grounded, wall::TouchingWall};

use super::{gravity::Gravity, wall_jump_controller::WallJumpController};



#[derive(Component)]
pub struct JumpController {
    pub key: KeyCode,
    pub force: f32,
    pub duration: f64,
    pub last_jump_pressed_time: f64,
    pub last_jump_released_time: f64,
    pub last_grounded: f64,
    pub coyote_time: f64,
}

#[derive(Component)]
pub struct CoyoteGrounded;
#[derive(Component)]
pub struct Jumping;
#[derive(Component)]
pub struct Falling;

pub fn apply_wall_friction(
    mut query: Query<(&mut Velocity, &WallJumpController), With<TouchingWall>>,
    time: Res<Time>
) {
    for (mut v, wjc) in &mut query {
        if v.linvel.y < 0.0 {
            // we want to simulate grabbing the wall
            // which would only happen when sliding down
            // sliding up should be fast
            v.linvel.y -= wjc.friction_coefficient * v.linvel.y.powi(2) * v.linvel.y.signum() * time.delta_seconds();
        }
    }
}

pub fn maintain_player_jump(
    mut query: Query<(&mut JumpController, &mut Gravity)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    for (mut jc, mut g) in &mut query {
        if input.pressed(jc.key) 
        && time.elapsed_seconds_f64() - jc.last_jump_pressed_time < jc.duration 
        && jc.last_jump_released_time < jc.last_jump_pressed_time {

        }
        else {
            g.current_force = g.max_force;
        }
        if input.just_released(jc.key) {
            jc.last_jump_released_time = time.elapsed_seconds_f64(); //todo: wrapped??
        }
    }
}

pub fn begin_player_jump(
    mut query: Query<(&mut Velocity, &mut JumpController, &mut Gravity), Or<(With<Grounded>, With<CoyoteGrounded>)>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    for (mut v, mut jc, mut g) in &mut query {
        if input.pressed(jc.key) {
            g.current_force = 0.0;
            v.linvel.y = jc.force;
            jc.last_grounded -= jc.coyote_time; //todo: this sucks but it fixes being able to jump from the ground, and then jump again during coyote time
            jc.last_jump_pressed_time = time.elapsed_seconds_f64(); //todo: wrapped??
        }
    }
}

pub fn is_coyote_grounded(
    query: Query<(Entity, &JumpController), Without<Grounded>>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, jc) in &query {
        if time.elapsed_seconds_f64() - jc.last_grounded < jc.coyote_time {
            commands.entity(e).try_insert(CoyoteGrounded);
        }
        else {
            commands.entity(e).remove::<CoyoteGrounded>();
        }
    }
}

pub fn update_last_grounded(
    mut query: Query<&mut JumpController, With<Grounded>>,
    time: Res<Time>
) {
    for mut jc in &mut query {
        jc.last_grounded = time.elapsed_seconds_f64(); //todo: wrapped??
    }
}


pub fn check_jump_fall_states(
    query: Query<(Entity, &Velocity, Option<&Grounded>)>,
    mut commands: Commands
) {
    for (e, v, g) in &query {
        if let Some(_) = g {
            commands.entity(e).remove::<Jumping>();
            commands.entity(e).remove::<Falling>();
            continue;
        }
        if v.linvel.y.abs() < 0.0001 {
            //no vertical movement
            commands.entity(e).remove::<Jumping>();
            commands.entity(e).remove::<Falling>();
        }
        else if v.linvel.y > 0.0 {
            //going up
            commands.entity(e).remove::<Falling>();
            commands.entity(e).try_insert(Jumping);        
        }
        else if v.linvel.y < 0.0 {
            //going down
            commands.entity(e).remove::<Jumping>();
            commands.entity(e).try_insert(Falling);        
        }
    }
}

