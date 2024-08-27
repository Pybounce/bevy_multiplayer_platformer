use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod networking_stuff;
use bevy_matchbox::{matchbox_socket::SingleChannel, MatchboxSocket};
use networked_players::remove_all_networked_players;
use networking_stuff::{ check_peer_connections, connect_socket, disconnect_socket, receive_messages, send_message, PeerConnectionEvent, PeerDisconnectionEvent };

use crate::{common::states::NetworkingState, stage::stage_builder::CurrentStageData};

pub mod messages;
pub mod networked_players;

pub struct GameNetworkingPlugin;
impl Plugin for GameNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(NetworkingPreferences { online: false, _retry_count: 3 })
        .add_event::<PeerConnectionEvent>()
        .add_event::<PeerDisconnectionEvent>()
        .add_systems(Update, check_connection.run_if(in_state(NetworkingState::Connected).or_else(in_state(NetworkingState::Disconnected))))      
        .add_systems(OnEnter(NetworkingState::Disconnecting), disconnect_socket)
        .add_systems(OnEnter(NetworkingState::Disconnected), remove_all_networked_players)
        .add_systems(Update, connect_socket.run_if(in_state(NetworkingState::Connecting)))
        .add_systems(Update, (receive_messages, check_peer_connections)
            .run_if(in_state(NetworkingState::Connected)))
        .add_systems(
            Update,
            send_message.run_if(on_timer(Duration::from_millis(16))).run_if(in_state(NetworkingState::Connected)),
        );
    }
}

#[derive(Resource)]
pub struct CurrrentNetworkData {
    socket: MatchboxSocket<SingleChannel>,
    stage_id: usize
}

impl CurrrentNetworkData {
    pub fn is_valid(optional_data: &mut Option<ResMut<CurrrentNetworkData>>) -> bool {
        //check connection is open and it's SOME
        match optional_data {
            Some(x) => {
                if x.socket.is_closed() {
                    return false;
                }
                else {
                    return true;
                }
            },
            None => false,
        }
    }
}

#[derive(Resource)]
pub struct NetworkingPreferences {
    online: bool,
    _retry_count: u8
}

fn check_connection(
    current_stage_data: Option<Res<CurrentStageData>>,
    current_network_data: Option<Res<CurrrentNetworkData>>,
    networking_preferences: Res<NetworkingPreferences>,
    mut networking_state: ResMut<NextState<NetworkingState>>,
) {
    if networking_preferences.online == false { return; }
    match current_stage_data {
        Some(stage_data) => {
            match current_network_data {
                Some(network_data) => {
                    if network_data.stage_id != stage_data.stage_id || network_data.socket.is_closed() {
                        // disconnect current connection
                        networking_state.set(NetworkingState::Disconnecting);
                    }
                },
                None => {
                    // start new connection
                    networking_state.set(NetworkingState::Connecting);
                },
            }
        },
        None => {
            match current_network_data {
                Some(_) => {
                    //disconnect current connection
                    networking_state.set(NetworkingState::Disconnecting);
                },
                None => (),
            }
        },
    }


}
