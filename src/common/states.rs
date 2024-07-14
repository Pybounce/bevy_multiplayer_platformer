use bevy::prelude::*;


pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .init_state::<GameState>()
        .init_state::<NetworkingState>()
        .init_state::<StageState>()
        .add_systems(Update, check_state_transitions);
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
    NA,
    Playing,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum StageState {
    #[default]
    NA,
    Loading,
    Loaded
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum NetworkingState {
    Connected,
    #[default]
    Disconnected,
    PreparingDisconnect,
}


//entities with state lifetime x, will be removed when state x is exited
#[derive(Component)]
pub enum DespawnOnStateExit {
    App(AppState),
    Game(GameState),
    Networking(NetworkingState),
    Stage(StageState)
}

#[derive(Component)]
pub enum DespawnOnStateEnter {
    App(AppState),
    Game(GameState),
    Networking(NetworkingState),
    Stage(StageState)
}
fn check_state_transitions(
    mut game_state_transition_events: EventReader<StateTransitionEvent<GameState>>,
    mut stage_state_events: EventReader<StateTransitionEvent<StageState>>,
    mut app_state_transition_events: EventReader<StateTransitionEvent<AppState>>,
    mut networking_state_transition_events: EventReader<StateTransitionEvent<NetworkingState>>,
    mut commands: Commands,
    query: Query<(Entity, &DespawnOnStateExit), With<DespawnOnStateExit>>,
    enter_state_query: Query<(Entity, &DespawnOnStateEnter), With<DespawnOnStateEnter>>
) {
    let game_state_events: Vec<_> = game_state_transition_events.read().collect();
    let stage_state_events: Vec<_> = stage_state_events.read().collect();
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
            DespawnOnStateExit::Stage(x) => {
                for ste in &stage_state_events {
                    if x == &ste.before {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }


    //yeah this is bad, shut up

    for (entity, state_lifetime) in enter_state_query.iter() {
        match state_lifetime {
            DespawnOnStateEnter::App(x) => {
                for ste in &app_state_events {
                    if x == &ste.after {
                        commands.entity(entity).despawn();
                    }
                }
            }
            DespawnOnStateEnter::Game(x) => {
                for ste in &game_state_events {
                    if x == &ste.after {
                        commands.entity(entity).despawn();
                    }
                }
            }
            DespawnOnStateEnter::Networking(x) => {
                for ste in &networking_state_events {
                    if x == &ste.after {
                        commands.entity(entity).despawn();
                    }
                }
            }
            DespawnOnStateEnter::Stage(x) => {
                for ste in &stage_state_events {
                    if x == &ste.after {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }





}