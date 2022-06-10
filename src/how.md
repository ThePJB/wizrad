# What is a game
* Simulation with discrete timesteps (frames)
* Store some state (99% entities and then some timers, bools for menu etc)
## Every frame
 * if supposed to show spell menu, show spell menu
 * ...
 * if player pressed w, set velocity
 * ...
 * AIs move toward nearest enemy
 * AIs shoot at nearest enemy
 * ...
 * move entities according to their velocity
 * compute list of collisions
 * move entities back according to collisions
 * move entities back according to leaving arena
 * now update velocities based on the final position -- important
 * ...
 * handle projectile impacts (deal out damage, mark projectiles for deletion)
 * ...
 * remove everything marked for deletion
 * draw everything