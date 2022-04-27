pub struct Entity {
    ai: Option<AI>,
    caster: Option<Caster>,
    emitter: Option<Emitter>,
    common: Option<Common>,
    health: Option<Health>,
    melee_damage: Option<MeleeDamage>,
    projectile: Option<Projectile>,
    render: Option<Render>,
    expiry: Option<Expiry>,
}

// there could be such a ncie builder

impl Entity {
    pub fn new() -> Entity {
        
    }
}