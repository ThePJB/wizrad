use std::collections::HashMap;
use crate::kgui::*;
use crate::kmath::*;
use crate::entity::*;

pub struct Spawner {
    spawner_t: f64,
    entity_counter: u32,
    entities: HashMap<u32, Entity>,
    intervals: HashMap<u32, f32>,
    // interval min times etc
    next_spawns: HashMap<u32, f32>,
}

pub fn enemy_position(seed: u32) -> Vec2 {
    let level_min = -19.5;
    let level_max = 19.5;

    match khash(seed * 123415) % 4 {
        0 => Vec2::new(level_min, kuniform(seed * 138971377, level_min, level_max)),
        1 => Vec2::new(level_max, kuniform(seed * 138971377, level_min, level_max)),
        2 => Vec2::new(kuniform(seed * 138971377, level_min, level_max), level_min),
        3 => Vec2::new(kuniform(seed * 138971377, level_min, level_max), level_max),
        _ => panic!("unreachable"),
    }
}

impl Spawner {
    pub fn new() -> Spawner {
        Spawner {
            spawner_t: 0.0,
            entity_counter: 0,
            entities: HashMap::new(),
            intervals: HashMap::new(),
            next_spawns: HashMap::new(),
        }
    }

    pub fn add_spawn_entity(&mut self, entity: Entity, interval: f32) {
        self.entities.insert(self.entity_counter, entity);
        self.intervals.insert(self.entity_counter, interval);
        self.next_spawns.insert(self.entity_counter, interval);

        self.entity_counter += 1;
    }

    pub fn frame(&mut self, inputs: &FrameInputState) -> Option<Entity> {
        self.spawner_t += inputs.dt;
        for (id, next_spawn) in self.next_spawns.iter_mut() {
            if self.spawner_t as f32 > *next_spawn {
                *next_spawn += self.intervals.get(id).unwrap();
                return Some(self.entities.get(id).unwrap().clone().with_position(enemy_position(inputs.seed)));
            }
        }
        None
    }
}