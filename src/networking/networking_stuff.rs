
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{common::states::NetworkingState, local_player::LocalPlayer, player::look_state::PlayerLookState, stage::stage_builder::CurrentStageData};

use super::{messages::NewLocationMessage, networked_players::NetworkedPlayer, CurrrentNetworkData};

pub fn connect_socket(
    mut commands: Commands,
    stage_data_opt: Option<Res<CurrentStageData>>,
    mut networking_state: ResMut<NextState<NetworkingState>>,
) {
    if let None = stage_data_opt { return; }
    let stage_data = stage_data_opt.unwrap();

    let mut room_url = String::from("ws://20.90.116.144:3536/game_name_");
    //let mut room_url = String::from("wss://4.158.59.49:443/game_name_");
    //let mut room_url = String::from("ws://4.158.59.49:80/game_name_");
    //let mut room_url = String::from("wss://platformer.skybounce.io:443/game_name_");
    //let mut room_url = String::from("ws://platformer.skybounce.io:80/game_name_");
    ////let mut room_url = String::from("ws://localhost:3536/");

    room_url.push_str(&stage_data.stage_id.to_string());
    let socket = MatchboxSocket::new_reliable(room_url);

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
        if !networking_data.socket.is_closed() {
            networking_data.socket.close();
        }
        commands.remove_resource::<CurrrentNetworkData>();
        networking_state.set(NetworkingState::Disconnected);
    }

}

pub fn send_message(
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    local_player_query: Query<(&Transform, &Velocity, &PlayerLookState), (With<LocalPlayer>, Without<NetworkedPlayer>)>
) {
    if CurrrentNetworkData::is_valid(&mut networking_data_opt) { 
        let mut networking_data = networking_data_opt.unwrap();
        let peers: Vec<_> = networking_data.socket.connected_peers().collect();

        for peer in peers {
            let query_result = local_player_query.get_single();
            if let Ok((t, v, ls)) = query_result {
                let message = NewLocationMessage {
                    code: 0,
                    translation_x: t.translation.x,
                    translation_y: t.translation.y,
                    velocity_x: v.linvel.x,
                    velocity_y: v.linvel.y,
                    look_state: *ls
                };
                networking_data.socket.send(bincode::serialize(&message).unwrap().into(), peer);
            }
        }
    }
}

pub fn receive_messages(
    mut networking_data_opt: Option<ResMut<CurrrentNetworkData>>,
    mut player_query: Query<(Entity, &mut Transform, &mut Velocity, &NetworkedPlayer)>,
    mut commands: Commands
) {
    if CurrrentNetworkData::is_valid(&mut networking_data_opt) { 
        let mut networking_data = networking_data_opt.unwrap();
        for (id, message) in networking_data.socket.receive() {
            let message: NewLocationMessage = bincode::deserialize(&message[..]).unwrap();
            for (e, mut t, mut v, np) in &mut player_query {
                if np.peer_id == id {
                    t.translation = Vec3::new(message.translation_x, message.translation_y, 0.0);
                    v.linvel = Vec2::new(message.velocity_x, message.velocity_y);
                    commands.entity(e).try_insert(message.look_state);
                }
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
    if CurrrentNetworkData::is_valid(&mut networking_data_opt) { 
        let mut networking_data = networking_data_opt.unwrap();

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

