
use bevy::prelude::*;
use bevy_matchbox::matchbox_socket::PeerId;
use bevy_rapier2d::prelude::{GravityScale, RigidBody, Velocity};

use crate::{common::states::{DespawnOnStateExit, NetworkingState}, local_player::PLAYER_SIZE, networking::networking_stuff::{PeerConnectionEvent, PeerDisconnectionEvent}};

#[derive(Component)]
pub struct NetworkedPlayer {
    pub peer_id: PeerId
}

pub fn spawn_new_players(
    mut commands: Commands,
    mut player_connection_event_reader: EventReader<PeerConnectionEvent>
) {
    for connection_event in player_connection_event_reader.read() {
        commands
            .spawn(NetworkedPlayer { peer_id: connection_event.peer_id })
            .insert(SpriteBundle {
                transform: Transform {
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                ..default()
            })
            .insert(DespawnOnStateExit::Networking(NetworkingState::Connected))
            .insert(Velocity::default())
            .insert(RigidBody::KinematicVelocityBased)
            .insert(GravityScale(0.0));
    }
}

pub fn remove_disconnected_players(
    networked_player_query: Query<(&NetworkedPlayer, Entity)>,
    mut commands: Commands,
    mut player_disconnection_event_reader: EventReader<PeerDisconnectionEvent>
) {
    for disconnection_event in player_disconnection_event_reader.read() {
        let disconnected_players: Vec<(&NetworkedPlayer, Entity)> = networked_player_query.iter().filter(|x| x.0.peer_id == disconnection_event.peer_id).collect();
        for dp in disconnected_players {
            commands.entity(dp.1).despawn();
        }
    }
}

pub fn remove_all_networked_players(
    networked_player_query: Query<Entity, With<NetworkedPlayer>>,
    mut commands: Commands,
) {
    for e in &networked_player_query {
        commands.entity(e).despawn();
    }
}