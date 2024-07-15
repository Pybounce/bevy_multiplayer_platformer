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

Stage Mechanics

- Triggers
  - Many types of stage mechanic will be a trigger, such as:
    - Keys
    - Buttons
    - Switches
  - There should probably be some unified way of triggering an event off to the subscribers
  - Subscribers might be
    - Key Blocks
    - Stage Event (rising water etc)
    - Interval blocks
    - Door opening (basically key blocks but it closes after x seconds?)
- Key Blocks
  - Keys are placed around the stage
  - Collect the key to unlock the assigned locked blocks
  - Different keys will be assigned to different blocks in one stage
- Moving platforms
  - Horizontal or vertical
  - Moving platforms should also be able to contain spikes etc
    - The moving 'block' could just be a spike
    - Last project handled this by moving them all separately but in the same way
- Crumbling blocks
  - On touch (by anything?), they dissapear
- Ghost blocks
  - They look solid until the player is close
  - Once one in the group is revealed, all of the group is revealed
- Bouncing blocks
  - Touching them gives the object a boost of velocity in the normal direction
- Rotating stage??
  - Just hold the player in place, rotate the camera, flip gravity
- Interval blocks
  - They 'switch' on and off every x seconds
  - When on, they are just blocks, when off, they have dissapear
  - Can be triggered by timer or maybe even player events (button/switch?)
- Crushing blocks
  - A trigger subscriber that usually just stays on a timer
  - Can act as a door if it subscribes to a non timer trigger?
  - Will crush the player if they hit the bottom or top of it as well as touching ground/ceiling

Player Mechanics

- Dash
  - Works on ground and air
  - Restores after touching ground (+delay) or wall
- Wall jumping
  - Literally just the bees knees, what more must be said
- Slide
  - Press down to trigger
  - Only works when grounded
  - Gives a bit of a boost in speed like dash? (might be weird in air to slide transition)
  - Player becomes shorter and can slide under things

Bugs

- Cannot preload (with a stage load event) on build complete, will try building that stage immediately and fail
  - Need to test the build failed events at different points (1 stage in, 0 stages in etc)
  - Seems like it's not scrubbing the current stage
  - Also might not be taking the user back to stage select
  - NOTE this happens when preloading the next, non-existent stage, in the BuildComplete event
