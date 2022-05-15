use std::f32::consts::PI;

use crate::kmath::*;
use crate::manifest::*;
use crate::wave_game::*;
use crate::entity_definitions::*;
use crate::entity::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spell {
    Missile,
    Pulse,
    Lifesteal,
    SummonBloodcasters,
    ConeFlames,
    SummonRushers,
    SummonSummoners,
    Fireball,
    Firestorm,
    Water,
    Homing,
}

impl WaveGame {
    pub fn cast_spell(&mut self, t: f32, caster_id: u32, target: Vec2, spell: Spell, repeat: bool) {
        let caster_team = self.team.get(&caster_id).unwrap().team;
        let caster_pos = self.rect.get(&caster_id).unwrap().centroid();
        if let Some(cc) = self.caster.get_mut(&caster_id) {
            match spell {
                Spell::ConeFlames => {
                    // frame rate dependent...... needs to emit a certain amount per unit time
                    let cost = 1.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                        self.add_flame_projectile(caster_id, target, t);
                    }
                },
                Spell::Water => {
                    // frame rate dependent...... needs to emit a certain amount per unit time
                    let cost = 0.4;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                        self.add_water_projectile(caster_id, target, t);
                    }
                },
                Spell::Missile => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 10.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let missile = Entity::new()
                            .with_team(caster_team)
                            .with_physics(10.0, (target - caster_pos).normalize() * 15.0)
                            .with_rect(Rect::new_centered(caster_pos.x, caster_pos.y, 0.4, 0.4))
                            .with_projectile(caster_id, 34.0)
                            .with_emitter(0.05, Vec3::new(0.8, 0.0, 0.8), 2.0, 0.7, 0.1)
                            .with_render_solid(Vec3::new(0.8, 0.0, 0.8));
                        self.add_entity(missile);
                    }
                },
                Spell::Homing => {
                    let colour = Vec3::new(0.0, 0.4, 1.0);
                    let speed = 5.0;
                    
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 25.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let m1 = Entity::new()
                            .with_team(caster_team)
                            .with_physics(10.0, (target - caster_pos).normalize() * speed)
                            .with_rect(Rect::new_centered(caster_pos.x, caster_pos.y, 0.5, 0.5))
                            .with_projectile(caster_id, 20.0)
                            .with_emitter(0.05, colour, 2.0, 0.7, 0.1)
                            .with_ai(999999.0, 0.0, speed, 0.8) // see if it works lmao
                            .with_render_solid(colour);
                        let m2 = m1.clone()
                            .with_physics(10.0, (target - caster_pos).normalize().rotate(-1.0) * speed);
                        let m3 = m1.clone()
                            .with_physics(10.0, (target - caster_pos).normalize().rotate(1.0) * speed);

                        self.add_entity(m1);
                        self.add_entity(m2);
                        self.add_entity(m3);
                    }
                },
                Spell::Pulse => {
                    if cc.last_cast + 0.1 > t { return ; }
                    cc.last_cast = t;
                    let cost = 6.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let missile = Entity::new()
                            .with_team(caster_team)
                            .with_physics(4.0, (target - caster_pos).normalize() * 25.0)
                            .with_rect(Rect::new_centered(caster_pos.x, caster_pos.y, 0.4, 0.4))
                            .with_projectile(caster_id, 34.0)
                            .with_emitter(0.05, Vec3::new(0.0, 0.8, 0.0), 3.0, 0.3, 0.1)
                            .with_render_solid(Vec3::new(0.0, 0.8, 0.0))
                            .with_expiry(t + (4.0 / 25.0));
                    self.add_entity(missile);
                    }
                },
                Spell::Lifesteal => {
                    if cc.last_cast + 0.5 > t { return ; }
                    cc.last_cast = t;
                    let cost = 10.0;
                    let mut hp = self.health.get_mut(&caster_id).unwrap();
                    if hp.current >= cost {
                        hp.current -= cost;
                        let missile = Entity::new()
                            .with_team(caster_team)
                            .with_physics(4.0, (target - caster_pos).normalize() * 10.0)
                            .with_rect(Rect::new_centered(caster_pos.x, caster_pos.y, 0.4, 0.4))
                            .with_projectile_ex(caster_id, 20.0, 0.0, 0.0, 0.7)
                            .with_emitter(0.05, Vec3::new(0.8, 0.0, 0.0), 2.0, 0.7, 0.1)
                            .with_render_solid(Vec3::new(0.8, 0.0, 0.0));
                        self.add_entity(missile);
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
                Spell::Firestorm => {
                    if cc.last_cast + 0.25 > t { return ; }
                    cc.last_cast = t;
                    let cost = 8.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        self.add_entity(Entity::new()
                            .with_team(caster_team)
                            .with_physics(4.0, (target - caster_pos).normalize() * 15.0)
                            .with_rect(Rect::new_centered(caster_pos.x, caster_pos.y, 0.5, 0.5))
                            .with_projectile_ex(caster_id, 18.0, 2.0, 0.0, 0.0)
                            .with_emitter(0.05, Vec3::new(1.0, 0.0, 0.0), 2.0, 0.7, 0.15)
                            .with_render_solid(Vec3::new(1.0, 0.0, 0.0)));
                    }
                },
                Spell::SummonRushers => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 20.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let pos = self.rect.get(&caster_id).unwrap().centroid();
                        let team = self.team.get(&caster_id).unwrap().team;
    
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 0.0));
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 2.0*PI / 3.0));
                        self.add_zerg_enemy(team, pos.offset_r_theta(1.0, 4.0*PI / 3.0));
                    }
                },
                Spell::SummonBloodcasters => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 50.0;
                    let mut hp = self.health.get_mut(&caster_id).unwrap();
                    if hp.current >= cost {
                        hp.current -= cost;
                        let pos = self.rect.get(&caster_id).unwrap().centroid();
                        let team = self.team.get(&caster_id).unwrap().team;
                        
                        self.add_bloodcaster(team, pos.offset_r_theta(1.0, 0.0));
                        self.add_bloodcaster(team, pos.offset_r_theta(1.0, PI));
                    }
                },
                Spell::SummonSummoners => {
                    if repeat { return; }
                    if cc.last_cast + 0.3 > t { return ; }
                    cc.last_cast = t;
                    let cost = 100.0;
                    if cc.mana >= cost {
                        cc.mana -= cost;
                        let pos = self.rect.get(&caster_id).unwrap().centroid();
                        let team = self.team.get(&caster_id).unwrap().team;
                        self.add_summoner_enemy(team, pos.offset_r_theta(2.0, 0.0));
                        self.add_summoner_enemy(team, pos.offset_r_theta(2.0, 2.0*PI / 3.0));
                        self.add_summoner_enemy(team, pos.offset_r_theta(2.0, 4.0*PI / 3.0));
                    }
                },
            }
        }
    }
}

pub fn spell_sprite(spell: Spell) -> i32 {
    match spell {
        Spell::ConeFlames => ICON_FIRE,
        Spell::Missile => ICON_MAGIC_MISSILE,
        Spell::Pulse => ICON_PULSE,
        Spell::Firestorm => ICON_FIRESTORM,
        Spell::Lifesteal => ICON_BLOOD_MISSILE,
        Spell::SummonBloodcasters => ICON_BLOOD_ACOLYTES,
        Spell::SummonRushers => ICON_SUMMON_ZERGS,
        Spell::Fireball => ICON_FIREBALL,
        Spell::Water => ICON_WATER,
        
        Spell::Homing => ICON_HOMING,
        
        Spell::SummonSummoners => 0,
    }
}