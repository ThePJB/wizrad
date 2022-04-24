use crate::application::*;
use crate::components::caster;
use crate::components::melee_damage::MeleeDamage;
use crate::particles::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::kgui::*;
use crate::kmath::*;
use crate::collision_system::*;
use crate::manifest::*;
use crate::entity_definitions::*;

use crate::components::entity_common::*;
use crate::components::ai::*;
use crate::components::health::*;
use crate::components::caster::*;
use crate::components::projectile::*;
use crate::components::render::*;
use crate::components::expiry::*;
use crate::components::emitter::*;
use crate::components::player::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::INFINITY;
use std::f32::consts::PI;

use glutin::event::VirtualKeyCode;

#[derive(Clone, Copy)]
pub enum Spell {
    Missile,
    ConeFlames,
    SummonRushers,
    HealSelf,
    Fireball,
    CurseOfFear,
}

pub struct WaveGame {
    pub last_spawn: f32,

    pub look_center: Vec2,

    pub particle_system: ParticleSystem,

    pub entity_id_counter: u32,
    pub entity_ids: HashSet<u32>,

    pub player: HashMap<u32, Player>,
    pub common: HashMap<u32, Common>,
    pub caster: HashMap<u32, Caster>,
    pub health: HashMap<u32, Health>,
    pub ai: HashMap<u32, AI>,
    pub projectile: HashMap<u32, Projectile>,
    pub render: HashMap<u32, Render>,
    pub expiry: HashMap<u32, Expiry>,
    pub melee_damage: HashMap<u32, MeleeDamage>,
    pub emitter: HashMap<u32, Emitter>,
    pub ai_caster: HashMap<u32, AICaster>,
}

impl WaveGame {
    pub fn new() -> WaveGame {
        let mut wg = WaveGame {
            last_spawn: 0.0,
            look_center: Vec2::new(0.0, 0.0),
            particle_system: ParticleSystem{particles: Vec::new()},
            entity_id_counter: 0,
            entity_ids: HashSet::new(),
            player: HashMap::new(),
            common: HashMap::new(),
            caster: HashMap::new(),
            health: HashMap::new(),
            ai: HashMap::new(),
            projectile: HashMap::new(),
            render: HashMap::new(),
            expiry: HashMap::new(),
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

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.entity_ids.remove(&entity_id);
        self.ai.remove(&entity_id);
        self.health.remove(&entity_id);
        self.player.remove(&entity_id);
        self.common.remove(&entity_id);
        self.caster.remove(&entity_id);
        self.projectile.remove(&entity_id);
        self.render.remove(&entity_id);
        self.expiry.remove(&entity_id);
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
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                    }
                },
                Spell::Missile => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 10.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_projectile(caster_id, target, t);
                    }
                },
                Spell::Fireball => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 30.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_fireball(caster_id, target, t);
                    }
                },
                Spell::SummonRushers => {
                    if repeat { return; }
                    if cc.last_cast + 2.0 > t { return ; }
                    cc.last_cast = t;
                    let cost = 50.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let (pos, team) = {
                            let ccom = self.common.get(&caster_id).unwrap();
                            (ccom.rect.centroid(), ccom.team)
                        };
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 0.0));
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 2.0*PI / 3.0));
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 4.0*PI / 3.0));
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
        let spawn_interval = 3.0;
        if inputs.t as f32 - self.last_spawn > spawn_interval {
            self.last_spawn = inputs.t as f32;

            let pos = match khash(inputs.frame * 123415) % 4 {
                0 => Vec2::new(level_min, kuniform(inputs.frame * 138971377, level_min, level_max)),
                1 => Vec2::new(level_max, kuniform(inputs.frame * 138971377, level_min, level_max)),
                2 => Vec2::new(kuniform(inputs.frame * 138971377, level_min, level_max), level_min),
                3 => Vec2::new(kuniform(inputs.frame * 138971377, level_min, level_max), level_max),
                _ => panic!("unreachable"),
            };

            match khash(inputs.frame * 13498713) % 4 {
                0 => self.add_fbm_enemy(pos),
                1 => self.add_zerg_enemy(TEAM_ENEMIES, pos),
                2 => self.add_caster_enemy(pos),
                3 => self.add_summoner_enemy(pos),
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

        for (id, cc) in self.player.iter_mut() {
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
            if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::Q, true) => {true}, _ => {false}}) {
                if cc.spell_cursor > 0 { cc.spell_cursor -= 1 }
                let mut player_caster = self.caster.get_mut(id).unwrap();
                player_caster.last_cast = 0.0;
            }
            if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::E, true) => {true}, _ => {false}}) {
                if cc.spell_cursor < cc.spellbook.len() - 1 { cc.spell_cursor += 1 }
                let mut player_caster = self.caster.get_mut(id).unwrap();
                player_caster.last_cast = 0.0;
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
        
        // AI
        self.update_movement_ai(inputs.t as f32, inputs.dt as f32, inputs.frame);
        self.update_casting_ai(inputs.t as f32, &mut commands);
        
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

        collision_events.retain(|ce| {
            let steam = self.common.get(&ce.subject).unwrap().team;
            let oteam = self.common.get(&ce.object).unwrap().team;
            if steam == oteam {return false};
            if self.projectile.get(&ce.subject).is_some() && self.projectile.get(&ce.object).is_some() {return false};
            return true
        });

        // handle projectile impacts
        for ce in collision_events.iter() {
            if let Some(proj) = self.projectile.get(&ce.subject) {
                let impact_location = self.common.get(&ce.object).unwrap().rect.centroid();
                let proj_team = self.common.get(&ce.subject).unwrap().team;
                if proj.aoe > 0.0 {
                    // self.add_firesplat(impact_location, inputs.t as f32);
                    for (id, com) in self.common.iter().filter(|(id, com)| (com.rect.centroid() - impact_location).magnitude() <= proj.aoe && proj_team != com.team) {
                        if let Some(health) = self.health.get_mut(&id) {
                            health.current -= proj.damage;
                        }
                    }
                } else {
                    if let Some(health) = self.health.get_mut(&ce.object) {
                        health.current -= proj.damage;
                    }
                }
                dead_list.push(ce.subject);
            }
        }

        // handle melee damage
        self.resolve_melee_damage(&collision_events, inputs.t as f32);

        // expire timed lives
        for (id, timed) in self.expiry.iter() {
            if timed.expiry < inputs.t as f32 {
                dead_list.push(*id);
            }
        }

        // kill 0 hp
        for (&id, hc) in self.health.iter() {
            if hc.current <= 0.0 {
                dead_list.push(id);
            }
        }

        for dead in dead_list {
            self.remove_entity(dead);
        }

        apply_movement(&mut self.common, &collision_events, inputs.dt as f32);

        if let Some((id, _)) = self.player.iter().nth(0) {
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
        self.draw_entities(&mut buf, inputs.t as f32, inputs.frame);
        
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

        if let Some((player_id, _)) = self.player.iter().nth(0) {
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