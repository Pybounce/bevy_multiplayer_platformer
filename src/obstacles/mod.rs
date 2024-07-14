use bevy::prelude::*;
use bevy_rapier2d::geometry::CollidingEntities;

use crate::common::{death::{DeathMarker, Killable}, shake::Shake};

#[derive(Component)]
pub struct InstantKiller;


pub fn check_insta_kill_collisions(
    killable_query: Query<(Entity, &CollidingEntities), With<Killable>>,
    killer_query: Query<(), With<InstantKiller>>,
    mut commands: Commands,
    cam_query: Query<Entity, With<Camera>>
) {
    for (e, colliding_entities) in &killable_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = killer_query.get(colliding_entity) {
                commands.entity(e).try_insert(DeathMarker);
                let cam = cam_query.single();
                commands.entity(cam).try_insert(Shake {
                    current_offset: Vec2::ZERO,
                    force: 10.0,
                    duration: Some(0.2),
                    shake_delay: 0.01,
                    current_delay: 0.0,
                });
            }
        }
    }
}


