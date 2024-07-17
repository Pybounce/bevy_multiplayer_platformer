use bevy::prelude::*;
use bevy_rapier2d::prelude::*;



#[derive(Component)]
pub struct Wall;
#[derive(Component)]
pub struct Wallable;

/// varients describe the side of the wall, not which side the wall is on
#[derive(Component)]
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
        commands.entity(entity).remove::<TouchingWall>();
        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = wall_query.get(colliding_entity) {
                if let Some(contact_pair) = rapier_context.contact_pair(entity, colliding_entity) {
                    let mut normal_sign = 1.0;
                    if contact_pair.collider1() == entity {
                        normal_sign = -1.0;
                    }

                    for manifold in contact_pair.manifolds() {
                        if manifold.normal().x * normal_sign > 0.5 {
                            commands.entity(entity).try_insert(TouchingWall::Right);
                            break;
                        }
                        if manifold.normal().x * normal_sign < -0.5 {
                            commands.entity(entity).try_insert(TouchingWall::Left);
                            break;
                        }
                    }
                }

            }
        }
    }
}

pub fn wallupdate(
    mut query: Query<(&mut Sprite, &TouchingWall)>
) {
    for (mut s, w) in &mut query {
        s.color = match w {
            TouchingWall::Left => Color::GOLD,
            TouchingWall::Right => Color::SEA_GREEN,
        };
    }
}
