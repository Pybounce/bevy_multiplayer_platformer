
use bevy::prelude::*;
use bevy_matchbox::matchbox_socket::PeerId;

use crate::networking::networking_stuff::{PeerConnectionEvent, PeerDisconnectionEvent};

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 2.0, 2.0);


#[derive(Component)]
pub struct NetworkedPlayer {
    peer_id: PeerId
}

pub fn spawn_new_players(
    mut commands: Commands,
    mut player_connection_event_reader: EventReader<PeerConnectionEvent>
) {
    for connection_event in player_connection_event_reader.read() {
        error!("connection event read");
        commands
            .spawn(NetworkedPlayer { peer_id: connection_event.peer_id })
            .insert(SpriteBundle {
                transform: Transform {
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            });
    }
}

pub fn remove_disconnected_players(
    networked_player_query: Query<(&NetworkedPlayer, Entity)>,
    mut commands: Commands,
    mut player_disconnection_event_reader: EventReader<PeerDisconnectionEvent>
) {
    for disconnection_event in player_disconnection_event_reader.read() {
        error!("disconnection event read");

        let aaa = disconnection_event.peer_id.clone();
        //error!("disconnect event with id {aaa}");
        let disconnected_players: Vec<(&NetworkedPlayer, Entity)> = networked_player_query.iter().filter(|x| x.0.peer_id == disconnection_event.peer_id).collect();
        for dp in disconnected_players {
            commands.entity(dp.1).despawn();
        }
    }
}