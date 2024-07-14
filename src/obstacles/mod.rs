use bevy::prelude::*;
use bevy_rapier2d::geometry::CollidingEntities;

use crate::{common::death::{DeathMarker, Killable}, local_player::LocalPlayer};

#[derive(Component)]
pub struct InstantKiller;


pub fn check_insta_kill_collisions(
    player_query: Query<(Entity, &CollidingEntities), With<Killable>>,
    killer_query: Query<(), With<InstantKiller>>,
    mut commands: Commands
) {
    for (e, colliding_entities) in &player_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = killer_query.get(colliding_entity) {
                commands.entity(e).try_insert(DeathMarker);
            }
        }
    }
}
