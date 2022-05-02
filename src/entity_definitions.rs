use crate::kmath::*;
use crate::spell::*;
use crate::wave_game::*;
use crate::manifest::*;
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
use crate::components::melee_damage::*;
use crate::components::physics::*;

impl WaveGame {
    pub fn add_player(&mut self, pos: Vec2) {
        let player = Entity::new()
            // .with_player(10.0, vec![Spell::Missile, Spell::Pulse, Spell::ConeFlames, Spell::Water, Spell::Fireball, Spell::SummonRushers, Spell::Lifesteal, Spell::SummonBloodcasters, Spell::Homing])
            .with_player(10.0, vec![])
            .with_caster(100.0, 15.0)
            .with_team(TEAM_PLAYER)
            .with_physics(1.0, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 1.0, 1.0))
            .with_health(100.0, 1.0)
            .with_render_solid(Vec3::new(1.0, 0.0, 1.0));
        self.add_entity(player);
    }
    
    pub fn add_fbm_enemy(&mut self, pos: Vec2) {
        let goon = Entity::new()
            .with_team(TEAM_ENEMIES)
            .with_physics(1.0, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 1.0, 1.0))

            .with_melee_damage(20.0)
            .with_health(50.0, 1.0)
            .with_ai(10.0, 0.0, 4.0, 6.0)
            .with_render_solid(Vec3::new(1.0, 0.0, 0.0));
        self.add_entity(goon);
    }

    pub fn add_zerg_enemy(&mut self, team: u32, pos: Vec2) {
        let zerg = Entity::new()
            .with_team(team)
            .with_physics(0.25, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 0.5, 0.5))
            .with_melee_damage(20.0)
            .with_health(20.0, 1.0)
            .with_ai(10.0, 0.0, 7.0, 6.0)
            .with_render_solid(Vec3::new(0.7, 0.0, 0.0));
        self.add_entity(zerg);
    }

    pub fn add_caster_enemy(&mut self, pos: Vec2) {
        let caster = Entity::new()
            .with_team(TEAM_ENEMIES)
            .with_physics(0.7, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 0.9, 0.9))
            .with_health(20.0, 1.0)
            .with_ai(10.0, 5.0, 3.0, 6.0)
            .with_render_solid(Vec3::new(0.0, 0.8, 0.8))
            .with_ai_caster(9.0, Spell::Missile)
            .with_caster(10.0, 3.0);
        self.add_entity(caster);
    }
    
    pub fn add_pulsecaster_enemy(&mut self, pos: Vec2) {
        let caster = Entity::new()
            .with_team(TEAM_ENEMIES)
            .with_physics(0.7, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 0.9, 0.9))

            .with_health(30.0, 1.0)
            .with_ai(10.0, 2.0, 5.0, 6.0)
            .with_render_solid(Vec3::new(0.8, 0.6, 0.8))
            .with_ai_caster(3.0, Spell::Pulse)
            .with_caster(50.0, 12.0);
        self.add_entity(caster);
    }
    
    pub fn add_bloodcaster(&mut self, team: u32, pos: Vec2) {
        let caster = Entity::new()
            .with_team(team)
            .with_physics(0.7, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 0.9, 0.9))
            .with_health(40.0, 1.0)
            .with_ai(10.0, 5.0, 5.0, 6.0)
            .with_render_solid(Vec3::new(0.4, 0.3, 0.3))
            .with_ai_caster(7.0, Spell::Lifesteal)
            .with_caster(0.0, 0.0);
        self.add_entity(caster);
    }

    pub fn add_summoner_enemy(&mut self, team: u32, pos: Vec2) {
        let caster = Entity::new()
            .with_team(team)
            .with_physics(2.0, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 1.2, 1.2))
            .with_health(100.0, 1.0)
            .with_ai(10.0, 0.0, 2.0, 6.0)
            .with_render_solid(Vec3::new(0.5, 0.0, 0.0))
            .with_ai_caster(10.0, Spell::SummonRushers)
            .with_caster(50.0, 4.0);
        self.add_entity(caster);
    }

    pub fn add_summoner_summoner_enemy(&mut self, team: u32, pos: Vec2) {
        let caster = Entity::new()
            .with_team(team)
            .with_physics(4.0, Vec2::new(0.0, 0.0))
            .with_rect(Rect::new_centered(pos.x, pos.y, 1.8, 1.8))
            .with_health(200.0, 1.0)
            .with_ai(10.0, 0.0, 1.6, 6.0)
            .with_render_solid(Vec3::new(0.3, 0.0, 0.0))
            .with_ai_caster(12.0, Spell::SummonSummoners)
            .with_caster(100.0, 3.0);
        self.add_entity(caster);
    }
    
    pub fn add_flame_projectile(&mut self, caster: u32, target: Vec2, t: f32) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        let lifespan = kuniform(id * 4234777, 0.6, 0.8);

        let (team, pos, v) = {
            let caster_comp = self.team.get(&caster).unwrap();
            let caster_phys = self.physics.get(&caster).unwrap();
            let caster_pos = self.rect.get(&caster).unwrap().centroid();
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
            old_pos: Vec2::new(0.0, 0.0),
        });
        self.rect.insert(id, Rect::new_centered(pos.x, pos.y, 0.2, 0.2));
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
            let caster_pos = self.rect.get(&caster).unwrap().centroid();
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
            old_pos: Vec2::new(0.0, 0.0),
        });
        self.rect.insert(id, Rect::new_centered(pos.x, pos.y, 0.2, 0.2));
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
            let caster_pos = self.rect.get(&caster).unwrap().centroid();
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
            old_pos: Vec2::new(0.0, 0.0),
        });
        self.rect.insert(id, Rect::new_centered(pos.x, pos.y, 0.5, 0.5));
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
        self.rect.insert(id, Rect::new_centered(target.x, target.y, 0.5, 0.5));
        self.render.insert(id, Render::FireSplat(6.0));
        self.expiry.insert(id, Expiry {expiry: t + 0.4});
    }
}