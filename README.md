# bevy_multiplayer_platformer

Random Ideas

- Ability to mute (/automute) other players

To Do General

- Despawn all players on transition to GameState::Transitioning (only local, since each stage is a new p2p room)
- Spawn all players on transition to GameState::Playing

To Do #1

- Add in large sprite that is the colour of the 2 index (floor colour) in the palette
  The size of this sprite should be the level + camera max width/height
  It should be behind everything else
- Add in tiles that surround the level, with the floor colour, so that the player can't get out of the map
- Add in multiple colour palettes and pick a random offset index
- Refactor the stage loading/building so it doesn't make me want to cry

To Do #2

- Add in spikes (obstacles in general)
- Add in player deaths/spawns
- Refactor player files
- Make the player explode into many tiny squares with collision (good ol' ECS)

To Theory Craft

- Player mechanics
- Obstacles
- Moustache/player appearance
