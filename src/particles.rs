use crate::kmath::*;
use crate::renderer::*;

pub struct Particle {
    pub expiry: f32,
    pub velocity: Vec2,
    pub rect: Rect,
    pub colour: Vec3,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn add_particle(&mut self, p: Particle) {
        self.particles.push(p);
    }

    pub fn update(&mut self, t: f32, dt: f32) {
        self.particles.retain(|p| p.expiry > t);

        for particle in self.particles.iter_mut() {
            particle.rect.x += particle.velocity.x * dt;
            particle.rect.y += particle.velocity.y * dt;
        }
    }

    pub fn draw(&self, buf: &mut TriangleBuffer) {
        for particle in self.particles.iter() {
            buf.draw_rect(particle.rect, particle.colour, 100.0);
        }
    }
}