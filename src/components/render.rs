use crate::kmath::*;

pub enum Render {
    Colour(Vec3),
    FOfT(FOfT),
}

pub struct FOfT {
    pub f: fn(f32) -> Vec3,
    pub t_start: f32,
    pub t_end: f32,
}