
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


pub fn camera_shake_on_death(
    mut commands: Commands,
    query: Query<(), (With<LocalPlayer>, With<DeathMarker>)>,
    camera_query: Query<Entity, With<Camera>>
) {
    if let Ok(e) = camera_query.get_single() {
        if query.iter().len() > 0 {
            commands.entity(e).try_insert(Shake {
                current_offset: Vec2::ZERO,
                force: 15.0,
                duration: Some(0.15),
                shake_delay: 0.015,
                current_delay: 0.0,
            });
        }
    }

}