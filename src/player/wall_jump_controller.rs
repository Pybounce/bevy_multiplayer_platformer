use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground::Grounded, wall::TouchingWall};

use super::jump_controller::{CoyoteGrounded, JumpController};

#[derive(Component)]
pub struct WallJumpController {
    /// Force applied when jumping into the wall
    pub force_in: Vec2,
    /// Force applied when jumping away from the wall
    pub force_out: Vec2,
    pub friction_coefficient: f32
}

pub fn begin_player_wall_jump(
    mut query: Query<(&mut Velocity, &mut JumpController, &TouchingWall, &WallJumpController), 
        (Without<Grounded>, Without<CoyoteGrounded>)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    for (mut v, mut jc, w, wjc) in &mut query {
        if input.just_pressed(jc.key) {
            v.linvel = match w {
                TouchingWall::Left => wjc.force_in * Vec2::new(-1.0, 1.0),
                TouchingWall::Right => wjc.force_in,
            }; 
            wjc.force_out;
            jc.last_grounded -= jc.coyote_time; //todo: this sucks but it fixes being able to jump from the ground, and then jump again during coyote time
            jc.last_jump_pressed_time = time.elapsed_seconds_f64(); //todo: wrapped??
        }
    }
}