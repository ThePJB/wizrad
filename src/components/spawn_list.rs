use crate::actual_entity_definitions::*;
use crate::entity::*;
use crate::kmath::*;

#[derive(Clone)]
pub struct SpawnList {
    pub t: f32,
    pub list: Vec<(f32, Box<Entity>)>,
}

impl SpawnList {
    pub fn builder() -> SpawnList {
        SpawnList {
            t: 0.0,
            list: Vec::new(),
        }
    }

    pub fn spawn_entity(&mut self, e: Entity) {
        self.list.push((self.t, Box::new(e)));
    }

    pub fn wait(&mut self, amount: f32) {
        self.t += amount;
    }

    pub fn build(&mut self) {
        self.t = 0.0;
    }
}

// spawn list: what if we just had spawn wait, spawn wait - i like this, just recipe mode
// vs. spawn at time, spawn at time 

impl SpawnList {
    pub fn wave1(pos: Vec2, team: u32) -> SpawnList {
        let mut list = Vec::new();
        let mut t = 0.0;
        for i in 0..=40 {
            list.push((t, Box::new(zerg(pos, team))));
            t += 0.5;
            
            if i % 2 == 0 {
                list.push((t + 0.25, Box::new(deathspawner(pos, team))));
            }
            if i % 10 == 0 {
                list.push((t + 0.25, Box::new(barrager(pos, team))));
            }
            
            if i == 40 {
                list.push((t, Box::new(goon(pos, team))));
                list.push((t, Box::new(goon(pos, team))));
                list.push((t, Box::new(goon(pos, team))));
                list.push((t, Box::new(zerg(pos, team))));
                list.push((t, Box::new(zerg(pos, team))));
                list.push((t, Box::new(zerg(pos, team))));
                list.push((t, Box::new(zerg(pos, team))));
                list.push((t, Box::new(zerg(pos, team))));
            }
        }

        SpawnList {t: 0.0, list}
    }
}