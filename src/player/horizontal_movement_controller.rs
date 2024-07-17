
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ground::Grounded;

use super::wall_jump_controller::WallStuck;

#[derive(Component)]
pub struct GroundedHorizontalMovementController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub acceleration: f32,
    pub resistance: f32,
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
            let friction_diff = con.resistance * time.delta_seconds();
            vel.linvel.x -= vel.linvel.x.signum() * friction_diff.min(vel.linvel.x.abs());
        }

        vel.linvel.x = vel.linvel.x.abs().min(con.max_speed) * vel.linvel.x.signum();
    }
}



#[derive(Component)]
pub struct AirbourneHorizontalMovementController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub acceleration: f32,
    pub resistance: f32,
    pub max_speed: f32,
}


pub fn move_airbourne_horizontal_controller(
    mut query: Query<(&mut Velocity, &AirbourneHorizontalMovementController), (Without<Grounded>, Without<WallStuck>)>,    //todo: need an airbourne state, right now there are seaprate states for jumping
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut vel, con) in &mut query {

        let mut player_inputting = false;

        if input.pressed(con.right_key) {
            vel.linvel.x += con.acceleration * time.delta_seconds();
            player_inputting = true;
        }
        if input.pressed(con.left_key) {
            vel.linvel.x -= con.acceleration * time.delta_seconds();
            player_inputting = true;
        }

        if player_inputting == false {
            let friction_diff = con.resistance * time.delta_seconds();
            vel.linvel.x -= vel.linvel.x.signum() * friction_diff.min(vel.linvel.x.abs());
        }

        vel.linvel.x = vel.linvel.x.abs().min(con.max_speed) * vel.linvel.x.signum();
    }
}