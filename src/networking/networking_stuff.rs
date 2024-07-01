
use bevy::prelude::*;
use bevy_matchbox::prelude::*;

use crate::{local_player::LocalPlayer, networked_players::NetworkedPlayer};

use super::messages::NewLocationMessage;

pub fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/hello");
    commands.insert_resource(socket);
}

pub fn send_message(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    local_player_query: Query<&Transform, (With<LocalPlayer>, Without<NetworkedPlayer>)>
) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let t_result = local_player_query.get_single();
        match t_result {
            Ok(t) => {
                let message = NewLocationMessage {
                    code: 0,
                    translation_x: t.translation.x,
                    translation_y: t.translation.y,
                };
                socket.send(bincode::serialize(&message).unwrap().into(), peer);
            },
            Err(e) => error!("ERROR SENDING MESSAGE: {e}"),
        }
    }
}

pub fn receive_messages(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut player_query: Query<&mut Transform, (Without<LocalPlayer>, With<NetworkedPlayer>)>
) {
    for (_id, message) in socket.receive() {
        let message: NewLocationMessage = bincode::deserialize(&message[..]).unwrap();
        for mut t in &mut player_query {
            t.translation = Vec3::new(message.translation_x, message.translation_y, 0.0);
        }

    }
}


#[derive(Event)]
pub struct PeerConnectionEvent {
    pub peer_id: PeerId
}
#[derive(Event)]
pub struct PeerDisconnectionEvent {
    pub peer_id: PeerId
}

pub fn check_peer_connections(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut connection_event_writer: EventWriter<PeerConnectionEvent>,
    mut disconnection_event_writer: EventWriter<PeerDisconnectionEvent>
) {
    for (id, state) in socket.update_peers().into_iter() {
        match state {
            PeerState::Connected => {
                connection_event_writer.send(PeerConnectionEvent { peer_id: id });
            },
            PeerState::Disconnected => {
                disconnection_event_writer.send(PeerDisconnectionEvent { peer_id: id });
            },
        }
    }
}

