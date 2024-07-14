use bevy::prelude::*;

use crate::{common::death_marker::DeathMarker, local_player::LocalPlayer};

use super::spawner::LocalPlayerSpawner;


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn trigger_dead_local_player_respawn(
    mut commands: Commands,
    query: Query<&Respawnable, (With<LocalPlayer>, With<DeathMarker>)>,
    time: Res<Time>
) {
    for respawnable in &query {
        commands.spawn(LocalPlayerSpawner {
            spawn_time: time.elapsed_seconds_f64() + 3.0,
            translation: respawnable.translation,
        });
    }
}