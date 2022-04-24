use crate::kmath::*;
use crate::wave_game::Spell;

pub enum AIKind {
    Roamer,
    Rush,
}

// AI movt -> pursue player
pub struct AI {
    pub kind: AIKind,
    pub target_location: Vec2,
    pub last_update: f64,
    pub update_interval: f64,
}

pub struct AICaster {
    pub spell: Spell,
    pub acquisition_range: f32,
}