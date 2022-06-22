use crate::application::*;
use crate::components::make_entities::*;
use crate::components::melee_damage::MeleeDamage;
use crate::particles::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::kgui::*;
use crate::kmath::*;
use crate::spell::*;
use crate::manifest::*;
use crate::entity_definitions::*;
use crate::actual_entity_definitions::*;
use crate::spawner::*;
use crate::spell_menu::*;
use crate::entity::*;

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
use crate::components::spawn_list::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

use glutin::event::VirtualKeyCode;

pub struct Event {
    pub condition: fn(&WaveGame, &FrameInputState) -> bool,
    pub effect: fn(&mut WaveGame, &FrameInputState),
}

pub struct DamageEvent {
    amount: f32,
    src: u32,
    target: u32,
}

pub enum Command {
    Cast(u32, Vec2, Spell, bool),
}

pub const LOOK_STRENGTH: f32 = 0.2;
pub const SCALE: f32 = 20.0;

pub struct WaveGame {
    pub wave: i32,

    pub last_spawn: f32,

    pub t: f64,
    pub look_center: Vec2,
    pub pause: bool,

    pub particle_system: ParticleSystem,
    pub spawn_system: Spawner,

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
    pub spawn_list: HashMap<u32, SpawnList>,
    pub make_on_damage: HashMap<u32, MakeEntitiesOnDamage>,
    pub make_on_death: HashMap<u32, MakeEntitiesOnDeath>,

    pub spell_menu: Option<SpellMenu>,

    pub events: Vec<Event>,
}

impl WaveGame {
    pub fn new(time: f32) -> WaveGame {
        let mut spawner = Spawner::new();
        // spawner.add_spawn_entity(entity_basic(Vec2::new(0.0, 0.0)), 4.0);
        // spawner.add_spawn_entity(entity_zerg(TEAM_ENEMIES, Vec2::new(0.0, 0.0)), 2.5);
        // // spawner.add_spawn_entity(entity_bloodcaster(TEAM_ENEMIES, Vec2::new(0.0, 0.0)), 7.0);
        // spawner.add_spawn_entity(entity_caster(Vec2::new(0.0, 0.0)), 5.0);
        // // spawner.add_spawn_entity(entity_pulsecaster(Vec2::new(0.0, 0.0)), 9.5);
        // spawner.add_spawn_entity(entity_summoner(TEAM_ENEMIES, Vec2::new(0.0, 0.0)), 10.0);
        // spawner.add_spawn_entity(entity_summoner_summoner(TEAM_ENEMIES, Vec2::new(0.0, 0.0)), 20.0);

        let mut wg = WaveGame {
            wave: 0,
            last_spawn: 0.0,
            pause: false,

            t: 0.0,
            look_center: Vec2::new(0.0, 0.0),
            particle_system: ParticleSystem{particles: Vec::new()},
            spawn_system: spawner,
            
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
            spawn_list: HashMap::new(),
            make_on_damage: HashMap::new(),
            make_on_death: HashMap::new(),
            
            events: Vec::new(),

            spell_menu: None,
        };

        // wg.add_player(Vec2::new(0.0, 5.0)); old
        
        wg.events.push(Event {
            condition: |wg, inputs| {
                wg.player.values().nth(0).unwrap().spellbook.len() == 0
            }, effect: |wg, inputs| {
                let current_spells = &wg.player.values().nth(0).unwrap().spellbook;
                wg.spell_menu = Some(SpellMenu::new(inputs.seed * 172137163, current_spells, inputs.t as f32));
            }
        });
        
        wg.events.push(Event {
            condition: |wg, inputs| {
                wg.player.values().nth(0).unwrap().spellbook.len() == 1
            }, effect: |wg, inputs| {
                let current_spells = &wg.player.values().nth(0).unwrap().spellbook;
                wg.spell_menu = Some(SpellMenu::new(inputs.seed * 3487498743, current_spells, inputs.t as f32));
            }
        });
        
        wg.events.push(Event {
            condition: |wg, inputs| {
                wg.player.values().nth(0).unwrap().spellbook.len() == 2
            }, effect: |wg, inputs| {
                wg.add_entity(&portal3(Vec2::new(0.0, 0.0), TEAM_ENEMIES));
                wg.wave = 1;
            }
        });

        wg.events.push(Event {
            condition: |wg, inputs| {
                if wg.spawn_list.values().nth(0).is_none() {return false};
                let spawnlist_component = wg.spawn_list.values().nth(0).unwrap();
                let n_enemies = wg.team.iter().filter(|(id, com)| com.team == TEAM_ENEMIES).count() as i32;
                wg.wave == 1 && 
                n_enemies == 0 &&
                spawnlist_component.t > spawnlist_component.list[spawnlist_component.list.len() - 1].0

            }, effect: |wg, inputs| {
                let id = wg.spawn_list.keys().nth(0).unwrap();
                wg.remove_entity(*id);
                wg.add_entity(&portal2(Vec2::new(0.0, 0.0), TEAM_ENEMIES));
                wg.wave = 2;
            }
        });
        wg.add_entity(&entity_player(Vec2::new(0.0, 0.0)));

        println!("Welcome to WAVE GAME. Controls are WASD movement, Q-E spellbook page, Left click to cast. Survive all rounds. ");

        wg
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
    
        pub fn damage_entity(&mut self, inputs: &FrameInputState, id: u32, amount: f32, buf: &mut Vec<Entity>) {
            if !self.health.contains_key(&id) { return };
            let health = self.health.get_mut(&id).unwrap();
            health.damage(amount, inputs.t as f32);
    
            if let Some(make_on_damage) = self.make_on_damage.get_mut(&id) {
                make_on_damage.acc += amount;
                if make_on_damage.acc > make_on_damage.thresh {
                    make_on_damage.acc = 0.0;
                    (make_on_damage.f)(self, inputs, id, buf);
                }
            }
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
        self.make_on_damage.remove(&entity_id);
        self.make_on_death.remove(&entity_id);
        self.spawn_list.remove(&entity_id);
    }

fn update(&mut self, inputs: &FrameInputState, mut commands: Vec<Command>) {
    let mut dead_list = Vec::new();
    let mut commands = Vec::new();
    let mut new_entities = Vec::new();
    let level_rect = Rect::new_centered(0.0, 0.0, 40.0, 40.0);
    
    self.t += inputs.dt;

    // emit particles
    for (id, ec) in self.emitter.iter_mut() {
        let mut iter_count = 0;
        if ec.last + ec.interval < self.t as f32 {
            iter_count += 1;
            ec.last += ec.interval;
            let pos = self.rect.get(&id).unwrap().centroid();
            let seed = inputs.frame * 12315 + *id * 1412337 + iter_count;
            self.particle_system.add_particle(Particle {
                expiry: self.t as f32 + ec.lifespan,
                velocity: Vec2::new(kuniform(seed, -1.0, 1.0), kuniform(seed * 1771715, -1.0, 1.0)).normalize() * ec.speed,
                rect: Rect::new_centered(pos.x, pos.y, ec.size.x, ec.size.y),
                colour: ec.colour,
            });
        }
    }

    self.particle_system.update(self.t as f32, inputs.dt as f32);
    
    // AI
    self.update_movement_ai(self.t as f32, inputs.dt as f32, inputs.frame, level_rect);
    self.update_casting_ai(self.t as f32, &mut commands);
    
    for command in commands {
        match command {
            Command::Cast(caster_id, target, spell, repeat) => {
                self.cast_spell(self.t as f32, caster_id, target, spell, repeat, inputs.seed, inputs.dt as f32);
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

        // maybe I should get damage events, that would make the code a bit nicer
        // in some sense collision events -> damage events
        let mut damage_events = Vec::new();

        // handle projectile impacts
        for ce in collision_events.iter() {
            if let Some(proj) = self.projectile.get(&ce.subject) {
                let impact_location = self.rect.get(&ce.object).unwrap().centroid();
                let proj_team = self.team.get(&ce.subject).unwrap().team;
                let target_team = self.team.get(&ce.object).unwrap().team;
                if proj.aoe > 0.0 {
                    for (id, _) in self.physics.iter().filter(|(id, com)| self.rect.get(id).unwrap().centroid().dist(impact_location) <= proj.aoe && proj_team != target_team) {
                        if let Some(health) = self.health.get_mut(&id) {
                            damage_events.push(DamageEvent{amount: proj.damage, src: proj.source, target: *id});
                            // self.damage_entity(&inputs, *id, proj.damage);
                            // health.damage(proj.damage, inputs.t as f32);
                            if let Some(caster_hp) = self.health.get_mut(&proj.source) {
                                caster_hp.current += proj.lifesteal_percent * proj.damage;
                                caster_hp.current = caster_hp.current.min(caster_hp.max);
                            }
                        }
                    }
                } else {
                    if let Some(health) = self.health.get_mut(&ce.object) {
                        damage_events.push(DamageEvent{amount: proj.damage, src: proj.source, target: ce.object});
                        // health.damage(proj.damage, inputs.t as f32);
                        if let Some(caster_hp) = self.health.get_mut(&proj.source) {
                            caster_hp.current += proj.lifesteal_percent * proj.damage;
                            caster_hp.current = caster_hp.current.min(caster_hp.max);
                        }
                    }
                }
                let pos = self.rect.get(&ce.subject).unwrap().centroid();
                if proj.splat_duration > 0.0 {
                    self.add_firesplat(pos, self.t as f32);
                }
                dead_list.push(ce.subject);
            }
        }

        // handle melee damage
        let melee_damage_events: Vec<DamageEvent> = collision_events.iter()
            .filter(|ce| self.team.contains_key(&ce.subject) && self.team.contains_key(&ce.object) && self.team.get(&ce.subject).unwrap().team != self.team.get(&ce.object).unwrap().team)
            .filter_map(|ce| {
                if let Some(md) = self.melee_damage.get(&ce.subject) {
                    Some(DamageEvent {src: ce.subject, target: ce.object, amount: md.amount})
                } else {
                    None
                }
        }).collect();

        // filter iframes. actually its done in health. could clean up
        // assign credit for kills here too
        for damage in damage_events.iter().chain(melee_damage_events.iter()) {
            self.damage_entity(&inputs, damage.target, damage.amount, &mut new_entities);
        }

        // self.resolve_melee_damage(&collision_events, inputs.t as f32);

        // expire timed lives
        for (id, timed) in self.expiry.iter() {
            if timed.expiry < self.t as f32 {
                dead_list.push(*id);
            }
        }

        // kill 0 hp
        for (&id, hc) in self.health.iter() {
            if hc.current <= 0.0 {
                dead_list.push(id);
            }
        }

        self.fix_overlaps(&collision_events, 1.0);

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
            if let Some(make_on_death) = self.make_on_death.get(&dead) {
                (make_on_death.f)(self, &inputs, dead, &mut new_entities);
            }
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
    }
}

impl Scene for WaveGame {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let start = Instant::now();
        let mut reset = false;
        let mut commands = Vec::new();
        let mut dead_list = Vec::new();
        let mut new_entities = Vec::new();
        let mouse_world = WaveGame::screen_to_world(inputs.mouse_pos, self.look_center, inputs.mouse_pos, inputs.screen_rect);
        let level_rect = Rect::new_centered(0.0, 0.0, 30.0, 30.0);

        // Events
        let trigger_events: Vec<usize> = self.events.iter().enumerate().filter_map(|(idx, e)| if (e.condition)(&self, &inputs) {Some(idx)} else {None}).collect();
        for idx in trigger_events.iter() {
            (self.events[*idx].effect)(self, &inputs);
        }
        for idx in trigger_events.iter() {
            self.events.swap_remove(*idx);
        }

        // Inputs
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::Escape, true) => {true}, _ => {false}}) {
            return (SceneOutcome::QuitProgram, TriangleBuffer::new(inputs.screen_rect), None);
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::R, true) => {true}, _ => {false}}) {
            if self.player.iter().nth(0).is_none() {
                reset = true;
            }
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::M, true) => {true}, _ => {false}}) {
            for (id, com) in self.team.iter() {
                if com.team == TEAM_ENEMIES {
                    dead_list.push(*id);
                }
            }
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
            *self = WaveGame::new(inputs.t as f32);
        }

        // refactor this, own t, pause if spell menu
        // maybe have a update game routine



        let start = Instant::now();

        let mut commands = Vec::new();

        let mouse_world = WaveGame::screen_to_world(inputs.mouse_pos, self.look_center, inputs.mouse_pos, inputs.screen_rect);


        let level_rect = Rect::new_centered(0.0, 0.0, 40.0, 40.0);


        let mut reset = false;

        if let Some(player) = self.player.values().nth(0) {
            if player.spellbook.len() < 1 || 
                    (player.kills > 10 && player.spellbook.len() < 2) ||
                    (player.kills > 100 && player.spellbook.len() < 3) ||
                    (player.kills > 300 && player.spellbook.len() < 4) ||
                    (player.kills > 600 && player.spellbook.len() < 5)
                      {
                if self.spell_menu.is_none() {
                    self.spell_menu = Some(SpellMenu::new(inputs.seed, &player.spellbook, inputs.t as f32));
                }
            } else {
                // spawning
                if let Some(ent) = self.spawn_system.frame(&inputs) {
                    self.add_entity(&ent);
                }
            }
        }


        // Inputs
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::Escape, true) => {true}, _ => {false}}) {
            return (SceneOutcome::QuitProgram, TriangleBuffer::new(inputs.screen_rect), None);
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::R, true) => {true}, _ => {false}}) {
            // if self.player.iter().nth(0).is_none() {
                reset = true;
            // }
        }
        if inputs.events.iter().any(|e| match e { KEvent::Keyboard(VirtualKeyCode::P, true) => {true}, _ => {false}}) {
            self.pause = !self.pause;
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
            *self = WaveGame::new(inputs.t as f32);
        }

        if self.spell_menu.is_none() && self.pause == false {
            self.update(&inputs, commands);
        }


        
        // spawn list stuff
        for (id, spawn_list) in self.spawn_list.iter_mut() {
            spawn_list.t += inputs.dt as f32;
            for (t_spawn, e_spawn) in spawn_list.list.iter() {
                if *t_spawn < spawn_list.t && *t_spawn > spawn_list.t - inputs.dt as f32 {
                    new_entities.push(*e_spawn.clone());
                }
            }
        }

        for (i, entity) in new_entities.iter_mut().enumerate() {
            entity.rect.unwrap().x += kuniform(i as u32 * 17231653 + inputs.seed, -0.05, 0.05);
            entity.rect.unwrap().y += kuniform(i as u32 * 12983715 + inputs.seed, -0.05, 0.05);
            self.add_entity(entity);
        }

        {   // trying to separate so they dont get physics kick
            // it actually gets more cooked the more you do lol
            // maybe its just when they spawn exactly on top of one another.
            // yeah this is just fucked, what if it is exactly on top case? or one within

            // i reckon within just do centroid, some amount, and if centroid is equal, random direction, just make sure its opposite
            // just not enough noise
            for i in 0..4 {
                let mut cols = self.collisions();
                
                // filter projectils etc
                cols.retain(|ce| {
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
                self.fix_overlaps(&cols, 0.25);
            }
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
        self.draw_entities(&mut buf, self.t as f32, inputs.frame);
        
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


