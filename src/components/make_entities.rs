use crate::kgui::FrameInputState;
use crate::kmath::*;
use crate::entity::*;
use crate::wave_game::*;
// MakeEntitiesOnDamage, fn ptr that takes like amount of damage, position, returns a vec of entities

#[derive(Clone)]
pub struct MakeEntitiesOnDamage {
    pub acc: f32,
    pub thresh: f32,
    pub f: fn(wg: &mut WaveGame, inputs: &FrameInputState, id: u32, new_entities: &mut Vec<Entity>),
}

#[derive(Clone)]
pub struct MakeEntitiesOnDeath {
    pub f: fn(wg: &mut WaveGame, inputs: &FrameInputState, id: u32, new_entities: &mut Vec<Entity>),
}