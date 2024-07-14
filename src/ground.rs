use bevy::prelude::*;
use bevy_rapier2d::prelude::*;



#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Groundable;

#[derive(Component)]
pub struct Grounded;

pub fn check_grounded(
    mut commands: Commands,
    groundable_query: Query<(Entity, &CollidingEntities), With<Groundable>>,
    ground_query: Query<(), With<Ground>>,
    rapier_context: Res<RapierContext>
) {
    for (entity, colliding_entities) in &groundable_query {
        commands.entity(entity).remove::<Grounded>();
        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = ground_query.get(colliding_entity) {
                if let Some(contact_pair) = rapier_context.contact_pair(entity, colliding_entity) {
                    let mut normal_sign = 1.0;
                    if contact_pair.collider1() == entity {
                        normal_sign = -1.0;
                    }

                    for manifold in contact_pair.manifolds() {
                        if manifold.normal().y * normal_sign > 0.5 {
                            commands.entity(entity).try_insert(Grounded);
                            break;
                        }
                    }
                }

            }
        }
    }
}