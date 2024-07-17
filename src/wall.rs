use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::wall_jump_controller::{WallStickable, WallStuck};



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
    wallable_query: Query<(Entity, &Transform), With<Wallable>>,
    wall_query: Query<(), With<Wall>>,
    rapier_context: Res<RapierContext>
) {
    for (entity, transform) in &wallable_query {
        let mut colliding_left = false;
        let mut colliding_right = false;

        let ray_pos = transform.translation.truncate();
        let ray_dir = Vec2::new(-1.0, 0.0);
        let max_toi = 20.0;
        let solid = true;

        let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(entity);

        if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            colliding_right = true;
        }

        if colliding_left {
            commands.entity(entity).try_insert(TouchingWall::Left);
        }
        else if colliding_right {
            commands.entity(entity).try_insert(TouchingWall::Right);
        }
        else if !colliding_right && !colliding_left {
            commands.entity(entity).remove::<TouchingWall>();
        }
    }
}

pub fn asdfdasd(
    mut query: Query<&mut Sprite, (Without<TouchingWall>, With<WallStickable>)>
) {
    for mut s in &mut query {
        s.color = Color::RED;
    }
}
pub fn asdfdasd2(
    mut query: Query<&mut Sprite, (With<TouchingWall>, With<WallStickable>)>
) {
    for mut s in &mut query {
        s.color = Color::GREEN;
    }
}


