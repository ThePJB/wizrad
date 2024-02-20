# How to compile / run
* There shouldnt be any dependancies other than a rust toolchian
* Probably get rustup and use that to install cargo etc.
* Might need nightly rust (`cargo default nightly`?)
  * Im sorry, the only thing nightly is for is casting arrays of float to arrays of u8 for openGL which shouldnt be so hard but it is
 * cargo run --release

 # How to play
 * WASD movement
 * QE flip through spellbook
 * left click to cast current spell
 * Escape quit, R reset

 ints for entities would make things much better

 this but spells overwrite and upgrade
 would need to redesign form the ground up
 its a lot of content
 lets do something more minimal instead


 oo spells you hold and it sprays a number of projectiles, displaying internal logic with effects and cool sounds
 you could honestly have reduced colour palette and be a circle lol.
 but yea honestly with circles and effects might not be that bad


 player character could be a floating tentacle/eyeball wizard alien
 simulated tentacles maybe a grid of x, y and z

 spawners active in a smaller range but they do deplete

 locked in a room with them is danger mode
 but otherwise you can be kinda cautious
 they could be totems or something

 allows you to partition experiences

 could have things with constrained threat detection too, like sneak grass
 imagine being able to hide from random patrolling encounters
 maybe have a approximate MST for roads that are patrolled
  or union of 2 MSTs etc

imagine interpolating between multiple reduced colour palettes over the course of a day
would look like the atmospheres changing

could straight up synthesize textures for terrain as well

lol having enemies and that just defined in simulation. well it saves other overhead. could just literally put everything in simulation
you would need codes for looking stuff up. cant get away from that I think


but everything has to be simulation objects. everything the enemies come out of. thats alright, make spawners


tbh this game needs to be done well. its the hyper tradeoff action game. give it flavoursome levels of openness and systemsness. while being simple as