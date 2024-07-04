# bevy_multiplayer_platformer

Random Ideas

- Ability to mute (/automute) other players

To Do

- Despawn all players on transition to GameState::Transitioning (only local, since each stage is a new p2p room)
- Spawn all players on transition to GameState::Playing
- Move movement data (controls, speeds etc) out of LocalPlayer and into PlayerPlatformController component
