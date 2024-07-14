
use bevy::prelude::*;
use bevy_matchbox::prelude::*;

use crate::{common::states::NetworkingState, local_player::LocalPlayer, stage::stage_builder::CurrentStageData};

use super::{messages::NewLocationMessage, networked_players::NetworkedPlayer, CurrrentNetworkData};

pub fn connect_socket(
    mut commands: Commands,
    stage_data: Res<CurrentStageData>,
    mut networking_state: ResMut<NextState<NetworkingState>>,
) {
    let mut room_url = String::from("ws://20.90.116.144:3536/game_name_");
    //let mut room_url = String::from("ws://localhost:3536/");

    room_url.push_str(&stage_data.stage_id.to_string());
    let socket = MatchboxSocket::new_reliable(room_url);

    //commands.insert_resource(socket);
    commands.insert_resource(CurrrentNetworkData {
        socket,
        stage_id: stage_data.stage_id,
    });
    networking_state.set(NetworkingState::Connected);
}

pub fn disconnect_socket(
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    mut networking_state: ResMut<NextState<NetworkingState>>,
    mut commands: Commands
) {
    if let Some(networking_data) = &mut networking_data_opt { 
        networking_data.socket.close();
        commands.remove_resource::<CurrrentNetworkData>();
        networking_state.set(NetworkingState::Disconnected);
    }

}

pub fn send_message(
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    local_player_query: Query<&Transform, (With<LocalPlayer>, Without<NetworkedPlayer>)>
) {
    if let Some(networking_data) = &mut networking_data_opt { 
        let peers: Vec<_> = networking_data.socket.connected_peers().collect();

        for peer in peers {
            let t_result = local_player_query.get_single();
            match t_result {
                Ok(t) => {
                    let message = NewLocationMessage {
                        code: 0,
                        translation_x: t.translation.x,
                        translation_y: t.translation.y,
                    };
                    networking_data.socket.send(bincode::serialize(&message).unwrap().into(), peer);
                },
                Err(e) => error!("ERROR SENDING MESSAGE: {e}"),
            }
        }
    }
}

pub fn receive_messages(
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    mut player_query: Query<&mut Transform, (Without<LocalPlayer>, With<NetworkedPlayer>)>
) {
    if let Some(networking_data) = &mut networking_data_opt { 
        for (_id, message) in networking_data.socket.receive() {
            let message: NewLocationMessage = bincode::deserialize(&message[..]).unwrap();
            for mut t in &mut player_query {
                t.translation = Vec3::new(message.translation_x, message.translation_y, 0.0);
            }
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
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    mut connection_event_writer: EventWriter<PeerConnectionEvent>,
    mut disconnection_event_writer: EventWriter<PeerDisconnectionEvent>
) {
    if let Some(networking_data) = &mut networking_data_opt { 
        for (id, state) in networking_data.socket.update_peers().into_iter() {
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

}

