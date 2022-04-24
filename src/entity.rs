pub struct Entity {
    ai: Option<AI>,
    caster: Option<Caster>,
    emitter: Option<Emitter>,
    common: Option<Common>,
    health: Option<Health>,
    melee_damage: Option<MeleeDamage>,
    projectile: Option<Projectile>,
    render: Option<Render>,
    timed_life: Option<TimedLife>,
}

impl Entity {
    pub fn new() -> Entity {
        
    }
}