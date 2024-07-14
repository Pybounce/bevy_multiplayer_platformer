
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::local_player::LocalPlayer;

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
        })
        .insert(Velocity::default());
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    time: Res<Time>
) {
    let mut ct = camera_query.single_mut();
    let pt = player_query.get_single();
    match pt {
        Ok(pt) => {
            let distance = ct.translation.truncate().distance(pt.translation.truncate());
            let speed = distance * 3.0;
            let dir = (pt.translation - ct.translation).truncate().normalize();
            ct.translation += (dir * speed * time.delta_seconds()).extend(0.0);
        }
        Err(_) => (),
    }
}
