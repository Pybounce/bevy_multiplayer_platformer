use bevy::{prelude::*, transform::commands};
use bevy_rapier2d::prelude::*;



fn spawn_tile(x: f32, y: f32, commands: &mut Commands) {
    commands
    .spawn(Ground)
    .insert(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(30.0, 30.0, 1.0),
            translation: Vec3::new(x * 30.0, y * 30.0, 0.0),
            //rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 60.0),
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

pub fn spawn_stage_vec(
    mut commands: Commands
) {
    let width = 9;
    let height = 9;
    let stage: Vec::<u32> = vec![
    0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 1, 1, 1, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,];


    for i in 0..stage.len() {
        if stage[i] == 0 { continue; }
        let x = i % width;
        let y = i / height;
        spawn_tile(x as f32, -(y as f32), &mut commands);
    }
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