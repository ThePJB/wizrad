use crate::application::*;
use crate::components::melee_damage::MeleeDamage;
use crate::particles::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::kgui::*;
use crate::kmath::*;
use crate::collision_system::*;
use crate::manifest::*;

use crate::components::entity_common::*;
use crate::components::ai::*;
use crate::components::health::*;
use crate::components::caster::*;
use crate::components::projectile::*;
use crate::components::render::*;
use crate::components::timed_life::*;
use crate::components::emitter::*;

use std::collections::HashMap;
use std::collections::HashSet;

use glutin::event::VirtualKeyCode;


/*
Fat entities with hashmap (randomized handle) is what I did last time
it seems the best way

other options are what,
ECS. Each component needs to store an ID that can be used to access all other components?

heterogenous storage with COOH, but I think you would need to store a reference to allow recovery of parent
or really separate your concerns so that it was never a problem (spoiler alert impossible)
or store information in component
but still the player would never know what it had collided with

or you have combinatorial explostion for each heterogenous category (but it is fastest)
*/

// how am I doing camera tho
// return what screen rect should be?

// maybe no point in commands because i was just insta applying them
// unless theres anything best implemented by interfering with the buffer

#[derive(Clone, Copy)]
pub enum Spell {
    Missile,
    ConeFlames,
    HealSelf,
    Fireball,
    CurseOfFear,
}

pub struct PlayerController {
    spellbook: Vec<Spell>,
    spell_cursor: usize,
}

// how about entity IDs
// if I had a controllable storm cloud what would happen:
// player tries to cast 'storm cloud spell'. I could check the entities for a storm cloud on player's force with the status summoned
// yeah how often to entities need to reference one another, and plus I could always add it however.

pub struct WaveGame {
    last_spawn: f32,

    look_center: Vec2,

    particle_system: ParticleSystem,

    entity_id_counter: u32,
    entity_ids: HashSet<u32>,

    player_controller: HashMap<u32, PlayerController>,
    common: HashMap<u32, Common>,
    caster: HashMap<u32, Caster>,
    health: HashMap<u32, Health>,
    ai: HashMap<u32, AI>,
    projectile: HashMap<u32, Projectile>,
    render: HashMap<u32, Render>,
    timed_life: HashMap<u32, TimedLife>,
    melee_damage: HashMap<u32, MeleeDamage>,
    emitter: HashMap<u32, Emitter>,
    ai_caster: HashMap<u32, AICaster>,
}

impl WaveGame {
    pub fn new() -> WaveGame {
        let mut wg = WaveGame {
            last_spawn: 0.0,
            look_center: Vec2::new(0.0, 0.0),
            particle_system: ParticleSystem{particles: Vec::new()},
            entity_id_counter: 0,
            entity_ids: HashSet::new(),
            player_controller: HashMap::new(),
            common: HashMap::new(),
            caster: HashMap::new(),
            health: HashMap::new(),
            ai: HashMap::new(),
            projectile: HashMap::new(),
            render: HashMap::new(),
            timed_life: HashMap::new(),
            melee_damage: HashMap::new(),
            emitter: HashMap::new(),
            ai_caster: HashMap::new(),
        };

        wg.add_player(Vec2::new(0.0, 0.0));
        for i in 0..30 {
            let pos = Vec2::new(
                (krand(i)-0.5) * 40.0,
                (krand(i*1324121)-0.5) * 40.0,
            );
            wg.add_fbm_enemy(pos);
        }

        wg
    }

    pub fn add_player(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;

        self.entity_ids.insert(id);
        self.player_controller.insert(id, PlayerController {
            spellbook: vec![Spell::Missile, Spell::ConeFlames], 
            spell_cursor: 0,
        });
        self.common.insert(id, Common {
            team: TEAM_PLAYER, 
            rect: Rect::new_centered(pos.x, pos.y, 1.0, 1.0),
            speed: 10.0, 
            velocity: Vec2::new(0.0, 0.0),
        });
        self.caster.insert(id, Caster { 
            mana: 100.0,
            mana_max: 100.0, 
            mana_regen: 10.0,
            last_cast: 0.0,
        });
        self.health.insert(id, Health {
            current: 100.0,
            max: 100.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.8, 0.8, 0.8)));
    }
    
    pub fn add_fbm_enemy(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.common.insert(id, Common {
            team: TEAM_ENEMIES, 
            rect: Rect::new_centered(pos.x, pos.y, 0.7, 0.7),
            speed: 8.0, 
            velocity: Vec2::new(0.0, 0.0),
        });
        self.health.insert(id, Health {
            current: 50.0,
            max: 50.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            kind: AIKind::Roamer,
            target_location: pos, 
            last_update: 0.0, 
            update_interval: 2.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(1.0, 0.0, 0.0)));
        self.melee_damage.insert(id, MeleeDamage { amount: 20.0 });
    }
    pub fn add_zerg_enemy(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.common.insert(id, Common {
            team: TEAM_ENEMIES, 
            rect: Rect::new_centered(pos.x, pos.y, 0.5, 0.5),
            speed: 8.0, 
            velocity: Vec2::new(0.0, 0.0),
        });
        self.health.insert(id, Health {
            current: 20.0,
            max: 20.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            kind: AIKind::Rush,
            target_location: pos, 
            last_update: 0.0, 
            update_interval: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.7, 0.0, 0.0)));
        self.melee_damage.insert(id, MeleeDamage { amount: 20.0 });
    }
    pub fn add_caster_enemy(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.common.insert(id, Common {
            team: TEAM_ENEMIES, 
            rect: Rect::new_centered(pos.x, pos.y, 0.9, 0.9),
            speed: 3.0, 
            velocity: Vec2::new(0.0, 0.0),
        });
        self.health.insert(id, Health {
            current: 20.0,
            max: 20.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            kind: AIKind::Rush,
            target_location: pos, 
            last_update: 0.0, 
            update_interval: 0.0,
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::Missile,
            acquisition_range: 7.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 50.0,
            mana_regen: 5.0,
            mana: 0.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.0, 0.8, 0.8)));
    }

    pub fn add_projectile(&mut self, caster: u32, target: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);


        let (team, pos, v) = {
            let caster_comp = self.common.get(&caster).unwrap();
            let caster_pos = caster_comp.rect.centroid();
            let v = (target - caster_comp.rect.centroid()).normalize() * 15.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };

        self.common.insert(id, Common {
            team: team, 
            rect: Rect::new_centered(pos.x, pos.y, 0.4, 0.4),
            speed: 10.0, 
            velocity: v,
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 34.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.8, 0.0, 0.8)));
        self.emitter.insert(id, Emitter {
            interval: 0.05,
            last: 0.0,
            colour: Vec3::new(0.8, 0.0, 0.8),
            size: Vec2::new(0.1, 0.1),
            speed: 2.0,
            lifespan: 0.7,
        });

    }

    pub fn add_flame_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let lifespan = kuniform(id * 4234777, 0.6, 0.8);

        let (team, pos, v) = {
            let caster_comp = self.common.get(&caster).unwrap();
            let caster_pos = caster_comp.rect.centroid();
            let v = (target - caster_comp.rect.centroid()).normalize() * 10.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };

        let spray = 0.25;
        let spray_angle = kuniform(id * 4134123, -spray, spray);
        let v = v.rotate(spray_angle);

        self.common.insert(id, Common {
            team: team, 
            rect: Rect::new_centered(pos.x, pos.y, 0.2, 0.2),
            speed: 10.0, 
            velocity: v,
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 2.0,
        });

        self.render.insert(id, Render::FOfT(FOfT {
            f: |t| {
                let fire_gradient = vec![
                    (Vec3::new(1.0, 1.0, 1.0), 0.0),
                    (Vec3::new(1.0, 1.0, 0.0), 0.3),
                    (Vec3::new(1.0, 0.0, 0.0), 0.6),
                    (Vec3::new(0.0, 0.0, 0.0), 1.0),
                ];
              gradient(t, fire_gradient)  
            },
            t_start: t,
            t_end: t + lifespan,
        }));
        self.timed_life.insert(id, TimedLife {expiry: t + lifespan});
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.entity_ids.remove(&entity_id);
        self.ai.remove(&entity_id);
        self.health.remove(&entity_id);
        self.player_controller.remove(&entity_id);
        self.common.remove(&entity_id);
        self.caster.remove(&entity_id);
        self.projectile.remove(&entity_id);
        self.render.remove(&entity_id);
        self.timed_life.remove(&entity_id);
        self.melee_damage.remove(&entity_id);
        self.emitter.remove(&entity_id);
        self.ai_caster.remove(&entity_id);
    }

    // p is in world space, how to make it into screen space
    pub fn screen_to_world(p: Vec2, world_tether: Vec2, look_offset: Vec2, screen_rect: Rect) -> Vec2 {
        let dims = SCALE * screen_rect.br();
        let look_vec = SCALE * look_offset - dims/2.0;
        let screen_center = world_tether + LOOK_STRENGTH * look_vec;
        let cam_rect = Rect::new_centered(screen_center.x, screen_center.y, dims.x, dims.y);

        // the rect that represents where the camera is in world space
        // maybe the child function
        
        Vec2::new(
            cam_rect.x + cam_rect.w * p.x / screen_rect.w,
            cam_rect.y + cam_rect.h * p.y / screen_rect.h,
        )
    }

    pub fn cast_spell(&mut self, t: f32, caster_id: u32, target: Vec2, spell: Spell, repeat: bool) {
        if let Some(cc) = self.caster.get_mut(&caster_id) {
            match spell {
                Spell::ConeFlames => {
                    // frame rate dependent...... needs to emit a certain amount per unit time
                    let cost = 0.8;
                    if cc.mana > cost {
                        cc.mana -= cost;
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                    }
                },
                Spell::Missile => {
                    if repeat { return; }
                    if cc.last_cast + 0.5 > t { return ; }
                    cc.last_cast = t;
                    let cost = 10.0;
                    if cc.mana > cost {
                        cc.mana -= cost;
                        self.add_projectile(caster_id, target);
                    }
                },
                _ => {},
            }
        }
    }
}

pub const LOOK_STRENGTH: f32 = 0.2;
pub const SCALE: f32 = 10.0;

pub enum Command {
    Cast(u32, Vec2, Spell, bool),
}

impl Scene for WaveGame {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let level_min = -20.0;
        let level_max = 20.0;

        let mouse_world = WaveGame::screen_to_world(inputs.mouse_pos, self.look_center, inputs.mouse_pos, inputs.screen_rect);

        let mut commands = Vec::new();



        // spawning
        let spawn_interval = 1.0;
        if inputs.t as f32 - self.last_spawn > spawn_interval {
            self.last_spawn = inputs.t as f32;

            let pos = match khash(inputs.frame * 123415) % 4 {
                0 => Vec2::new(level_min, kuniform(inputs.frame * 138971377, level_min, level_max)),
                1 => Vec2::new(level_max, kuniform(inputs.frame * 138971377, level_min, level_max)),
                2 => Vec2::new(kuniform(inputs.frame * 138971377, level_min, level_max), level_min),
                3 => Vec2::new(kuniform(inputs.frame * 138971377, level_min, level_max), level_max),
                _ => panic!("unreachable"),
            };

            match khash(inputs.frame * 13498713) % 3 {
                0 => self.add_fbm_enemy(pos),
                1 => self.add_zerg_enemy(pos),
                2 => self.add_caster_enemy(pos),
                _ => panic!("unreachable"),
            }
        }


        let mut reset = false;

        // Inputs
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::Escape, true) => {true}, _ => {false}}) {
            return (SceneOutcome::Pop(SceneSignal::JustPop), TriangleBuffer::new(inputs.screen_rect), None);
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::R, true) => {true}, _ => {false}}) {
            reset = true;
        }

        for (id, cc) in self.player_controller.iter_mut() {
            let mut player_move_dir = Vec2::new(0.0, 0.0);
            if inputs.held_keys.contains(&VirtualKeyCode::W) {
                player_move_dir.y -= 1.0;
            }
            if inputs.held_keys.contains(&VirtualKeyCode::S) {
                player_move_dir.y += 1.0;
            }
            if inputs.held_keys.contains(&VirtualKeyCode::A) {
                player_move_dir.x -= 1.0;
            }
            if inputs.held_keys.contains(&VirtualKeyCode::D) {
                player_move_dir.x += 1.0;
            }
            if inputs.held_keys.contains(&VirtualKeyCode::Q) {
                if cc.spell_cursor > 0 { cc.spell_cursor -= 1 }
            }
            if inputs.held_keys.contains(&VirtualKeyCode::E) {
                if cc.spell_cursor < cc.spellbook.len() - 1 { cc.spell_cursor += 1 }
            }
            let pce = self.common.entry(*id);
            pce.and_modify(|e| e.velocity = player_move_dir.normalize() * e.speed);
            
            if inputs.events.iter().any(|e| match e { KEvent::MouseLeft(true) => {true}, _ => {false}}) {
                commands.push(Command::Cast(*id, mouse_world, cc.spellbook[cc.spell_cursor], false));
            } else if inputs.held_lmb {
                commands.push(Command::Cast(*id, mouse_world, cc.spellbook[cc.spell_cursor], true));
            }
        }
        
        if reset {
            *self = WaveGame::new();
        }

        // emit particles
        for (id, ec) in self.emitter.iter_mut() {
            if ec.last + ec.interval < inputs.t as f32 {
                ec.last += ec.interval;
                let pos = self.common.get(&id).unwrap().rect.centroid();
                let seed = inputs.frame * 12315 + *id * 1412337;
                self.particle_system.add_particle(Particle {
                    expiry: inputs.t as f32 + ec.lifespan,
                    velocity: Vec2::new(kuniform(seed, -1.0, 1.0), kuniform(seed * 1771715, -1.0, 1.0)).normalize() * ec.speed,
                    rect: Rect::new_centered(pos.x, pos.y, ec.size.x, ec.size.y),
                    colour: ec.colour,
                });
            }
        }

        self.particle_system.update(inputs.t as f32, inputs.dt as f32);
        

        // i need screen to world
        // world to screen is 

        let mut player_pos = Some(Vec2::new(0.0, 0.0));
        if let Some((pid, _)) = self.player_controller.iter().nth(0) {
            player_pos = Some(self.common.get(pid).unwrap().rect.centroid());
        } else {
            player_pos = None;
        }

        // AI
        for (id, ai) in self.ai.iter_mut() {
            let aic = self.common.get_mut(id).unwrap();

            match ai.kind {
                AIKind::Roamer => {
                    if inputs.t - ai.last_update > ai.update_interval {
                        ai.last_update += ai.update_interval;
                        let seed = inputs.frame * 2351352729 + id * 423476581;

                        if let Some(pos) = player_pos {
                            if (aic.rect.centroid() - pos).magnitude() < 5.0 {
                                ai.target_location = pos;
                            } else {
                                ai.target_location = aic.rect.centroid() + Vec2::new(krand(seed) - 0.5, krand(seed + 1) - 0.5).normalize() * 2.0;
                            }
                        }

                    }
                    
                },
                AIKind::Rush => {
                    if let Some(pos) = player_pos {
                        ai.target_location = pos;
                    }
                },
            }

            let dist = (ai.target_location - aic.rect.centroid()).magnitude();
            let speed = aic.speed.min(dist/inputs.dt as f32);
            aic.velocity = speed * (ai.target_location - aic.rect.centroid()).normalize();
        }

        for (id, ai_caster) in self.ai_caster.iter() {
            let (self_pos, self_team) = {
                let self_com = self.common.get(id).unwrap();
                (self_com.rect.centroid(), self_com.team)
            };
            if let Some((target_id, target_com)) = self.common.iter().filter(|(id, c)| c.team != self_team && (c.rect.centroid() - self_pos).magnitude() < ai_caster.acquisition_range).nth(0) {
                commands.push(Command::Cast(*id, target_com.rect.centroid(), ai_caster.spell, false));
            }
        }

        for command in commands {
            match command {
                Command::Cast(caster_id, target, spell, repeat) => {
                    self.cast_spell(inputs.t as f32, caster_id, target, spell, repeat);
                },
            }
        }


        let mut dead_list = Vec::new();

        // update entities
        let mut collision_events = Vec::new();
        collide_entity_entity(&self.common, &mut collision_events, inputs.dt as f32);

        // remove projectile collisions with their source or other projectiles
        collision_events.retain(|ce| {
            if let Some(subject_projectile) = self.projectile.get(&ce.subject) {
                if subject_projectile.source == ce.object {
                    return false;
                }
                if let Some(object_projectile) = self.projectile.get(&ce.object) {
                    return false;
                }
            }
            if let Some(object_projectile) = self.projectile.get(&ce.object) {
                if object_projectile.source == ce.subject {
                    return false;
                }
            }
            let steam = self.common.get(&ce.subject).unwrap().team;
            let oteam = self.common.get(&ce.object).unwrap().team;
            if steam == oteam {return false};
            
            true
        });

        // handle projectile impacts
        for ce in collision_events.iter() {
            if let Some(proj) = self.projectile.get(&ce.subject) {
                
                if let Some(health) = self.health.get_mut(&ce.object) {
                    health.current -= proj.damage;
                    if health.current <= 0.0 {
                        dead_list.push(ce.object);
                    }
                }
                dead_list.push(ce.subject);
            }
        }

        // handle melee damage
        for ce in collision_events.iter() {

            if let Some(md) = self.melee_damage.get(&ce.subject) {
                let subj_com = self.common.get(&ce.subject).unwrap();
                let obj_com = self.common.get(&ce.object).unwrap();

                if subj_com.team != obj_com.team {
                    if let Some(obj_health) = self.health.get_mut(&ce.object) {
                        if inputs.t as f32 - obj_health.invul_time > 0.25 {
                            obj_health.invul_time = inputs.t as f32;
                            obj_health.current -= md.amount;
                            if obj_health.current <= 0.0 {
                                dead_list.push(ce.object);
                            }
                        }
                    }
                }

            }
        }

        // expire timed lives
        for (id, timed) in self.timed_life.iter() {
            if timed.expiry < inputs.t as f32 {
                dead_list.push(*id);
            }
        }

        for dead in dead_list {
            self.remove_entity(dead);
        }


        apply_movement(&mut self.common, &collision_events, inputs.dt as f32);

        if let Some((id, _)) = self.player_controller.iter().nth(0) {
            let pc = self.common.get(id).unwrap();
            self.look_center = pc.rect.centroid();
        }

        // regen
        for caster in self.caster.values_mut() {
            caster.mana = caster.mana_max.min(caster.mana + caster.mana_regen * inputs.dt as f32)
        }
        for health in self.health.values_mut() {
            health.current = health.max.min(health.current + health.regen * inputs.dt as f32)
        }

        // Camera
        let scale = 15.0;
        let dims = scale * inputs.screen_rect.br();
        let look_vec = scale * inputs.mouse_pos - dims/2.0;
        let screen_center = self.look_center + 0.2 * look_vec;
        let cam_rect = Rect::new_centered(screen_center.x, screen_center.y, dims.x, dims.y);
        let mut buf = TriangleBuffer::new(cam_rect);

        // draw entities
        for (id, er) in self.render.iter() {
            let ec = self.common.get(id).unwrap();
            match er {
                Render::Colour(colour) => {
                    // if iframe
                    if let Some(health) = self.health.get(id) {
                        if inputs.t as f32 - health.invul_time < 0.25 {
                            buf.draw_rect(ec.rect, Vec3::new(1.0, 1.0, 1.0), 3.0)
                        } else {
                            buf.draw_rect(ec.rect, *colour, 3.0)
                        }
                    }
                    buf.draw_rect(ec.rect, *colour, 3.0)
                },
                Render::FOfT(f_of_t) => {
                    let t = unlerp(inputs.t as f32, f_of_t.t_start, f_of_t.t_end);
                    let c = (f_of_t.f)(t);
                    buf.draw_rect(ec.rect, c, 3.0);
                }
            }
        }

        // draw level
        let level_rect = Rect::new_centered(0.0, 0.0, 40.0, 40.0);
        buf.draw_rect(level_rect, Vec3::new(0.3, 0.3, 0.3), 1.0);
        for i in 0..20 {
            for j in 0..20 {
                buf.draw_rect(level_rect.grid_child(i, j, 20, 20).dilate(-0.1), Vec3::new(0.1, 0.1, 0.1), 1.5);
            }
        }

        // draw particles
        self.particle_system.draw(&mut buf);

        // draw gui
        buf.screen_rect = inputs.screen_rect;

        let hmsize = 0.1;

        let health_rect = inputs.screen_rect.child(0.0, 1.0 - hmsize, hmsize, hmsize).fit_center_square();
        let mana_rect = inputs.screen_rect.child(1.0 - hmsize, 1.0 - hmsize, hmsize, hmsize).fit_center_square();

        buf.draw_rect(health_rect, Vec3::new(0.0, 0.0, 0.0), 10.0);
        buf.draw_rect(mana_rect, Vec3::new(0.0, 0.0, 0.0), 10.0);

        if let Some((player_id, _)) = self.player_controller.iter().nth(0) {
            let player_health = self.health.get(player_id).unwrap();
            let player_cast = self.caster.get(player_id).unwrap();

            let player_health_amount = player_health.current / player_health.max;
            let player_mana_amount = player_cast.mana / player_cast.mana_max;

            buf.draw_rect(health_rect.child(0.0, 1.0 - player_health_amount, 1.0, player_health_amount), Vec3::new(1.0, 0.0, 0.0), 11.0);
            buf.draw_rect(mana_rect.child(0.0, 1.0 - player_mana_amount, 1.0, player_mana_amount), Vec3::new(0.0, 0.0, 1.0), 11.0);
        }


        (SceneOutcome::None, buf, None)
    }
}