#[derive(Clone)]
pub struct Projectile {
    pub source: u32,
    pub damage: f32,
    pub aoe: f32,
    pub splat_duration: f32,
    pub lifesteal_percent: f32,
}