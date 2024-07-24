use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground::{Groundable, Grounded}, player::wall_jump_controller::WallStickable};



#[derive(Component)]
pub struct Wall;
#[derive(Component)]
pub struct Wallable;

/// varients describe the side of the wall, not which side the wall is on
#[derive(Component, Copy, Clone)]
pub enum TouchingWall {
    Left,
    Right
}


pub fn check_touching_wall(
    mut commands: Commands,
    wallable_query: Query<(Entity, &Transform, Option<&TouchingWall>), With<Wallable>>,
    _wall_query: Query<(), With<Wall>>,
    rapier_context: Res<RapierContext>
) {
    for (entity, transform, tw_opt) in &wallable_query {
        let mut new_left_collision = false;
        let mut new_right_collision = false;


        let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(entity)
        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));

        let max_toi = (transform.scale.x / 2.0) - 5.0;
        let solid = true;
        let ray_count = 3;

        let mut ray_pos = transform.translation.truncate() - (transform.scale.y / 2.0);
        for _ in 0..ray_count {
            ray_pos += transform.scale.y / (ray_count + 1) as f32;
            if let Some((_entity, _toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(-1.0, 0.0), max_toi, solid, filter) {
                new_right_collision = true;
            }
            if let Some((_entity, _toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(1.0, 0.0), max_toi, solid, filter) {
                new_left_collision = true;
            }
        }

        // if it's the new collision is already set, continue.
        if let Some(tw) = tw_opt {
            match tw {
                TouchingWall::Left => {
                    if new_left_collision { continue; }
                },
                TouchingWall::Right => {
                    if new_right_collision { continue; }
                },
            }
        }

        if new_left_collision {
            commands.entity(entity).try_insert(TouchingWall::Left);
        }
        else if new_right_collision {
            commands.entity(entity).try_insert(TouchingWall::Right);
        }
        else if !new_right_collision && !new_left_collision {
            commands.entity(entity).remove::<TouchingWall>();
        }
    }
}

pub fn asdfdasd(
    mut query: Query<&mut Sprite, (Without<Grounded>, With<Groundable>)>
) {
    for mut s in &mut query {
        s.color = Color::RED;
    }
}
pub fn asdfdasd2(
    mut query: Query<&mut Sprite, (With<Grounded>, With<Groundable>)>
) {
    for mut s in &mut query {
        s.color = Color::GREEN;
    }
}


