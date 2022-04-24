use crate::kmath::*;
use crate::wave_game::*;
use ordered_float::*;
use std::f32::INFINITY;

pub enum AIKind {
    Roamer,
    Rush,
}

// AI movt -> pursue player
pub struct AI {
    pub kind: AIKind,
    pub target_location: Vec2,
    pub last_update: f32,
    pub update_interval: f32,
}

pub struct AICaster {
    pub spell: Spell,
    pub acquisition_range: f32,
}

impl WaveGame {
    pub fn update_movement_ai(&mut self, t: f32, dt: f32, frame: u32){
        for (id, ai) in self.ai.iter_mut() {
            let aic = self.common.get(id).unwrap();

            match ai.kind {
                AIKind::Roamer => {
                    if t - ai.last_update > ai.update_interval {
                        ai.last_update += ai.update_interval;
                        let seed = frame * 2351352729 + id * 423476581;

                        let target = self.common.iter()
                            .filter(|(id, com)| com.team != aic.team)
                            .map(|x| x.1.rect.centroid())
                            .filter(|&x| aic.rect.centroid().dist(x) < 5.0)
                            .min_by_key(|&x| OrderedFloat(aic.rect.centroid().dist(x)));

                        if let Some(pos) = target {
                            ai.target_location = pos;
                        } else {
                            ai.target_location = aic.rect.centroid() + Vec2::new(krand(seed) - 0.5, krand(seed + 1) - 0.5).normalize() * 2.0;
                        }
                    }
                    
                },
                AIKind::Rush => {
                    let target = self.common.iter()
                        .filter(|(id, com)| com.team != aic.team)
                        .map(|x| x.1.rect.centroid())
                        .filter(|&x| aic.rect.centroid().dist(x) < 10.0)
                        .min_by_key(|&x| OrderedFloat(aic.rect.centroid().dist(x)));
                    
                    if let Some(pos) = target {
                        ai.target_location = pos;
                    }
                },
            }

            let dist = ai.target_location.dist(aic.rect.centroid());
            let speed = aic.speed.min(dist/dt as f32);
            let aic = self.common.get_mut(id).unwrap();
            aic.velocity = speed * (ai.target_location - aic.rect.centroid()).normalize();
        }            
    }

    pub fn update_casting_ai(&mut self, t: f32, commands: &mut Vec<Command>) {
        for (id, ai_caster) in self.ai_caster.iter() {
            let (self_pos, self_team) = {
                let self_com = self.common.get(id).unwrap();
                (self_com.rect.centroid(), self_com.team)
            };
            let target = self.common.iter()
                .filter(|(id, c)| c.team != self_team && (c.rect.centroid() - self_pos).magnitude() < ai_caster.acquisition_range)
                .fold((INFINITY, None), |acc, e| {
                    let d = (self_pos - e.1.rect.centroid()).magnitude();
                    if d < acc.0 {
                        (d, Some(e.1.rect.centroid()))
                    } else {
                        acc
                    }
                }).1;
            if let Some(pos) = target {
                commands.push(Command::Cast(*id, pos, ai_caster.spell, false));
            }
        }
    }
}