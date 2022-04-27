use crate::kmath::*;
use crate::wave_game::*;
use crate::manifest::*;

use crate::components::team::*;
use crate::components::ai::*;
use crate::components::health::*;
use crate::components::caster::*;
use crate::components::projectile::*;
use crate::components::render::*;
use crate::components::expiry::*;
use crate::components::emitter::*;
use crate::components::player::*;
use crate::components::melee_damage::*;
use crate::components::physics::*;

impl WaveGame {
    pub fn add_player(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;

        self.entity_ids.insert(id);
        self.player.insert(id, Player {
            spellbook: vec![Spell::Missile, Spell::Pulse, Spell::ConeFlames, Spell::Water, Spell::Fireball, Spell::SummonRushers, Spell::Lifesteal, Spell::SummonBloodcasters], 
            spell_cursor: 0,
            speed: 10.0, 
        });
        self.team.insert(id, Team {
            team: TEAM_PLAYER, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 1.0,
            rect: Rect::new_centered(pos.x, pos.y, 1.0, 1.0),
        });
        self.caster.insert(id, Caster { 
            mana: 100.0,
            mana_max: 100.0, 
            mana_regen: 15.0,
            last_cast: 0.0,
        });
        self.health.insert(id, Health {
            current: 100.0,
            max: 100.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(1.0, 0.0, 1.0)));
    }
    
    pub fn add_fbm_enemy(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: TEAM_ENEMIES, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 1.0,
            rect: Rect::new_centered(pos.x, pos.y, 1.0, 1.0),
        });
        self.health.insert(id, Health {
            current: 50.0,
            max: 50.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 0.0,
            speed: 4.0, 
        });
        self.render.insert(id, Render::Colour(Vec3::new(1.0, 0.0, 0.0)));
        self.melee_damage.insert(id, MeleeDamage { amount: 20.0 });
    }
    pub fn add_zerg_enemy(&mut self, team: u32, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.25,
            rect: Rect::new_centered(pos.x, pos.y, 0.5, 0.5),
        });
        self.health.insert(id, Health {
            current: 20.0,
            max: 20.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 0.0,
            speed: 7.0, 
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.7, 0.0, 0.0)));
        self.melee_damage.insert(id, MeleeDamage { amount: 20.0 });
    }
    pub fn add_caster_enemy(&mut self, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: TEAM_ENEMIES, 
        });
        self.health.insert(id, Health {
            current: 20.0,
            max: 20.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.7,
            rect: Rect::new_centered(pos.x, pos.y, 0.9, 0.9),
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 5.0,
            speed: 3.0, 
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::Missile,
            acquisition_range: 9.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 10.0,
            mana_regen: 3.0,
            mana: 0.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.0, 0.8, 0.8)));
    }
    pub fn add_bloodcaster(&mut self, team: u32, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.7,
            rect: Rect::new_centered(pos.x, pos.y, 0.9, 0.9),
        });
        self.health.insert(id, Health {
            current: 40.0,
            max: 40.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 5.0,
            speed: 3.0, 
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::Lifesteal,
            acquisition_range: 7.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 10.0,
            mana_regen: 3.0,
            mana: 0.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.4, 0.3, 0.3)));
    }
    pub fn add_summoner_enemy(&mut self, team: u32, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 2.0,
            rect: Rect::new_centered(pos.x, pos.y, 1.2, 1.2),
        });
        self.health.insert(id, Health {
            current: 100.0,
            max: 100.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 0.0,
            speed: 2.0, 
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::SummonRushers,
            acquisition_range: 10.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 50.0,
            mana_regen: 5.0,
            mana: 50.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.5, 0.0, 0.0)));
    }
    pub fn add_pulsecaster_enemy(&mut self, team: u32, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.7,
            rect: Rect::new_centered(pos.x, pos.y, 0.9, 0.9),
        });
        self.health.insert(id, Health {
            current: 25.0,
            max: 25.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 2.0,
            speed: 5.0, 
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::Pulse,
            acquisition_range: 3.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 50.0,
            mana_regen: 12.0,
            mana: 50.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.8, 0.6, 0.0)));
    }
    pub fn add_summoner_summoner_enemy(&mut self, team: u32, pos: Vec2) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        
        self.entity_ids.insert(id);
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 4.0,
            rect: Rect::new_centered(pos.x, pos.y, 1.8, 1.8),
        });
        self.health.insert(id, Health {
            current: 200.0,
            max: 200.0,
            regen: 1.0,
            invul_time: 0.0,
        });
        self.ai.insert(id, AI { 
            dir: Vec2::new(0.0, 0.0), 
            acquisition_range: 10.0,
            flee_range: 0.0,
            speed: 1.6, 
        });
        self.ai_caster.insert(id, AICaster { 
            spell: Spell::SummonSummoners,
            acquisition_range: 10.0,
        });
        self.caster.insert(id, Caster { 
            mana_max: 100.0,
            mana_regen: 5.0,
            mana: 100.0,
            last_cast: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.3, 0.0, 0.0)));
    }

    pub fn add_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);


        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * 15.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };

        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 10.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.4, 0.4),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 34.0,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.0,
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
        self.expiry.insert(id, Expiry {expiry: t + 10.0});
    }
    pub fn add_blood_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);


        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * 10.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };

        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 4.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.4, 0.4),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 20.0,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.8,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.8, 0.0, 0.0)));
        self.emitter.insert(id, Emitter {
            interval: 0.05,
            last: 0.0,
            colour: Vec3::new(0.8, 0.0, 0.0),
            size: Vec2::new(0.1, 0.1),
            speed: 2.0,
            lifespan: 1.0,
        });
    }
    pub fn add_pulse(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let speed = 25.0;
        let range = 3.0;

        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * speed;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };

        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 4.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.4, 0.4),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 34.0,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.0,
        });
        self.emitter.insert(id, Emitter {
            interval: 0.01,
            last: 0.0,
            colour: Vec3::new(0.0, 0.8, 0.0),
            size: Vec2::new(0.1, 0.1),
            speed: 3.0,
            lifespan: 0.3,
        });
        self.render.insert(id, Render::Colour(Vec3::new(0.0, 0.8, 0.0)));
        self.expiry.insert(id, Expiry {expiry: t + range / speed});
    }

    pub fn add_flame_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let lifespan = kuniform(id * 4234777, 0.6, 0.8);

        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * 10.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };
        let spray = 0.25;
        let spray_angle = kuniform(id * 4134123, -spray, spray);
        let v = v.rotate(spray_angle);

        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 0.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.2, 0.2),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 2.0,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.0,
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
        self.expiry.insert(id, Expiry {expiry: t + lifespan});
    }

    pub fn add_water_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let lifespan = kuniform(id * 4234777, 0.6, 0.8);

        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * 6.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };
        let spray = 0.6;
        let spray_angle = kuniform(id * 4134123, -spray, spray);
        let v = v.rotate(spray_angle);

        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 20.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.2, 0.2),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 0.0,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.0,
        });

        self.render.insert(id, Render::FOfT(FOfT {
            f: |t| {
                let water_gradient = vec![
                    (Vec3::new(0.1, 0.3, 1.0), 0.0),
                    (Vec3::new(0.1, 0.3, 1.0), 0.8),
                    (Vec3::new(1.0, 1.0, 1.0), 1.0),
                ];
              gradient(t, water_gradient)  
            },
            t_start: t,
            t_end: t + lifespan,
        }));
        self.expiry.insert(id, Expiry {expiry: t + lifespan});
    }
    pub fn add_fireball(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = caster_phys.pos();
            let v = (target - caster_pos).normalize() * 10.0;
            let team = caster_comp.team;
            (team, caster_pos, v)
        };
        self.team.insert(id, Team {
            team: team, 
        });
        self.physics.insert(id, Physics {
            velocity: v,
            mass: 1.0,
            rect: Rect::new_centered(pos.x, pos.y, 0.5, 0.5),
        });
        self.projectile.insert(id, Projectile {
            source: caster,
            damage: 50.0,
            aoe: 4.0,
            splat_duration: 0.7,
            lifesteal_percent: 0.0,
        });
        self.render.insert(id, Render::Colour(Vec3::new(1.0, 0.1, 0.0)));
        self.emitter.insert(id, Emitter {
            interval: 0.2,
            last: 0.0,
            size: Vec2::new(0.15, 0.15),
            speed: 0.3,
            colour: Vec3::new(0.3, 0.3, 0.3),
            lifespan: 0.5,
        });
        self.expiry.insert(id, Expiry {expiry: t + 10.0});
    }
    pub fn add_firesplat(&mut self, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        self.team.insert(id, Team {
            team: TEAM_NEUTRAL, 
        });
        self.physics.insert(id, Physics {
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.0,
            rect: Rect::new_centered(target.x, target.y, 0.5, 0.5),
        });
        self.render.insert(id, Render::FireSplat(6.0));
        self.expiry.insert(id, Expiry {expiry: t + 0.4});
    }
}