use crate::kmath::*;

pub struct Emitter {
    pub interval: f32,
    pub last: f32,
    pub colour: Vec3,
    pub size: Vec2,
    pub speed: f32,
    pub lifespan: f32,
}