
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;

const CAMERA_ZOOM: u16 = 3;

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
                //clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.5, 0.5, 0.5)),
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
        .insert(PixelPerfectTranslation {
            translation: Vec3::default(),
            factor: CAMERA_ZOOM as u32
        });

    commands.spawn(Text2dBundle {
        text: Text::from_section("DEADLOCKED 2: RELOADED", TextStyle::default()),
        ..default()
    });
}

pub fn move_camera(
    mut camera_query: Query<&mut PixelPerfectTranslation, With<Camera>>,
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

pub fn move_pixel_perfect_translations(
    mut query : Query<(&mut Transform, &PixelPerfectTranslation)>,
) {
    for (mut t, pp) in &mut query {
        t.translation = Vec3::new(
            round_by_factor(pp.translation.x, pp.factor), 
            round_by_factor(pp.translation.y, pp.factor), 
            round_by_factor(pp.translation.z, pp.factor)); 
    }
}

fn round_by_factor(val: f32, factor: u32) -> f32 {
    (val * factor as f32).trunc() / factor as f32
}

#[derive(Component)]
pub struct PixelPerfectTranslation {
    pub translation: Vec3,
    pub factor: u32
}
