
use bevy::{prelude::*, transform::commands};

use crate::ground::Grounded;

use super::horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController};

#[derive(Component)]
pub enum PlayerLookState {
    LookingLeft,
    LookingRight
}



pub fn update_player_airborn_look_state(
    query: Query<(Entity, &AirbourneHorizontalMovementController), Without<Grounded>>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    for (e, con) in &query {
        if input.pressed(con.right_key) {
            commands.entity(e).try_insert(PlayerLookState::LookingRight);
        }
        if input.pressed(con.left_key) {
            commands.entity(e).try_insert(PlayerLookState::LookingLeft);
        }

    }
}

pub fn update_player_grounded_look_state(
    query: Query<(Entity, &GroundedHorizontalMovementController), With<Grounded>>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    for (e, con) in &query {
        if input.pressed(con.right_key) {
            commands.entity(e).try_insert(PlayerLookState::LookingRight);
        }
        if input.pressed(con.left_key) {
            commands.entity(e).try_insert(PlayerLookState::LookingLeft);
        }

    }
}