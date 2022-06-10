# How to compile / run
* There shouldnt be any dependancies other than a rust toolchian
* Probably get rustup and use that to install cargo etc.
* Might need nightly rust (`rustup default nightly`?)
  * Im sorry, the only thing nightly is for is casting arrays of float to arrays of u8 for openGL which shouldnt be so hard but it is
 * cargo run --release

 # How to play
 * WASD movement
 * QE flip through spellbook
 * left click to cast current spell
 * Escape quit, R reset


it sucks specifying events, conditions, actions.
have events: condition fn ptr, action: e.g. make events
triggers can let you capture context I spose

e.g. if player hasnt got 2 spells: spell menu
if player has got 2 spells: start wave1 portal, insert dead portal event
if wave1 portal is dead:  


its gettin there
probably if spawnlist is done

event system is actually really good. closeures are fine if they are fn ptrs. mm segregated state, composition
no capture but its fine for those global things



maybe spells could be function pointers