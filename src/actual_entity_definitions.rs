use std::f32::consts::PI;

use crate::components::make_entities::*;
use crate::components::spawn_list::SpawnList;
use crate::entity::*;
use crate::kmath::*;
use crate::spell::*;

// new components
// MakeEntitiesOnDeath, fn ptr that takes position and returns vec of entities
// MakeEntitiesPeriodically fn ptr that takes position and returns vec of entities
// SpawnList big vec of entities and times to spawn them
// roam

pub fn portal(spawns: SpawnList) -> Entity {
    let mut portal = Entity::new()
        .with_rect(Rect::new_centered(0.0, 0.0, 1.0, 2.0))
        .with_render_solid(Vec3::new(0.0, 1.0, 0.0));

    portal.spawn_list = Some(spawns);
    portal
}

pub fn portal2(pos: Vec2, team: u32) -> Entity {
    let mut portal = Entity::new()
        .with_rect(Rect::new_centered(0.0, 0.0, 1.0, 2.0))
        .with_render_solid(Vec3::new(0.8, 0.0, 1.0));

    let mut s = SpawnList::builder();
    for i in 0..=40 {
        if i % 2 == 0 {
            s.spawn_entity(magic_missile_caster(pos, team))
        }
        s.spawn_entity(zerg(pos, team));
        s.wait(0.5);
    }
    s.build();

    portal.spawn_list = Some(s);
    portal
}

pub fn goon(pos: Vec2, team: u32) -> Entity {
    Entity::new()
        .with_team(team)
        .with_physics(1.0, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 1.0, 1.0))
        .with_melee_damage(20.0)
        .with_health(50.0, 1.0)
        .with_ai(10.0, 0.0, 4.0, 6.0)
        .with_render_solid(Vec3::new(1.0, 0.0, 0.0))
}

pub fn retalliator(pos: Vec2, team: u32) -> Entity {
    let mut r = Entity::new()
        .with_team(team)
        .with_physics(1.0, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 1.5, 1.5))
        .with_health(60.0, 1.0)
        .with_ai(10.0, 0.0, 3.0, 6.0)
        .with_render_solid(Vec3::new(1.0, 0.5, 0.0));
    
        r.make_on_damage = Some(MakeEntitiesOnDamage {
            acc: 0.0,
            thresh: 20.0,
            f: |wg, inputs, id, buf| {
                let team = wg.team.get(&id).unwrap().team;
                let pos = wg.rect.get(&id).unwrap().centroid();
                let retalliations: Vec<Entity> = (0..8_i32).map(|i| i as f32 * 2.0 * PI / 8.0)
                    .map(|theta| Vec2::new(theta.cos(), theta.sin()))
                    .map(|vhat| retalliator_projectile(pos, id, team, vhat))
                    .collect();
                for e in retalliations {
                    buf.push(e);
                }
            }
        });

        r
}

pub fn zerg(pos: Vec2, team: u32) -> Entity {
    Entity::new()
        .with_team(team)
        .with_physics(0.25, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 0.5, 0.5))
        .with_melee_damage(20.0)
        .with_health(20.0, 1.0)
        .with_ai(10.0, 0.0, 7.0, 6.0)
        .with_render_solid(Vec3::new(0.7, 0.0, 0.0))
}

pub fn magic_missile_caster(pos: Vec2, team: u32) -> Entity {
    Entity::new()
        .with_team(team)
        .with_physics(0.7, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 0.9, 0.9))
        .with_health(20.0, 1.0)
        .with_ai(10.0, 5.0, 3.0, 6.0)
        .with_render_solid(Vec3::new(0.0, 0.8, 0.8))
        .with_ai_caster(9.0, Spell::Missile)
        .with_caster(10.0, 3.0)
}

pub fn retalliator_projectile(pos: Vec2, source: u32, team: u32, vhat: Vec2) -> Entity {
    Entity::new()
    .with_team(team)
    .with_physics(10.0, vhat * 10.0)
    .with_rect(Rect::new_centered(pos.x, pos.y, 0.4, 0.4))
    .with_projectile(source, 20.0)
    .with_render_solid(Vec3::new(1.0, 0.5, 0.0))
}

pub fn barrager(pos: Vec2, team: u32) -> Entity {
    Entity::new()
        .with_team(team)
        .with_physics(2.0, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 1.2, 1.2))
        .with_health(60.0, 1.0)
        .with_ai(20.0, 10.0, 3.0, 6.0)
        .with_render_solid(Vec3::new(0.6, 0.0, 0.8))
        .with_ai_caster_unleasher(15.0, Spell::Barrage)
        .with_caster(50.0, 6.0)
}

pub fn deathspawner(pos: Vec2, team: u32) -> Entity {
    let mut d = Entity::new()
        .with_team(team)
        .with_physics(2.0, Vec2::new(0.0, 0.0))
        .with_rect(Rect::new_centered(pos.x, pos.y, 1.1, 1.1))
        .with_melee_damage(20.0)
        .with_health(60.0, 1.0)
        .with_ai(10.0, 0.0, 3.5, 6.0)
        .with_render_solid(Vec3::new(1.0, 0.4, 0.3));

    d.make_on_death = Some(MakeEntitiesOnDeath {
        f: |wg, inputs, id, buf| {
            let team = wg.team.get(&id).unwrap().team;
            let pos = wg.rect.get(&id).unwrap().centroid();
            let retalliations: Vec<Entity> = (0..3_i32).map(|i| i as f32 * 2.0 * PI / 8.0)
                .map(|theta| zerg(pos.offset_r_theta(0.3, theta), team))
                .collect();
            for e in retalliations {
                buf.push(e);
            }
        }
    });

    d
}

// plaguebearer: also need a dot component with uid, bool stacking
// and periodic summon component