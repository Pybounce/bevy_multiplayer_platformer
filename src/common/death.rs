use bevy::{prelude::*, transform::commands};

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
pub struct TimedDespawn {
    time_remaining: f32
}

pub fn update_timed_despawns(
    mut query: Query<(Entity, &mut TimedDespawn)>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (e, mut td) in &mut query {
        td.time_remaining -= time.delta_seconds();
        if td.time_remaining <= 0.0 {
            commands.entity(e).try_insert(DeathMarker);
        }
    }
}
