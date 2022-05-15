use crate::application::*;
use crate::components::caster;
use crate::components::melee_damage::MeleeDamage;
use crate::particles::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::kgui::*;
use crate::kmath::*;
use crate::spell::*;
use crate::manifest::*;
use crate::entity_definitions::*;

use crate::components::team::*;
use crate::components::ai::*;
use crate::components::health::*;
use crate::components::caster::*;
use crate::components::projectile::*;
use crate::components::render::*;
use crate::components::expiry::*;
use crate::components::emitter::*;
use crate::components::player::*;
use crate::components::physics::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::INFINITY;
use std::f32::consts::PI;
use std::time::{Instant, Duration};
use std::convert::TryInto;

use glutin::event::VirtualKeyCode;

pub struct SpellMenu {
    choices: [Spell; 3],
}

impl SpellMenu {
    pub fn new(seed: u32, current_spells: &[Spell]) -> SpellMenu {
        let mut spell_list = vec![
            Spell::Missile,
            Spell::Fireball,
            Spell::ConeFlames,
            Spell::Pulse,
            Spell::Firestorm,
            Spell::Lifesteal,
            Spell::SummonBloodcasters,
            Spell::SummonRushers,
            Spell::Homing,
        ];

        spell_list.retain(|s| !current_spells.contains(s));
        shuffle_vec(&mut spell_list, seed);
        
        SpellMenu { choices: spell_list[0..3].try_into().unwrap() }

    }

    pub fn frame(&self, inputs: &FrameInputState, buf: &mut TriangleBuffer, buf_uv:  &mut TriangleBufferUV) -> Option<Spell> {

        let spell_menu = inputs.screen_rect.dilate(-0.35).fit_aspect_ratio(3.0);//.translate(Vec2::new(0.0, 0.2));
        // buf.draw_rect(spell_menu, Vec3::new(0.2, 0.2, 0.2), 15.0);
        for i in 0..3 {
            let btn_rect = spell_menu.grid_child(i, 0, 3, 1).dilate(-0.01);
            buf.draw_rect(btn_rect, Vec3::new(0.1, 0.1, 0.1), 15.0);
            buf.draw_rect(btn_rect.dilate(-0.01), Vec3::new(0.3, 0.3, 0.3), 15.5);
            let spell_rect = btn_rect.dilate(-0.01);
            if spell_rect.contains(inputs.mouse_pos) {
                buf.draw_rect(spell_rect, Vec3::new(1.0, 1.0, 1.0), 16.0);
                if inputs.events.iter().any(|event| match event {
                    KEvent::MouseLeft(false) => true,
                    _ => false,
                }) {
                    return Some(self.choices[i as usize]);
                }
            }
            buf_uv.draw_sprite(spell_rect, spell_sprite(self.choices[i as usize]), 17.0);
        }
        None
    }
}

pub enum State {
    Wave(i32),
    Spawn(i32),
    Recess(i32),
}

pub struct WaveGame {
    pub last_spawn: f32,

    pub state: State,

    pub look_center: Vec2,

    pub particle_system: ParticleSystem,

    pub entity_id_counter: u32,
    pub entity_ids: HashSet<u32>,

    pub player: HashMap<u32, Player>,
    pub team: HashMap<u32, Team>,
    pub caster: HashMap<u32, Caster>,
    pub health: HashMap<u32, Health>,
    pub ai: HashMap<u32, AI>,
    pub projectile: HashMap<u32, Projectile>,
    pub render: HashMap<u32, Render>,
    pub expiry: HashMap<u32, Expiry>,
    pub melee_damage: HashMap<u32, MeleeDamage>,
    pub emitter: HashMap<u32, Emitter>,
    pub ai_caster: HashMap<u32, AICaster>,
    pub physics: HashMap<u32, Physics>,
    pub rect: HashMap<u32, Rect>,

    pub spell_menu: Option<SpellMenu>,
}

impl WaveGame {
    pub fn new() -> WaveGame {
        let mut wg = WaveGame {
            state: State::Recess(0),
            last_spawn: 0.0,
            look_center: Vec2::new(0.0, 0.0),
            particle_system: ParticleSystem{particles: Vec::new()},
            
            entity_id_counter: 0,
            entity_ids: HashSet::new(),

            player: HashMap::new(),
            team: HashMap::new(),
            caster: HashMap::new(),
            health: HashMap::new(),
            ai: HashMap::new(),
            projectile: HashMap::new(),
            render: HashMap::new(),
            expiry: HashMap::new(),
            melee_damage: HashMap::new(),
            emitter: HashMap::new(),
            ai_caster: HashMap::new(),
            physics: HashMap::new(),
            rect: HashMap::new(),
            spell_menu: None,
        };

        wg.add_player(Vec2::new(0.0, 0.0));

        println!("Welcome to WAVE GAME. Controls are WASD movement, Q-E spellbook page, Left click to cast. Survive all rounds. ");

        wg
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.entity_ids.remove(&entity_id);
        self.ai.remove(&entity_id);
        self.health.remove(&entity_id);
        self.player.remove(&entity_id);
        self.team.remove(&entity_id);
        self.caster.remove(&entity_id);
        self.projectile.remove(&entity_id);
        self.render.remove(&entity_id);
        self.expiry.remove(&entity_id);
        self.melee_damage.remove(&entity_id);
        self.emitter.remove(&entity_id);
        self.ai_caster.remove(&entity_id);
        self.physics.remove(&entity_id);
        self.rect.remove(&entity_id);
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


    pub fn spawn(&mut self, id: u32, seed: u32) {
        let level_min = -14.5;
        let level_max = 14.5;

        let pos = match khash(seed * 123415) % 4 {
            0 => Vec2::new(level_min, kuniform(seed * 138971377, level_min, level_max)),
            1 => Vec2::new(level_max, kuniform(seed * 138971377, level_min, level_max)),
            2 => Vec2::new(kuniform(seed * 138971377, level_min, level_max), level_min),
            3 => Vec2::new(kuniform(seed * 138971377, level_min, level_max), level_max),
            _ => panic!("unreachable"),
        };

        match id {
            0 => self.add_fbm_enemy(pos),
            1 => self.add_zerg_enemy(TEAM_ENEMIES, pos),
            2 => self.add_caster_enemy(pos),
            3 => self.add_summoner_enemy(TEAM_ENEMIES, pos),
            4 => self.add_summoner_summoner_enemy(TEAM_ENEMIES, pos),
            5 => self.add_pulsecaster_enemy(pos),
            6 => self.add_bloodcaster(TEAM_ENEMIES, pos),
            _ => panic!("unreachable"),
        }
    }
}

pub const LOOK_STRENGTH: f32 = 0.2;
pub const SCALE: f32 = 20.0;

pub enum Command {
    Cast(u32, Vec2, Spell, bool),
}

impl Scene for WaveGame {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {

        let start = Instant::now();

        let mouse_world = WaveGame::screen_to_world(inputs.mouse_pos, self.look_center, inputs.mouse_pos, inputs.screen_rect);

        let mut commands = Vec::new();
        let mut dead_list = Vec::new();

        let level_rect = Rect::new_centered(0.0, 0.0, 30.0, 30.0);


        // new spawning
        let enemy_count = self.team.iter().filter(|(id, com)| com.team == TEAM_ENEMIES).count() as i32;
        if enemy_count == 0 {
            match self.state {
                State::Recess(0) |
                State::Recess(1) |
                State::Recess(3) |
                State::Recess(5)
                 => {
                    if self.spell_menu.is_none() {
                        if let Some(player) = self.player.values().nth(0) {
                            self.spell_menu = Some(SpellMenu::new(inputs.seed * 172137163, &player.spellbook));
                        }
                    }
                },
                State::Recess(n) => {
                    self.state = State::Spawn(n);
                },
                State::Spawn(n) => {
                    match n {
                        1 => {
                            for i in 0..10 {
                                self.spawn(0, inputs.seed + i);
                            }
                            for i in 0..20 {
                                self.spawn(1, inputs.seed * 12314121 + i);
                            }
                        },
                        2 => {
                            for i in 0..10 {
                                self.spawn(0, inputs.seed + i);
                            }
                            for i in 0..10 {
                                self.spawn(1, inputs.seed * 12314121 + i);
                            }
                            for i in 0..8 {
                                self.spawn(6, inputs.seed * 12364171 + i);
                            }
                        },
                        3 => {
                            for i in 0..10 {
                                self.spawn(0, inputs.seed + i);
                            }
                            for i in 0..10 {
                                self.spawn(2, inputs.seed * 12314121 + i);
                            }
                            for i in 0..5 {
                                self.spawn(5, inputs.seed * 95371 + i);
                            }
                        },
                        4 => {
                            for i in 0..10 {
                                self.spawn(0, inputs.seed + i);
                            }
                            for i in 0..10 {
                                self.spawn(2, inputs.seed * 12314121 + i);
                            }
                            for i in 0..10 {
                                self.spawn(3, inputs.seed * 12364171 + i);
                            }
        
                        },
                        5 => {
                            for i in 0..13 {
                                self.spawn(2, inputs.seed * 12314121 + i);
                            }
                            for i in 0..4 {
                                self.spawn(4, inputs.seed * 12364171 + i);
                            }
                        },
                        _ => {
                            println!("winner!");
                            return (SceneOutcome::Pop(SceneSignal::JustPop), TriangleBuffer::new(inputs.screen_rect), None);
                        },
                    }
                    self.state = State::Wave(n);
                    println!("Wave {}", n);
                },
                State::Wave(n) => {
                    self.state = State::Recess(n + 1);
                }
            }
        }

        let mut reset = false;

        // Inputs
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::Escape, true) => {true}, _ => {false}}) {
            return (SceneOutcome::QuitProgram, TriangleBuffer::new(inputs.screen_rect), None);
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::R, true) => {true}, _ => {false}}) {
            if self.player.iter().nth(0).is_none() {
                reset = true;
            }
        }
        // if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::M, true) => {true}, _ => {false}}) {
        //     for (id, com) in self.team.iter() {
        //         if com.team == TEAM_ENEMIES {
        //             dead_list.push(*id);
        //         }
        //     }
        // }

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
                if cc.spell_cursor < cc.spellbook.len() as i32 - 1 { cc.spell_cursor += 1 }
                let mut player_caster = self.caster.get_mut(id).unwrap();
                player_caster.last_cast = 0.0;
            }
            let p_phys = self.physics.entry(*id);
            p_phys.and_modify(|e| e.velocity = player_move_dir.normalize() * cc.speed);
            
            if self.spell_menu.is_none() {
                if inputs.events.iter().any(|e| match e { KEvent::MouseLeft(true) => {true}, _ => {false}}) {
                    commands.push(Command::Cast(*id, mouse_world, cc.spellbook[cc.spell_cursor as usize], false));
                } else if inputs.held_lmb {
                    commands.push(Command::Cast(*id, mouse_world, cc.spellbook[cc.spell_cursor as usize], true));
                }
            }
        }
        
        if reset {
            *self = WaveGame::new();
        }

        // emit particles
        for (id, ec) in self.emitter.iter_mut() {
            let mut iter_count = 0;
            if ec.last + ec.interval < inputs.t as f32 {
                iter_count += 1;
                ec.last += ec.interval;
                let pos = self.rect.get(&id).unwrap().centroid();
                let seed = inputs.frame * 12315 + *id * 1412337 + iter_count;
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
        self.update_movement_ai(inputs.t as f32, inputs.dt as f32, inputs.frame, level_rect);
        self.update_casting_ai(inputs.t as f32, &mut commands);
        
        for command in commands {
            match command {
                Command::Cast(caster_id, target, spell, repeat) => {
                    self.cast_spell(inputs.t as f32, caster_id, target, spell, repeat);
                },
            }
        }


        // update entities
        self.move_entities(inputs.dt as f32);
        let mut collision_events = self.collisions();
        // let mut collision_events = Vec::new();
        // collide_entity_entity(&self.common, &mut collision_events, inputs.dt as f32);

        collision_events.retain(|ce| {
            if !self.team.contains_key(&ce.subject) {return false;}
            if !self.team.contains_key(&ce.object) {return false;}

            let steam = self.team.get(&ce.subject).unwrap().team;
            let oteam = self.team.get(&ce.object).unwrap().team;
            let sproj = self.projectile.get(&ce.subject).is_some();
            let oproj = self.projectile.get(&ce.object).is_some();

            if steam == oteam && (oproj || sproj) {
                return false;
            }
            if oproj && sproj {
                return false;
            }

            return true
        });

        // handle projectile impacts
        for ce in collision_events.iter() {
            if let Some(proj) = self.projectile.get(&ce.subject) {
                let impact_location = self.rect.get(&ce.object).unwrap().centroid();
                let proj_team = self.team.get(&ce.subject).unwrap().team;
                let target_team = self.team.get(&ce.object).unwrap().team;
                if proj.aoe > 0.0 {
                    for (id, _) in self.physics.iter().filter(|(id, com)| self.rect.get(id).unwrap().centroid().dist(impact_location) <= proj.aoe && proj_team != target_team) {
                        if let Some(health) = self.health.get_mut(&id) {
                            health.damage(proj.damage, inputs.t as f32);
                            if let Some(caster_hp) = self.health.get_mut(&proj.source) {
                                caster_hp.current += proj.lifesteal_percent * proj.damage;
                                caster_hp.current = caster_hp.current.min(caster_hp.max);
                            }
                        }
                    }
                } else {
                    if let Some(health) = self.health.get_mut(&ce.object) {
                        health.damage(proj.damage, inputs.t as f32);
                        if let Some(caster_hp) = self.health.get_mut(&proj.source) {
                            caster_hp.current += proj.lifesteal_percent * proj.damage;
                            caster_hp.current = caster_hp.current.min(caster_hp.max);
                        }
                    }
                }
                let pos = self.rect.get(&ce.subject).unwrap().centroid();
                if proj.splat_duration > 0.0 {
                    self.add_firesplat(pos, inputs.t as f32);
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



        self.fix_overlaps(&collision_events, inputs.dt as f32);
        // apply_movement(&mut self.common, &collision_events, inputs.dt as f32);

        // constrain to arena
        for (id, rect) in self.rect.iter_mut() {

            if rect.top() < level_rect.top() {
                rect.y += level_rect.top() - rect.top();
                if self.projectile.contains_key(id) {
                    dead_list.push(*id);
                }
            }
            if rect.bot() > level_rect.bot() {
                rect.y += level_rect.bot() - rect.bot();
                if self.projectile.contains_key(id) {
                    dead_list.push(*id);
                }
            }
            if rect.left() < level_rect.left() {
                rect.x += level_rect.left() - rect.left();
                if self.projectile.contains_key(id) {
                    dead_list.push(*id);
                }
            }
            if rect.right() > level_rect.right() {
                rect.x += level_rect.right() - rect.right();
                if self.projectile.contains_key(id) {
                    dead_list.push(*id);
                }
            }
        }

        self.fix_velocities(inputs.dt as f32);

        for dead in dead_list {
            self.remove_entity(dead);
        }

        if let Some((id, _)) = self.player.iter().nth(0) {
            self.look_center = self.rect.get(id).unwrap().centroid();
        }

        // regen
        for caster in self.caster.values_mut() {
            caster.mana = caster.mana_max.min(caster.mana + caster.mana_regen * inputs.dt as f32)
        }
        for health in self.health.values_mut() {
            health.current = health.max.min(health.current + health.regen * inputs.dt as f32)
        }

        // Camera
        let scale = 20.0;
        let dims = scale * inputs.screen_rect.br();
        let look_vec = scale * inputs.mouse_pos - dims/2.0;
        let screen_center = self.look_center + 0.2 * look_vec;
        let cam_rect = Rect::new_centered(screen_center.x, screen_center.y, dims.x, dims.y);
        let mut buf = TriangleBuffer::new(cam_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);

        // draw entities
        self.draw_entities(&mut buf, inputs.t as f32, inputs.frame);
        
        // draw level
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
        buf_uv.draw_sprite(health_rect, VESSEL, 12.0);
        buf_uv.draw_sprite(mana_rect, VESSEL, 12.0);
        
        let spell_rect = inputs.screen_rect.child(0.5 - hmsize/2.0, 1.0 - hmsize, hmsize, hmsize).fit_center_square();
        let book_left_rect = spell_rect.translate(Vec2::new(-spell_rect.w/2.0, 0.0));
        let book_right_rect = spell_rect.translate(Vec2::new(spell_rect.w/2.0, 0.0));
        buf_uv.draw_sprite(book_left_rect, BOOK_LEFT, 11.0);
        buf_uv.draw_sprite(book_right_rect, BOOK_RIGHT, 11.0);
        if let Some(player) = self.player.values_mut().nth(0) {
            if player.spellbook.len() != 0 {
                buf_uv.draw_sprite(spell_rect, spell_sprite(player.spellbook[player.spell_cursor as usize]), 12.0);
            }

            // spell menu
            if let Some(spell_menu) = &self.spell_menu {
                if let Some(learned_spell) = spell_menu.frame(&inputs, &mut buf, &mut buf_uv) {
                    player.spellbook.push(learned_spell);
                    self.spell_menu = None;
                    match self.state {
                        State::Recess(0) => self.state = State::Recess(1),
                        State::Recess(n) => self.state = State::Spawn(n),
                        _ => {},
                    }
                }
            }
        }

        if self.player.iter().nth(0).is_none() {
            let reset_pane = inputs.screen_rect.child(0.4, 0.7, 0.2, 0.15).fit_aspect_ratio(2.0);
            buf_uv.draw_sprite(reset_pane.child(0.0, 0.0, 0.5, 1.0), TUT_R, 12.0);
            buf_uv.draw_sprite(reset_pane.child(0.5, 0.0, 0.5, 1.0), TUT_RESET, 12.0);
        }

        let frametime_ms = start.elapsed().as_secs_f32() * 1000.0;
        if frametime_ms > 1.0 {
            // println!("whoa that frame took forever: {}ms", frametime_ms);
        }

        (SceneOutcome::None, buf, Some(buf_uv))
    }
}

fn shuffle_vec(vec: &mut Vec<Spell>, mut seed: u32) {
    for i in 0..vec.len() {
        seed = khash(seed);
        let swap_idx = i + (seed % (vec.len() - i) as u32) as usize;
        vec.swap(i, swap_idx);
    }
}