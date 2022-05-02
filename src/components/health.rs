use crate::manifest::*;

#[derive(Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub regen: f32,
    pub invul_time: f32,
}

impl Health {
    pub fn damage(&mut self, amount: f32, t: f32) {
        if t - self.invul_time > INVUL_TIME {
            if amount > INVUL_DAMAGE_THRESHOLD {
                self.invul_time = t;
            }
            self.current -= amount;
        }
    }
}