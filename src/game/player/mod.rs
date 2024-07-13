

pub mod player_states;


// event that kills a player
    // removes all the controllers from the player (disables movement)
    // adds a respawn component that has a timer
        // the respawn component could also house a key input, so they only respawn on key input
        // timed_respawn component
        // input_respawn component
    // spawns death particles etc
    

// event that spawns a player
    //adds all the controllers back on the player
    // checks to see if there is already a player
        //if so, move them to spawn (reset)
        //else just spawn a brand new one in
