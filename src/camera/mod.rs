
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic);
}

pub fn move_camera(
    mut camera_query: Query<(&mut Velocity, &Transform), With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    time: Res<Time>
) {
    let (mut cv, ct) = camera_query.single_mut();
    let pt = player_query.get_single();
    match pt {
        Ok(pt) => {
            let distance = ct.translation.truncate().distance(pt.translation.truncate());
            let speed = distance * 2.5;
            let dir = (pt.translation - ct.translation).truncate().normalize();

            if distance < 10.0 {
                cv.linvel = Vec2::ZERO;
                return;
            }
            cv.linvel = dir * speed;
        }
        Err(_) => {
            cv.linvel = Vec2::ZERO;
            let current_linvel = cv.linvel.clone();
            //cv.linvel -= current_linvel.signum() * 300.0 * time.delta_seconds();
        },
    }
}

pub fn zoom_camera(
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    player_query: Query<(), (With<LocalPlayer>, Without<Camera>)>,
    time: Res<Time>
) {

    let mut op = camera_query.single_mut();

    let target_zoom = match player_query.get_single() {
        Ok(_) => 1.0,
        Err(_) => 1.1,
    };
    let delta = target_zoom - op.scale;
    let speed = (delta.abs() * 0.5).max(0.3);
    let diff = speed * time.delta_seconds();
    //op.scale += delta.signum() * diff.abs().min(delta.abs());
}