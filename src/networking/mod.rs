use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod networking_stuff;
use networking_stuff::{ check_peer_connections, receive_messages, send_message, start_socket, PeerConnectionEvent, PeerDisconnectionEvent };

pub mod messages;

pub struct GameNetworkingPlugin;
impl Plugin for GameNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PeerConnectionEvent>()
        .add_event::<PeerDisconnectionEvent>()
        .add_systems(Startup, start_socket)
        .add_systems(Update, (receive_messages, check_peer_connections))
        .add_systems(
            Update,
            send_message.run_if(on_timer(Duration::from_millis(300))),
        );
    }
}