use bevy::prelude::*;
use bevy::prelude::States;


pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .init_state::<GameState>()
        .init_state::<NetworkingState>()
        .add_systems(Update, check_exit_states);
    }
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    Game,
    #[default]
    StageSelect,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Playing,
}

#[derive(Resource)]
pub struct StageData {
    pub stage_id: usize
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum NetworkingState {
    Connecting,
    Connected,
    #[default]
    Disconnected,
}


//entities with state lifetime x, will be removed when state x is exited
#[derive(Component)]
pub enum DespawnOnStateExit {
    App(AppState),
    Game(GameState),
    Networking(NetworkingState)
}

fn check_exit_states(
    mut game_state_transition_events: EventReader<StateTransitionEvent<GameState>>,
    mut app_state_transition_events: EventReader<StateTransitionEvent<AppState>>,
    mut networking_state_transition_events: EventReader<StateTransitionEvent<NetworkingState>>,
    mut commands: Commands,
    query: Query<(Entity, &DespawnOnStateExit), With<DespawnOnStateExit>>
) {
    let game_state_events: Vec<_> = game_state_transition_events.read().collect();
    let app_state_events: Vec<_> = app_state_transition_events.read().collect();
    let networking_state_events: Vec<_> = networking_state_transition_events.read().collect();
    if app_state_events.len() == 0 
    && game_state_events.len() == 0
    && networking_state_events.len() == 0 {
        return;
    }

    for (entity, state_lifetime) in query.iter() {
        match state_lifetime {
            DespawnOnStateExit::App(x) => {
                for ste in &app_state_events {
                    if x == &ste.before {
                        commands.entity(entity).despawn();
                    }
                }
            }
            DespawnOnStateExit::Game(x) => {
                for ste in &game_state_events {
                    if x == &ste.before {
                        commands.entity(entity).despawn();
                    }
                }
            }
            DespawnOnStateExit::Networking(x) => {
                for ste in &networking_state_events {
                    if x == &ste.before {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}