
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

    commands.spawn(Text2dBundle {
        text: Text::from_section("value", TextStyle::default()),
        ..default()
    });
}

pub fn move_camera(
    mut camera_query: Query<(&mut Velocity, &Transform), With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>
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
        Err(_) => (),
    }
}
