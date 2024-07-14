# bevy_multiplayer_platformer

Random Ideas

- Ability to mute (/automute) other players

To Do #1

- Make the player explode into many tiny squares with collision (good ol' ECS)
- Move all that player config into a component for the player?

To Theory Craft

- Player mechanics
- Obstacles
- Moustache/player appearance (floating hats, such as a crown, and googly eyes)
- Fancy backgrounds (ones that are parallaxed or move in a nice animation by themselves)
- Fancy stage loading (fancy animation for a stage being loaded/unloaded)
- Stage mechanic where blocks swich on and off in intervals, so player must jump on them with correct timing

Bugs

- Cannot preload (with a stage load event) on build complete, will try building that stage immediately and fail
  - Need to test the build failed events at different points (1 stage in, 0 stages in etc)
  - Seems like it's not scrubbing the current stage
  - Also might not be taking the user back to stage select
  - NOTE this happens when preloading the next, non-existent stage, in the BuildComplete event
