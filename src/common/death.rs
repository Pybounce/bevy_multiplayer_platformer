use bevy::prelude::*;
use bevy_rapier2d::prelude::CollidingEntities;

#[derive(Component)]
pub struct DeathMarker;

#[derive(Component)]
pub struct Killable;

pub fn despawn_death_marked(
    mut commands: Commands,
    query: Query<Entity, With<DeathMarker>>
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}


#[derive(Component)]
pub struct DeathMarkOnTouch;

pub fn check_touched_by_death(
    query: Query<&CollidingEntities>,
    death_marked_on_touch_query: Query<Entity, With<DeathMarkOnTouch>>,
    mut commands: Commands
) {
    for colliding_entities in &query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(entity) = death_marked_on_touch_query.get(colliding_entity) {
                commands.entity(entity).insert(DeathMarker);
            }
        }
    }
}