use bevy::{prelude::*, transform::commands};

use crate::{local_player::LocalPlayerBundle, stage::stage_builder::CurrentStageData};


#[derive(Component)]
pub struct LocalPlayerSpawner {
    pub spawn_time: f64,
    pub translation: Vec3
}

pub fn spawn_local_players(
    query: Query<(Entity, &LocalPlayerSpawner)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, spawner) in &query {
        if time.elapsed_seconds_f64() >= spawner.spawn_time {
            commands.spawn(LocalPlayerBundle::from_spawn_pos(spawner.translation));
            commands.entity(entity).despawn();
        }
    }
}