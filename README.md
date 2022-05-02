# How to compile / run
* There shouldnt be any dependancies other than a rust toolchian
* Probably get rustup and use that to install cargo etc.
* The only bullshit thing you need to do is set it to nightly (`cargo default nightly`?)
  * Im sorry, the only thing nightly is for is casting arrays of float to arrays of u8 for openGL which shouldnt be so fucking hard as to require nightly features
 * cargo run --release

 # How to play
 * WASD movement
 * QE flip through spellbook
 * left click to cast current spell
 * Escape quit, R reset, M skip wave (no cheating)


# TODO
AI roam component,
Aggression component

see if thats nicer


put in acceleration
do a physics engine

could functions just mutate the array of ai components
could components just write their data to the other components that need them. that sounds like the true data oriented way


hiding in bushes


True structure:
game has a "wave"
returns a player
takes a player, thats if progression gets added

waves are scenes, pop 


last level: the tesseract / plato's cave

big structures to dodge, hexagonal ring of balls

spawning code is a joke at the moment

maybe physics code


gameplay loop is an allegory

plato is the final boss
or is it nietszche
or is it godel


i just wanna express spells and enemies more nicely


its quite possible that fat entities would let me express like spawn on death as a function of the dying entity, 


could spell selection literally just make 3 game instances of spells being cast and loop them



# Physics
Position based dynamics

Have a bunch of physics components:
Rect, vel(frame), mass

Move rects by vel*dt
then penetration is v
move one w*v and the other -(1-w)*v

implicit update of velocity Vel = (Pos - PrevPos)/dt

Substep:
1. Move all rects
2. Calculate all overlaps and put into collision events
3. For each collision event, move participants back according to w and v

you can get the velocity from that and update the thing's velocity. Have kinetic spells that push enemies, eg water spray

movement can apply force for slip and slide or it can rule with an iron fist




not real proud of the splats either

start using entity builders boi


----

Big refactor

movement seems erratic lol. but they dont overlap.

more die juice please

water: like fire but hella mass, push around

enemies acceleration


# Now todos
tinker with physics. maybe make behaviour more reliable with substeps
and get single collisions, first guy collided with
  sufficient to just recalculate penetration vector?
stop spawn cumming guys into my screen
could make homing missile


# well
immediate mode is god tier, combination full power declarative
but if things need to happen at different times you dont get that

# nemesis

yeah you could so have a nemesis who portals into your instances, fucking asshole
goal of tracking them down (+ their boss etc)

/ your increased prolificness has attracted attention of the time police, chance that they detect you and portal into your instances
craft another thing to make them fuck off
maybe you want their spells
oh yeah when your instance is collapsing make these holes that are parallax looking
but if you sruvive long enuogh you can portal and enter the singularity

spells to change instance
spells where the price is decaying the instance. the capitalist school of magic.


spells

ensnaring roots, guys within aoe of you get rooted

put a ward in the ground, line between you and ward is damaging
piercing projectile
4ways projectiles
homing projectiles
controllable projectile
direct aoe damage, op

black hole, lmao, test the physics engine

guys who reflect projectiles
guys who shoot stuff when they take damage (dont fire them bro)


next release
clear waves + goal
clear hotkeys instruction
choosing spells
spell gui -- done

right click could be an item you have to charge - persistent mana, super strong


on death - soz you died lad, r to reset
betwixt waves, spell options menu
maybe want that one to be transparent
or at least not immediately kill player

kill counts a bit of a reward
summary of spells you had
what wave you got to (plz finish)

todos:
  fix buggy splat
  death screen imminent
