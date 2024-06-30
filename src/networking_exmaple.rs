
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
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

pub fn receive_messages(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}