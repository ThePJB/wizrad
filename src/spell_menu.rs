use crate::spell::*;
use crate::kmath::*;
use crate::kgui::*;
use crate::renderer::*;
use crate::rendererUV::*;

pub struct SpellMenu {
    click_after: f32,
    choices: [Spell; 3],
}

fn shuffle_vec(vec: &mut Vec<Spell>, mut seed: u32) {
    for i in 0..vec.len() {
        seed = khash(seed);
        let swap_idx = i + (seed % (vec.len() - i) as u32) as usize;
        vec.swap(i, swap_idx);
    }
}

impl SpellMenu {
    pub fn new(seed: u32, current_spells: &[Spell], t: f32) -> SpellMenu {
        let mut spell_list = vec![
            Spell::Missile,
            Spell::Fireball,
            Spell::ConeFlames,
            Spell::Pulse,
            Spell::Firestorm,
            Spell::Lifesteal,
            Spell::SummonBloodcasters,
            Spell::SummonRushers,
            Spell::Homing,
            Spell::Healing,
        ];

        spell_list.retain(|s| !current_spells.contains(s));
        shuffle_vec(&mut spell_list, seed);
        
        SpellMenu { 
            choices: spell_list[0..3].try_into().unwrap(),
            click_after: t + 0.3,
        }

    }

    pub fn frame(&self, inputs: &FrameInputState, buf: &mut TriangleBuffer, buf_uv:  &mut TriangleBufferUV) -> Option<Spell> {

        let spell_menu = inputs.screen_rect.dilate(-0.35).fit_aspect_ratio(3.0);//.translate(Vec2::new(0.0, 0.2));
        // buf.draw_rect(spell_menu, Vec3::new(0.2, 0.2, 0.2), 15.0);
        for i in 0..3 {
            let btn_rect = spell_menu.grid_child(i, 0, 3, 1).dilate(-0.01);
            buf.draw_rect(btn_rect, Vec3::new(0.1, 0.1, 0.1), 25.0);
            buf.draw_rect(btn_rect.dilate(-0.01), Vec3::new(0.3, 0.3, 0.3), 25.5);
            let spell_rect = btn_rect.dilate(-0.01);
            if spell_rect.contains(inputs.mouse_pos) {
                buf.draw_rect(spell_rect, Vec3::new(1.0, 1.0, 1.0), 26.0);
                if inputs.t as f32 > self.click_after && inputs.events.iter().any(|event| match event {
                    KEvent::MouseLeft(false) => true,
                    _ => false,
                }) {
                    return Some(self.choices[i as usize]);
                }
            }
            buf_uv.draw_sprite(spell_rect, spell_sprite(self.choices[i as usize]), 27.0);
        }
        None
    }
}