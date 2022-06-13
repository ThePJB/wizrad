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