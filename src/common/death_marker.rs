use bevy::prelude::*;

#[derive(Component)]
pub struct DeathMarker;

pub fn despawn_death_marked(
    mut commands: Commands,
    query: Query<Entity, With<DeathMarker>>
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}