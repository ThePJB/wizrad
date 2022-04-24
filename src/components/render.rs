use crate::kmath::*;
use crate::renderer::TriangleBuffer;
use crate::wave_game::*;
pub enum Render {
    Colour(Vec3),
    FOfT(FOfT),
    FireSplat(f32),
}

pub struct FOfT {
    pub f: fn(f32) -> Vec3,
    pub t_start: f32,
    pub t_end: f32,
}

impl WaveGame {
    pub fn draw_entities(&self, buf: &mut TriangleBuffer, t: f32, frame: u32) {
        for (id, er) in self.render.iter() {
            let ec = self.common.get(id).unwrap();
            match er {
                Render::Colour(colour) => {
                    // if iframe
                    if let Some(health) = self.health.get(id) {
                        if t as f32 - health.invul_time < 0.25 {
                            buf.draw_rect(ec.rect, Vec3::new(1.0, 1.0, 1.0), 3.0)
                        } else {
                            buf.draw_rect(ec.rect, *colour, 3.0)
                        }
                    }
                    buf.draw_rect(ec.rect, *colour, 3.0)
                },
                Render::FOfT(f_of_t) => {
                    let t = unlerp(t as f32, f_of_t.t_start, f_of_t.t_end);
                    let c = (f_of_t.f)(t);
                    buf.draw_rect(ec.rect, c, 3.0);
                },
                Render::FireSplat(r) => {
                    let mut seed = frame * 123171717 + id * 123553;
                    let pos = ec.rect.centroid();
                    let mut draw_rect = |w, h, c, d| buf.draw_rect(Rect::new_centered(pos.x, pos.y, w, h), c, d);
                    draw_rect(kuniform(seed, r/4.0, *r), r - kuniform(seed, r/4.0, *r), Vec3::new(1.0, 0.0, 0.0), 50.0);
                    seed *= 1711457123;
                    draw_rect(kuniform(seed, r/4.0, *r), r - kuniform(seed, r/4.0, *r), Vec3::new(1.0, 0.0, 0.0), 50.0);
                    seed *= 1711457123;
                    draw_rect(kuniform(seed, r/4.0, *r), r - kuniform(seed, r/4.0, *r), Vec3::new(1.0, 0.0, 0.0), 50.0);
                    seed *= 1711457123;
                    draw_rect(kuniform(seed, r/8.0, *r/2.0), r - kuniform(seed, r/8.0, *r/2.0), Vec3::new(1.0, 1.0, 0.0), 60.0);
                    seed *= 1711457123;
                    draw_rect(kuniform(seed, r/8.0, *r/2.0), r - kuniform(seed, r/8.0, *r/2.0), Vec3::new(1.0, 1.0, 0.0), 60.0);
                    seed *= 1711457123;
                    draw_rect(kuniform(seed, r/8.0, *r/2.0), r - kuniform(seed, r/8.0, *r/2.0), Vec3::new(1.0, 1.0, 0.0), 60.0);
                },
            }
        }        
    }

}