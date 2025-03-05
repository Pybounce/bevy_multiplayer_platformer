
use bevy::{core_pipeline::bloom::BloomSettings, input::mouse::MouseWheel, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;

const CAMERA_ZOOM: u32 = 3;
const CAMERA_ZOOM_MAX: u32 = 10;
const CAMERA_ZOOM_MIN: u32 = 1;

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
                //clear_color: ClearColorConfig::Custom(Color::linear_rgb(222.0, 1.0, 201.0)),
               // hdr: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings {
            intensity: 0.0,
            ..default()
        })
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(PixelPerfectTranslation {
            translation: Vec3::default(),
            factor: CAMERA_ZOOM as u32
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



pub fn handle_zoom_change(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut PixelPerfectTranslation, &mut OrthographicProjection), With<Camera>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut pixel_translation, mut projection) in &mut camera_query {

            let new_zoom = match mouse_wheel_event.y > 0.0 {
                true => pixel_translation.factor + 1,
                false => pixel_translation.factor - 1,
            }.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);

            projection.scale = 1.0 / (new_zoom as f32);
            pixel_translation.factor = new_zoom;
        }
    }
}