use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn spawn_stage(
    mut commands: Commands
) {
    commands
        .spawn(Ground)
        .insert(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(100.0, 200.0, 1.0),
                translation: Vec3::new(0.0, -200.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(GravityScale(0.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

        commands
        .spawn(Ground)
        .insert(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(100.0, 200.0, 1.0),
                translation: Vec3::new(100.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(GravityScale(0.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

        commands
        .spawn(Ground)
        .insert(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 1.0),
                translation: Vec3::new(0.0, 40.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(GravityScale(0.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
}


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
                        error!("normal: {0}", manifold.normal().to_string());

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