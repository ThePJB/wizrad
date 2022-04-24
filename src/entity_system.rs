use crate::kmath::*;

use crate::entity::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::INFINITY;

use crate::components::entity_common::*;
use crate::components::ai::*;
use crate::components::health::*;
use crate::components::caster::*;
use crate::components::projectile::*;
use crate::components::render::*;
use crate::components::expiry::*;
use crate::components::emitter::*;
use crate::components::player::*;
use crate::components::melee_damage::*;

// No system code, just helper code!

pub struct EntitySystem {
    entity_id_counter: u32,
    entity_ids: HashSet<u32>,

    player: HashMap<u32, Player>,
    common: HashMap<u32, Common>,
    caster: HashMap<u32, Caster>,
    health: HashMap<u32, Health>,
    ai: HashMap<u32, AI>,
    projectile: HashMap<u32, Projectile>,
    render: HashMap<u32, Render>,
    expiry: HashMap<u32, Expiry>,
    melee_damage: HashMap<u32, MeleeDamage>,
    emitter: HashMap<u32, Emitter>,
    ai_caster: HashMap<u32, AICaster>,
}

impl EntitySystem {
    pub fn new() -> EntitySystem {
        EntitySystem {
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
        }
    }

    pub fn add_entity(&mut self, e: Entity) {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        self.entity_ids.insert(id);

        if let Some(c) = e.player {
            self.player.insert(id, c);
        }
        if let Some(c) = e.common {
            self.common.insert(id, c);
        }
        if let Some(c) = e.ai {
            self.ai.insert(id, c);
        }
        if let Some(c) = e.caster {
            self.caster.insert(id, c);
        }
        if let Some(c) = e.health {
            self.health.insert(id, c);
        }
        if let Some(c) = e.projectile {
            self.projectile.insert(id, c);
        }
        if let Some(c) = e.render {
            self.render.insert(id, c);
        }
        if let Some(c) = e.expiry {
            self.expiry.insert(id, c);
        }
        if let Some(c) = e.melee_damage {
            self.melee_damage.insert(id, c);
        }
        if let Some(c) = e.emitter {
            self.emitter.insert(id, c);
        }
        if let Some(c) = e.ai_caster {
            self.ai_caster.insert(id, c);
        }
    }

    pub fn has_health(&self, id: u32) -> bool {
        self.health.contains_key(&id)
    }
    pub fn team(&self, id: u32) -> u32 {
        self.common.get(&id).unwrap().team
    }
    pub fn pos(&self, id: u32) -> Vec2 {
        self.common.get(&id).unwrap().rect.centroid()
    }

    pub fn closest_pos_matching_pred(&self, pos: Vec2, pred: fn(&EntitySystem, u32) -> bool) -> Option<Vec2> {
        self.entity_ids.iter().filter(|&&x| pred(self,x))
            .fold((INFINITY, None), |(acc_dist, acc_pos), elem| {
                let elem_pos = self.common.get(elem).unwrap().rect.centroid();
                let dist = (pos - elem_pos).magnitude();
                if dist < acc_dist {
                    (dist, Some(elem_pos))
                } else {
                    (acc_dist, acc_pos)
                }
            }
        ).1
    }

    pub fn closest_pos_matching_pred2(&self, pos: Vec2, pred: &dyn Fn(u32) -> bool) -> Option<Vec2> {
        self.entity_ids.iter().filter(|&&x| pred(x))
            .fold((INFINITY, None), |(acc_dist, acc_pos), elem| {
                let elem_pos = self.common.get(elem).unwrap().rect.centroid();
                let dist = (pos - elem_pos).magnitude();
                if dist < acc_dist {
                    (dist, Some(elem_pos))
                } else {
                    (acc_dist, acc_pos)
                }
            }
        ).1
    }

    // gosh you can never work with closures

    pub fn update_ai(&mut self) {
        for (&id, ai_component) in self.ai.iter_mut() {
            match ai_component.kind {
                AIKind::Roamer => {},
                AIKind::Rush => {
                    if let Some(target) = self.closest_pos_matching_pred2(self.pos(id), &|other_id| self.team(id) != self.team(other_id) && self.has_health(other_id)) {

                    }
                },
            }
        }
    }




}

// yea idk if this can work