
use bevy::prelude::*;
use bevy_matchbox::prelude::*;

pub fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/hello");
    commands.insert_resource(socket);
}

pub fn send_message(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "Hello";
        //info!("Sending message: {message:?} to {peer}");
        //socket.send(message.as_bytes().into(), peer);
    }
}

pub fn receive_messages(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
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

