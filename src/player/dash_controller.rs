
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{common::physics::gravity::Gravity, local_player::{FORCE_MUL, MAX_HORIZONTAL_SPEED}};

use super::look_state::PlayerLookState;

pub const DASH_COOLDOWN: f32 = 1.0;
pub const DASH_SPEED: f32 = 400.0 * FORCE_MUL;
pub const DASH_DURATION: f32 = 0.2;
pub const DASH_END_SPEED: f32 = MAX_HORIZONTAL_SPEED * FORCE_MUL;
pub const DASH_KEY: KeyCode = KeyCode::Space;

#[derive(Component)]
pub struct DashController {
    pub key: KeyCode,
    pub dash_speed: f32,
    pub cooldown_timer: Timer,
    pub duration_timer: Timer,
    pub dash_direction_sign: f32,
    pub dash_end_speed: f32
}

impl Default for DashController {
    fn default() -> Self {
        let mut duration_timer = Timer::from_seconds(DASH_DURATION, TimerMode::Once);
        duration_timer.tick(Duration::from_secs(234));
        Self { 
            key: DASH_KEY,
            dash_speed: DASH_SPEED,
            cooldown_timer: Timer::from_seconds(DASH_COOLDOWN, TimerMode::Once),
            duration_timer: duration_timer,
            dash_direction_sign: 1.0, 
            dash_end_speed: DASH_END_SPEED
        }
    }
}

pub fn start_dashing(
    mut query: Query<(&mut DashController, &PlayerLookState)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,

) {
    for (mut dash_controller, player_look_state) in &mut query {
        dash_controller.cooldown_timer.tick(time.delta());
        if input.pressed(dash_controller.key) && dash_controller.cooldown_timer.finished() {
            dash_controller.cooldown_timer.reset();
            dash_controller.duration_timer.reset();
            dash_controller.dash_direction_sign = match player_look_state {
                PlayerLookState::LookingLeft => -1.0,
                PlayerLookState::LookingRight => 1.0,
            };
        }
    }
}

pub fn apply_dashing(
    mut query: Query<(&mut DashController, &mut Velocity, &mut Gravity)>,
    time: Res<Time>
) {
    for (mut dash_controller, mut velocity, mut gravity) in &mut query {
        dash_controller.duration_timer.tick(time.delta());
        if !dash_controller.duration_timer.finished() {
            velocity.linvel = Vec2::new(dash_controller.dash_direction_sign * dash_controller.dash_speed, 0.0);
        }
        else if dash_controller.duration_timer.just_finished() {
            velocity.linvel = Vec2::new(dash_controller.dash_direction_sign * dash_controller.dash_end_speed, 0.0);
            //gravity.current_force = 0.0;
        }
    }
}