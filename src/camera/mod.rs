
use bevy::{core::Zeroable, prelude::*};

use crate::{common::{death::DeathMarker, shake::Shake}, local_player::LocalPlayer};

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        });
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.get_single();
    match player_transform {
        Ok(t) => camera_transform.translation = t.translation,
        Err(_) => (),
    }
}
