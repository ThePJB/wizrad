use crate::kmath::*;

pub enum AIKind {
    Roamer,
    Rush,
}

pub struct AI {
    pub kind: AIKind,
    pub target_location: Vec2,
    pub last_update: f64,
    pub update_interval: f64,
}