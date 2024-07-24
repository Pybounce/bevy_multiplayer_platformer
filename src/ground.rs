use bevy::prelude::*;
use bevy_rapier2d::prelude::*;



#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Groundable;

#[derive(Component)]
pub struct Grounded;

pub fn check_grounded_old(
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




pub fn check_grounded(
    mut commands: Commands,
    mut wallable_query: Query<(Entity, &mut Transform, &mut Velocity), With<Groundable>>,
    rapier_context: Res<RapierContext>
) {
    for (entity, mut transform, mut velocity) in &mut wallable_query {
        let mut ground_collision = false;


        let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(entity)
        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));

        let raycast_buffer = 5.0;
        let raycast_length = transform.scale.y / 2.0;
        let solid = false;
        let ray_count = 3;

        let mut ray_pos = Vec2::new(transform.translation.x - (transform.scale.x / 2.0), transform.translation.y);
        for _ in 0..ray_count {
            ray_pos.x += transform.scale.x / (ray_count + 1) as f32;
            if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(0.0, 1.0), raycast_length + raycast_buffer , solid, filter) {
                if toi <= raycast_length {
                    velocity.linvel.y = 0.0;
                    transform.translation.y -= raycast_length - toi;
                }
            }
            if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(0.0, -1.0), raycast_length + raycast_buffer, solid, filter) {
                ground_collision = true;

                if toi <= raycast_length {
                    velocity.linvel.y = velocity.linvel.y.max(0.0);
                    transform.translation.y += raycast_length - toi;
                }
            }
        }

        if ground_collision {
            commands.entity(entity).try_insert(Grounded);
        }
        else if !ground_collision {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}





