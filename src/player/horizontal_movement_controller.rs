
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::stage_1::Grounded;

#[derive(Component)]
pub struct GroundedHorizontalMovementController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub acceleration: f32,
    pub friction: f32,
    pub max_speed: f32,
}


pub fn move_ground_horizontal_controller(
    mut query: Query<(&mut Velocity, &GroundedHorizontalMovementController), With<Grounded>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut vel, con) in &mut query {

        let mut player_inputting = false;

        if input.pressed(con.right_key) {
            vel.linvel.x += con.acceleration * time.delta_seconds();
            player_inputting = false;
        }
        if input.pressed(con.left_key) {
            vel.linvel.x -= con.acceleration * time.delta_seconds();
            player_inputting = false;
        }

        if player_inputting == false {
            let friction_diff = con.friction * time.delta_seconds();
            vel.linvel.x -= vel.linvel.x.signum() * friction_diff.min(vel.linvel.x.abs());
        }

        vel.linvel.x = vel.linvel.x.abs().min(con.max_speed) * vel.linvel.x.signum();
    }
}