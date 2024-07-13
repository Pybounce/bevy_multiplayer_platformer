use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::stage_1::Grounded;



#[derive(Component)]
pub struct JumpController {
    pub key: KeyCode,
    pub force: f32,
    pub duration: f64,
    pub last_jump_pressed_time: f64,
    pub last_jump_released_time: f64,
    pub last_grounded: f64,
    pub coyote_time: f64
}

#[derive(Component)]
pub struct CanJump;
#[derive(Component)]
pub struct Jumping;
#[derive(Component)]
pub struct Falling;

pub fn maintain_player_jump(
    mut query: Query<(Entity, &mut JumpController)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    for (e, mut jc) in &mut query {
        if input.pressed(jc.key) 
        && time.elapsed_seconds_f64() - jc.last_jump_pressed_time < jc.duration 
        && jc.last_jump_released_time < jc.last_jump_pressed_time {
            commands.entity(e).try_insert(GravityScale(0.0));
        }
        else {
            commands.entity(e).try_insert(GravityScale(2.0));
        }
        if input.just_released(jc.key) {
            jc.last_jump_released_time = time.elapsed_seconds_f64(); //todo: wrapped??
        }
    }
}

pub fn begin_player_jump(
    mut query: Query<(&mut Velocity, &mut JumpController), With<CanJump>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    for (mut v, mut jc) in &mut query {
        if input.pressed(jc.key) {
            v.linvel.y = jc.force;
            jc.last_grounded -= jc.coyote_time; //todo: this sucks but it fixes being able to jump from the ground, and then jump again during coyote time
            jc.last_jump_pressed_time = time.elapsed_seconds_f64(); //todo: wrapped??
        }
    }
}

pub fn can_jump(
    query: Query<(Entity, &JumpController)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, jc) in &query {
        if time.elapsed_seconds_f64() - jc.last_grounded < jc.coyote_time {
            commands.entity(e).try_insert(CanJump);
        }
        else {
            commands.entity(e).remove::<CanJump>();
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

