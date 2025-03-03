

use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{common::death::DeathMarker, player::death::ColorTween};

use super::gravity::Gravity;

#[derive(Component)]
pub struct Glass {
    pub shatter_range: Range<f32>
}
impl Glass {
    pub fn new_fast() -> Self {
        Self {
            shatter_range: Range {
                start: 0.2,
                end: 1.0,
            },
        }
    }
    pub fn new_slow() -> Self {
        Self {
            shatter_range: Range {
                start: 1.0,
                end: 2.0,
            },
        }
    }
}

pub fn shatter_glass(
    query: Query<(&Sprite, &Handle<Image>, &Transform, Option<&Velocity>, &Glass), With<DeathMarker>>,
    assets: Res<Assets<Image>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (sprite, img_handle, transform, vel_opt, glass) in &query {
        if sprite.rect.is_none() { continue; }
        if let Some(img) = assets.get(img_handle) {
            let rect = sprite.rect.unwrap();
            for x in (rect.min.x as i32)..(rect.max.x as i32) {
                for y in (rect.min.y as i32)..(rect.max.y as i32) {
                    let data_width = img.texture_descriptor.size.width as i32;
                    let colour = colour_from_point(&img.data, IVec2::new(x, y), data_width);
                    if colour.3 > 0 {
                        let mut x_force = rand::thread_rng().gen_range(-1.0..1.0);
                        let mut y_force = rand::thread_rng().gen_range(-1.0..1.0);
                        if let Some(vel) = vel_opt {
                            x_force += vel.linvel.x;
                            y_force += vel.linvel.y;
                        }
                        let death_delay = rand::thread_rng().gen_range(glass.shatter_range.start..glass.shatter_range.end);
                        commands.spawn((
                            SpriteBundle {
                            sprite: Sprite { color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 1.0)),
                                custom_size: Vec2::new(1.0, 1.0).into(), 
                                ..default() },
                            transform: Transform::from_translation(transform.translation + Vec3::new((x - 8) as f32 - rect.min.x, -((y - 8) as f32 - rect.min.y), 0.0)),
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
                            DeathMarker::from_seconds(death_delay),
                            CollisionGroups::new(Group::GROUP_10, Group::ALL),
                            ColorTween {
                                start_time: time.elapsed_seconds(),
                                duration: death_delay, 
                                start_color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 1.0)),
                                target_color: Color::LinearRgba(LinearRgba::new(2.0, 2.0, 2.0, 0.0)),                    
                            }
                        ));
                    }
                }
            }
        }
    }
}

fn colour_from_point(data: &Vec<u8>, point: IVec2, data_width: i32) -> (u8, u8, u8, u8) {
    let index = ((point.y * data_width + point.x) * 4) as usize;
    return (
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    );
}