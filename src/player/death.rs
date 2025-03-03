use bevy::prelude::*;
use bevy_rapier2d::prelude::{Ccd, Collider, GravityScale, RigidBody, Velocity};
use rand::Rng;

use crate::{common::{death::DeathMarker, physics::gravity::Gravity}, local_player::LocalPlayer, stage::stage_builder::StageAssets};

use super::spawner::LocalPlayerSpawner;


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn trigger_dead_local_player_respawn(
    mut commands: Commands,
    query: Query<&Respawnable, (With<LocalPlayer>, With<DeathMarker>)>,
    time: Res<Time>
) {
    for respawnable in &query {
        commands.spawn(LocalPlayerSpawner {
            spawn_time: time.elapsed_seconds_f64() + respawnable.delay_in_seconds,
            translation: respawnable.translation,
        });
    }
}

pub fn player_death_particles(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, With<DeathMarker>)>,
    time: Res<Time>
) {
    for player_transform in &query {

        let particle_count = 50;
        
        for i in 0..particle_count {
            let x = rand::thread_rng().gen_range(-8.0..8.0);
            let y = rand::thread_rng().gen_range(-8.0..8.0);
            let x_force = rand::thread_rng().gen_range(-50.0..50.0);
            let y_force = rand::thread_rng().gen_range(30.0..120.0);
            commands.spawn((
                SpriteBundle {
                sprite: Sprite { color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 1.0)),
                    custom_size: Vec2::new(1.0, 1.0).into(), 
                    ..default() },
                transform: Transform::from_translation(player_transform.translation + Vec3::new(x, y, 0.0)),
                ..default()
                },
                Velocity::linear(Vec2::new(x_force, y_force)),
                Gravity {
                    max_force: 500.0,
                    current_force: 0.0,
                    acceleration: 1000.0,
                },
                RigidBody::Dynamic,
                Collider::ball(0.5),
                GravityScale(0.0),
                Ccd::enabled(),
                DeathMarker::from_seconds(1.5),
                ColorTween {
                    start_time: time.elapsed_seconds(),
                    duration: 1.5, 
                    start_color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 1.0)),
                    target_color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 0.0)),                    
                }
            ));
        }


    }
}


#[derive(Component)]
pub struct ColorTween {
    pub start_time: f32,
    pub duration: f32,
    pub start_color: Color,
    pub target_color: Color
}

pub fn tween_colours(
    mut query: Query<(&mut Sprite, &ColorTween, Entity)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mut sprite, tween_data, e) in &mut query {
        let x = (time.elapsed_seconds() - tween_data.start_time) / tween_data.duration;
        let lerp_t = quadratic_lerp(x);
        if lerp_t <= 0.0 { continue; }    //start time not reached yet

        let color_rgba = ((tween_data.target_color.to_linear().to_vec4() - tween_data.start_color.to_linear().to_vec4()) * lerp_t) + tween_data.start_color.to_linear().to_vec4();
        sprite.color = Color::rgba(color_rgba.x, color_rgba.y, color_rgba.z, color_rgba.w);

        if lerp_t >= 1.0 { 
            sprite.color = tween_data.target_color;   //this doesn't work with multiple!!!
            commands.entity(e).remove::<ColorTween>(); 
        }
    }
}
fn quadratic_lerp(t: f32) -> f32 {
    t * t
}