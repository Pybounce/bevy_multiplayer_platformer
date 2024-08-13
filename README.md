# Bevy Multiplayer Platformer

## Alpha 1.0 Checklist

- [ ] Many full levels with checkpoints and difficulty
- [ ] Spectate mode
- [ ] Main menu UI
- [ ] New stage mechanics
- [ ] Networked player names
- [ ] Player death juice
- [ ] Player death counter?
- [x] Web build
- [x] Basic art
- [ ] Web build networking

## To Do

- Make the player explode into many tiny squares with collision (good ol' ECS)
- Move all that player config into a component for the player?
- Dying doesn't reset the level
  - Triggers like lock blocks or keys are not reset when the player dies

## Animations in Networking

- Cannot use inputs to deal with animations since you do not have the input of other players
  - Will therefore need to use acceleration for eye animations
- For collision particles (such as hitting the ground or jumping), will need to use IsGrounded states etc
  - This also means maintaining those states for networked players
  - Also making sure those states don't flicker and perhaps raise events for transitions? (ie leaving Grounded could mean you jumped or could mean you walked off an edge etc)

## To Theory Craft

- Moustache/player appearance (floating hats, such as a crown, and googly eyes)
- Fancy stage loading (fancy animation for a stage being loaded/unloaded)
- Stage mechanic where blocks swich on and off in intervals, so player must jump on them with correct timing
- Saws currently look like little donuts

## Critical Bugs

- Player spawns a little above the ground and doesn't fall all the way down
  - This links to the player being able to jump before hitting the ground
    - Likely just need to lower the raycast padding
- Touching a spring doesn't reset players jump
  - This means jumping right before touching a spring, will turn off gravity and then launch the player
- Player can stand on phantom blocks without activating them
  - Only breaks this way when standing on the corner
  - This is because the player collider is the circle and is a bit smaller than player
  - It's this way to be forgiving on spikes though perhaps I should change the spike collider instead

## Bugs

- Cannot preload (with a stage load event) on build complete, will try building that stage immediately and fail
  - Need to test the build failed events at different points (1 stage in, 0 stages in etc)
  - Seems like it's not scrubbing the current stage
  - Also might not be taking the user back to stage select
  - NOTE this happens when preloading the next, non-existent stage, in the BuildComplete event
- Removing Groundable or Wallable component will not remove Grounded or TouchingWall component
  - Can add a system to check Changed<Groundedable> but for now, who cares?
  - Consider removing groundable and wallable all together?
  - Same thing with controllers Changed<JumpController>
  - NOTE: Changed isn't how you track removed components, that is something different
- Stupid texture bleeding comes back if the PC is trash (such as NU's laptop)

## Refactorings

- Move the logic to get the atlas rect into the factories
  - ie the obstacle factories like saw or spike
- Add in actual player states, or SOMETHING that works with networking
  - Using components as state works well for local players but not networked
  - Take look direction, if you press left it should be left, but we don't get input from networked players.
    - So to keep it as a state we need to add a looking state and update it for local and networked players
  - This will become a larger issue when more effects are added for onJump or onDeath etc

## Web Build

- Use wss protocol for matchbox server
  - Also please add in a better way of joining networking
  - Right now it sets up the socket and then says it's connected
  - But that setup might fail, so there will need to be a check
  - You can use ws protocol on firefox to replicate it failing
- Make networked players predictive
  - Add velocity to their message and set it on read

## Networking

- [ ] Have 2 channels for reliable and unreliable
  - Reliable will contain events such as player death and spawn (also time? - does reliable keep ordering)
  - Unreliable will contain messages like position update
- [ ] Must cater to multiple message types in a message
- [x] Fix the networking state machine to handle the socket not connecting
- [ ] Network Retries
  - Retry count derived from NetworkingPreferences, describes how many times it should auto retry, before requiring manual intervention

## Stage Mechanics

- [x] Key Blocks
  - Keys are placed around the stage
  - Collect the key to unlock the assigned locked blocks
  - Different keys will be assigned to different blocks in one stage
- [ ] Moving platforms
  - Horizontal or vertical
  - Moving platforms should also be able to contain spikes etc
    - The moving 'block' could just be a spike
    - Last project handled this by moving them all separately but in the same way
- [ ] Crumbling blocks
  - On touch (by anything?), they dissapear
- [ ] Ghost blocks
  - They look solid until the player is close
  - Once one in the group is revealed, all of the group is revealed
- [x] Interval blocks
  - They 'switch' on and off every x seconds
  - When on, they are just blocks, when off, they have dissapear
  - Can be triggered by timer or maybe even player events (button/switch?)
- [ ] Crushing blocks
  - A trigger subscriber that usually just stays on a timer
  - Can act as a door if it subscribes to a non timer trigger?
  - Will crush the player if they hit the bottom or top of it as well as touching ground/ceiling
- [x] Springs
- [ ] Teleporters
- [ ] Enemies that will act as springs when they die
- [ ] Heavy blocks
  - Basically just blocks that have gravity
  - Can put lock blocks beneath them so the player collects a key and must race to the end before the heavy block falls, blocking the exit

## Player Mechanics

- Dash
  - Works on ground and air
  - Restores after touching ground (+delay) or wall
- Slide
  - Press down to trigger
  - Only works when grounded
  - Gives a bit of a boost in speed like dash? (might be weird in air to slide transition)
  - Player becomes shorter and can slide under things

## Thoughts on Multiplayer

- Main issue is that small short stages will become cluttered with players
- Fix #1: Can instead move to the server relay architecture with matchmaking
  - This would then look for any game on that stage, with less than x players
  - If there is one, join it -- if there isn't one, create a new lobby on that map
  - This way players will have a decent amount of fellow players without it becoming cluttered
  - DRAWBACK: Once you move to the next stage, you're unlikely to see the player you were just with agian
- Fix #2: Make stages far longer and with checkpoints
  - Players should be more spread out
  - You stay with the same players for longer
  - DRAWBACK: If the game gets lots of attention, clutter will still be a big issue
- Fix #3: Combine #1 and #2, make lobbies with long stages
  - Scales to many players well
  - You still get to play with the same players

## Random Ideas

- Ability to mute (/automute) other players

## Competitive Multiplayer Idea

- Put players in a lobby
- A stage is picked and players play for x minutes, trying to set their best time
- Best time after x minutes gets a point etc
- Players play i amount of maps until a winner is declared

## LevelReset

- Will likely need a component LevelResetter or similar that can reset the level when the player dies.

So we store an id with every stage object
They key objects contain a list of objectIds for their triggers

When creating the level, we go through the triggerables first and get a mapping <Id => EntityId>
Then we go through the rest and if they have triggers, we can set the correct entityId

## other shit idk

Saw vs Half Saw Issue

- Issue with saws is that a lot of the time you may want a half saw to go along the ground, and then go up into the ceiling and along it, but it's only half a saw.
- Can make it so half saws can ONLY connect to the ground and not go up in the air, and full saws must be used for that, but it's restrictive

Saw Placement

- Have the user select saw
- Then they click on the first tile they want it
- If the path should end there, click that tile again
- Otherwise click a different tile that is vertical or horizontal to set it's path to there next
- Continue until happy with path and click the same tile again to end path
- Right click to undo last path update
- ISSUE: If I want to add breaks to the path, where the saw stops for a second or so, how?

Idea for block that produces spikes when the player steps on it, simiar to crumbling block
