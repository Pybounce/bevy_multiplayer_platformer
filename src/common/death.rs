use bevy::prelude::*;
use bevy_rapier2d::prelude::CollidingEntities;

#[derive(Component)]
pub struct DeathMarker {
    pub countdown: Timer
}

impl DeathMarker {
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            countdown: Timer::from_seconds(seconds, TimerMode::Once)
        }
    }
    pub fn instant() -> Self {
        DeathMarker::from_seconds(0.0)
    }
}

#[derive(Component)]
pub struct Killable;

pub fn despawn_death_marked(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathMarker)>,
    time: Res<Time>
) {
    for (e, mut death_marker) in &mut query {
        death_marker.countdown.tick(time.delta());
        if death_marker.countdown.finished() {
            commands.entity(e).despawn();
        }
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
                commands.entity(entity).try_insert(DeathMarker::instant());
            }
        }
    }
}