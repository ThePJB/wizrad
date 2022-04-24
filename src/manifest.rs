use crate::kmath::*;

pub const COLOURS: [Vec3; 7] = [
    Vec3::new(0.8, 0.1, 0.1),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.1, 0.1, 0.1),
    Vec3::new(1.0, 1.0, 1.0),
];

pub const PLAYER: i32 = 0;
pub const DEAD_PLAYER: i32 = 3;
pub const POWERUP: i32 = 1;
pub const GOAL: i32 = 2;
pub const PLAY: i32 = 16;
pub const OPEN: i32 = 17;
pub const SAVE: i32 = 18;
pub const PLUS_H: i32 = 19;
pub const MINUS_H: i32 = 20;
pub const PLUS_W: i32 = 21;
pub const MINUS_W: i32 = 22;
pub const PLUS_TAPE: i32 = 23;
pub const MINUS_TAPE: i32 = 24;

pub const TILE_EDGES: i32 = 32;
pub const TILE_CRACKS1: i32 = 33;
pub const TILE_CRACKS2: i32 = 34;
pub const TILE_CRACKS3: i32 = 35;
pub const CHECKERBOARD: i32 = 36;

pub const ATLAS_W: i32 = 16;
pub const ATLAS_H: i32 = 16;

pub const TEAM_PLAYER: u32 = 0;
pub const TEAM_ENEMIES: u32 = 1;
pub const TEAM_NEUTRAL: u32 = 2;