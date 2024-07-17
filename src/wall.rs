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
    wallable_query: Query<(Entity, &CollidingEntities), With<Wallable>>,
    wall_query: Query<(), With<Wall>>,
    rapier_context: Res<RapierContext>
) {
    for (entity, colliding_entities) in &wallable_query {
        let mut colliding_left = false;
        let mut colliding_right = false;

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = wall_query.get(colliding_entity) {
                if let Some(contact_pair) = rapier_context.contact_pair(entity, colliding_entity) {
                    let mut normal_sign = 1.0;
                    if contact_pair.collider1() == entity {
                        normal_sign = -1.0;
                    }

                    for manifold in contact_pair.manifolds() {
                        colliding_right = manifold.normal().x * normal_sign > 0.5;
                        colliding_left = manifold.normal().x * normal_sign < -0.5;
                    };

                }
            }

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

//TODO - SPlit out the WallStuck into a separate function that looks for the current TouchingWall and compares it with WallStuck, then adds or removes

