use crate::components::physics::CollisionEvent;
use crate::wave_game::*;
use crate::manifest::*;

#[derive(Clone)]
pub struct MeleeDamage {
    pub amount: f32,
}

impl WaveGame {
    pub fn resolve_melee_damage(&mut self, collisions: &[CollisionEvent], t: f32) {
        for ce in collisions {
            if let Some(md) = self.melee_damage.get(&ce.subject) {
                if self.team.contains_key(&ce.subject) && self.team.contains_key(&ce.object)
                        && self.team.get(&ce.subject).unwrap().team != self.team.get(&ce.object).unwrap().team {
                    if let Some(obj_health) = self.health.get_mut(&ce.object) {
                        obj_health.damage(md.amount, t);
                    }
                }
            }
        }
    }
}