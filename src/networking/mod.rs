use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod networking_stuff;
use networking_stuff::{ check_peer_connections, connect_socket, disconnect_socket, prepare_disconnect, receive_messages, send_message, PeerConnectionEvent, PeerDisconnectionEvent };

use crate::common::states::{NetworkingState, StageState};

pub mod messages;
pub mod networked_players;

pub struct GameNetworkingPlugin;
impl Plugin for GameNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PeerConnectionEvent>()
        .add_event::<PeerDisconnectionEvent>()
        .add_systems(OnEnter(StageState::Loaded), connect_socket)
        .add_systems(OnExit(StageState::Loaded), prepare_disconnect)
        .add_systems(OnEnter(NetworkingState::PreparingDisconnect), disconnect_socket)
        .add_systems(Update, (receive_messages, check_peer_connections)
            .run_if(in_state(NetworkingState::Connected)))
        .add_systems(
            Update,
            send_message.run_if(on_timer(Duration::from_millis(100 / 6))).run_if(in_state(NetworkingState::Connected)),
        );
    }
}