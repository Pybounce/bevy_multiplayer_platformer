
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;

const CAMERA_ZOOM: u16 = 2;

pub fn spawn_camera(mut commands: Commands) {

    commands
        .spawn(Camera2dBundle {
            projection : OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 1.0 / (CAMERA_ZOOM as f32),
                ..default()
            },
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::linear_rgb(1.0, 0.5, 0.5)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        //.insert(BloomSettings::default())
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(PixelPerfection {
            translation: Vec3::default()
        });

    commands.spawn(Text2dBundle {
        text: Text::from_section("value", TextStyle::default()),
        ..default()
    });
}

pub fn move_camera(
    mut camera_query: Query<&mut PixelPerfection, With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    time: Res<Time>
) {
    let mut ct = camera_query.single_mut();
    let pt = player_query.get_single();
    match pt {
        Ok(pt) => {
            let distance = ct.translation.truncate().distance(pt.translation.truncate());
            let speed = distance * 2.5;
            let dir = (pt.translation - ct.translation).truncate().normalize_or_zero();

            let delta = time.delta_seconds() * speed * dir;
            ct.translation += delta.extend(0.0);
        }
        Err(_) => (),
    }
}

pub fn move_pixel_perfect_bois(
    mut query : Query<(&mut Transform, &PixelPerfection)>,
) {
    for (mut t, pp) in &mut query {
        t.translation = Vec3::new(
            round_by_zoom(pp.translation.x, CAMERA_ZOOM), 
            round_by_zoom(pp.translation.y, CAMERA_ZOOM), 
            round_by_zoom(pp.translation.z, CAMERA_ZOOM)); 
    }
}

fn round_by_zoom(val: f32, zoom: u16) -> f32 {
    (val * zoom as f32).trunc() / zoom as f32
}

#[derive(Component)]
pub struct PixelPerfection {
    pub translation: Vec3
}
