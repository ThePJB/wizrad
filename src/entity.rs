use crate::components::make_entities::*;
use crate::kmath::*;

use crate::wave_game::*;
use crate::spell::*;

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
use crate::components::melee_damage::*;
use crate::components::spawn_list::*;

#[derive(Clone)]
pub struct Entity {
    pub ai: Option<AI>,
    pub ai_caster: Option<AICaster>,
    pub team: Option<Team>,
    pub caster: Option<Caster>,
    pub emitter: Option<Emitter>,
    pub health: Option<Health>,
    pub melee_damage: Option<MeleeDamage>,
    pub projectile: Option<Projectile>,
    pub render: Option<Render>,
    pub expiry: Option<Expiry>,
    pub physics: Option<Physics>,
    pub player: Option<Player>,
    pub rect: Option<Rect>,
    pub spawn_list: Option<SpawnList>,
    pub make_on_damage: Option<MakeEntitiesOnDamage>,
    pub make_on_death: Option<MakeEntitiesOnDeath>,
}

// there could be such a ncie builder

impl Entity {
    pub fn new() -> Entity {
        Entity {
            ai: None,
            ai_caster: None,
            team: None,
            caster: None,
            emitter: None,
            health: None,
            melee_damage: None,
            projectile: None,
            render: None,
            expiry: None,
            physics: None,
            player: None,
            rect: None,
            spawn_list: None,
            make_on_damage: None,
            make_on_death: None,
        }
    }

    pub fn with_ai(mut self, acquisition_range: f32, flee_range: f32, speed: f32, accel: f32) -> Entity {
        self.ai = Some(AI {
            acquisition_range,
            flee_range,
            speed,
            accel,
            dir: Vec2::new(0.0, 0.0),
        });
        self
    }

    pub fn with_team(mut self, team: u32) -> Entity {
        self.team = Some(Team {
            team
        });
        self
    }

    pub fn with_emitter(mut self, interval: f32, colour: Vec3, speed: f32, lifespan: f32, size: f32) -> Entity {
        self.emitter = Some(Emitter {
            interval,
            colour,
            speed,
            lifespan,
            size: Vec2::new(size, size),
            last: 0.0,
        });
        self
    }

    pub fn with_health(mut self, max: f32, regen: f32) -> Entity {
        self.health = Some(Health {
            max,
            current: max,
            regen,
            invul_time: -1.0,
        });
        self
    }

    pub fn with_melee_damage(mut self, amount: f32) -> Entity {
        self.melee_damage = Some(MeleeDamage {
            amount
        });
        self
    }

    pub fn with_projectile(mut self, source: u32, damage: f32) -> Entity {
        self.projectile = Some(Projectile {
            source,
            damage,
            aoe: 0.0,
            splat_duration: 0.0,
            lifesteal_percent: 0.0,
        });
        self
    }

    pub fn with_projectile_ex(mut self, source: u32, damage: f32, aoe: f32, splat_duration: f32, lifesteal_percent: f32) -> Entity {
        self.projectile = Some(Projectile {
            source,
            damage,
            aoe,
            splat_duration,
            lifesteal_percent,
        });
        self
    }

    pub fn with_render_solid(mut self, colour: Vec3) -> Entity {
        self.render = Some(Render::Colour(colour));
        self
    }

    pub fn with_expiry(mut self, when: f32) -> Entity {
        self.expiry = Some(Expiry {
            expiry: when,
        });
        self
    }

    pub fn with_physics(mut self, mass: f32, velocity: Vec2) -> Entity {
        self.physics = Some(Physics {
            mass,
            old_pos: Vec2::new(0.0, 0.0), // ruh roh shraggy
            velocity,
        });
        self
    }
    
    pub fn with_rect(mut self, rect: Rect) -> Entity {
        self.rect = Some(rect);
        self
    }
    
    pub fn with_position(mut self, pos: Vec2) -> Entity {
        let r = self.rect.unwrap();
        self.rect = Some(Rect::new_centered(pos.x, pos.y, r.w, r.h));
        self
    }

    pub fn with_ai_caster(mut self, acquisition_range: f32, spell: Spell) -> Entity {
        self.ai_caster = Some(AICaster {
            acquisition_range,
            spell,
            unleasher: false,
            rising: false,
        });
        self
    }
    pub fn with_ai_caster_unleasher(mut self, acquisition_range: f32, spell: Spell) -> Entity {
        self.ai_caster = Some(AICaster {
            acquisition_range,
            spell,
            unleasher: true,
            rising: false,
        });
        self
    }

    pub fn with_caster(mut self, mana_max: f32, mana_regen: f32) -> Entity {
        self.caster = Some(Caster {
            mana_max,
            mana: mana_max,
            mana_regen,
            last_cast: -10000.0,
        });
        self
    }

    pub fn with_player(mut self, speed: f32, spellbook: Vec<Spell>) -> Entity {
        self.player = Some(Player {
            spellbook,
            speed,
            spell_cursor: 0,
            kills: 0,
        });
        self
    }
}

impl WaveGame {
    pub fn add_entity(&mut self, entity: &Entity) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        if let Some(player) = entity.player.clone() {
            self.player.insert(id, player);
        }
        if let Some(caster) = entity.caster.clone() {
            self.caster.insert(id, caster);
        }
        if let Some(expiry) = entity.expiry.clone() {
            self.expiry.insert(id, expiry);
        }
        if let Some(ai) = entity.ai.clone() {
            self.ai.insert(id, ai);
        }
        if let Some(ai_caster) = entity.ai_caster.clone() {
            self.ai_caster.insert(id, ai_caster);
        }
        if let Some(physics) = entity.physics.clone() {
            self.physics.insert(id, physics);
        }
        if let Some(render) = entity.render.clone() {
            self.render.insert(id, render);
        }
        if let Some(team) = entity.team.clone() {
            self.team.insert(id, team);
        }
        if let Some(emitter) = entity.emitter.clone() {
            self.emitter.insert(id, emitter);
        }
        if let Some(projectile) = entity.projectile.clone() {
            self.projectile.insert(id, projectile);
        }
        if let Some(melee_damage) = entity.melee_damage.clone() {
            self.melee_damage.insert(id, melee_damage);
        }
        if let Some(health) = entity.health.clone() {
            self.health.insert(id, health);
        }
        if let Some(rect) = entity.rect {
            self.rect.insert(id, rect);
        }
        if let Some(spawn_list) = entity.spawn_list.clone() {
            self.spawn_list.insert(id, spawn_list);
        }
        if let Some(make_on_damage) = entity.make_on_damage.clone() {
            self.make_on_damage.insert(id, make_on_damage);
        }
        if let Some(make_on_death) = entity.make_on_death.clone() {
            self.make_on_death.insert(id, make_on_death);
        }
    } 
}