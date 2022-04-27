use crate::kmath::*;
use crate::wave_game::*;
use ordered_float::*;
use std::f32::INFINITY;
use core::f32::consts::PI;


// AI movt -> pursue player
pub struct AI {
    pub dir: Vec2,
    pub acquisition_range: f32,
    pub flee_range: f32,
    pub speed: f32,
}

pub struct AICaster {
    pub spell: Spell,
    pub acquisition_range: f32,
}

impl WaveGame {
    pub fn update_movement_ai(&mut self, t: f32, dt: f32, frame: u32, level_rect: Rect){
        for (id, ai) in self.ai.iter_mut() {
            let my_team = self.team.get(id).unwrap().team;
            let my_phys = self.physics.get(id).unwrap();
            let my_pos = my_phys.pos();

            let target = self.entity_ids.iter()
                .filter(|id| self.team.contains_key(id) && self.team.get(id).unwrap().team != my_team)
                .filter(|id| !self.projectile.contains_key(id))
                .filter(|id| self.physics.contains_key(id))
                .map(|id| self.physics.get(id).unwrap().pos())
                .filter(|&pos| my_pos.dist(pos) < ai.acquisition_range)
                .min_by_key(|&pos| OrderedFloat(my_pos.dist(pos)));
            
            if let Some(pos) = target {
                ai.dir = (pos - my_pos).normalize();
                let dist = pos.dist(my_pos);
                if dist < ai.flee_range {
                    ai.dir = -ai.dir;
                } 
                let speed = ai.speed.min(dist/dt as f32); // potential bug butshould be fine
                let mut_phys = self.physics.get_mut(id).unwrap();
                mut_phys.velocity = speed * ai.dir;
            } else {
                let seed = frame * 123123 + id * 17236;
                ai.dir = (ai.dir +  dt * 0.02 * Vec2::new(kuniform(seed, -1.0, 1.0), kuniform(seed+13131313, -1.0, 1.0)).normalize()).normalize();
                if !level_rect.contains(my_pos + Vec2::new(ai.dir.x, 0.0).normalize() * 1.0) {
                    ai.dir.x = -ai.dir.x;
                }
                if !level_rect.contains(my_pos + Vec2::new(0.0, ai.dir.y).normalize() * 1.0) {
                    ai.dir.y = -ai.dir.y;
                }
                let mut_phys = self.physics.get_mut(id).unwrap();
                mut_phys.velocity = 0.25 * ai.speed * ai.dir;
            }
        }            
    }

    pub fn update_casting_ai(&mut self, t: f32, commands: &mut Vec<Command>) {
        for (id, aic) in self.ai_caster.iter() {
            let my_pos = self.physics.get(id).unwrap().pos();
            let my_team = self.team.get(id).unwrap().team;

            let target = self.entity_ids.iter()
                .filter(|id| self.team.contains_key(id) && self.team.get(id).unwrap().team != my_team)
                .filter(|id| !self.projectile.contains_key(id))
                .filter(|id| self.physics.contains_key(id))
                .map(|id| self.physics.get(id).unwrap().pos())
                .filter(|&pos| my_pos.dist(pos) < aic.acquisition_range)
                .min_by_key(|&pos| OrderedFloat(my_pos.dist(pos)));
            
            if let Some(pos) = target {
                commands.push(Command::Cast(*id, pos, aic.spell, false));
            }
        }
    }
}