use crate::collision_system::CollisionEvent;
use crate::wave_game::*;
pub struct MeleeDamage {
    pub amount: f32,
}

impl WaveGame {
    pub fn resolve_melee_damage(&mut self, collisions: &[CollisionEvent], t: f32) {
        for ce in collisions {
            if let Some(md) = self.melee_damage.get(&ce.subject) {
                let subj_com = self.common.get(&ce.subject).unwrap();
                let obj_com = self.common.get(&ce.object).unwrap();
    
                if subj_com.team != obj_com.team {
                    if let Some(obj_health) = self.health.get_mut(&ce.object) {
                        if t - obj_health.invul_time > 0.25 {
                            obj_health.invul_time = t;
                            obj_health.current -= md.amount;
                        }
                    }
                }
            }
        }
    }
}