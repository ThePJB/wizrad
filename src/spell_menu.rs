use crate::kgui::*;
use crate::kmath::*;
use crate::application::*;
use crate::manifest::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::spell::*;

pub struct SpellMenu {
    selection: i32,
    spells: [Spell; 3],
    t: f32,
}

impl SpellMenu {
    pub fn new(spells: &[Spell; 3]) -> SpellMenu {
        SpellMenu {
            selection: 1,
            spells: *spells,
            t: 0.0,
        }
    }
}

impl Scene for SpellMenu {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);
        let info_pane = inputs.screen_rect.dilate(-0.3).fit_aspect_ratio(3.0);

        self.t += inputs.dt as f32;
        
        buf.draw_rect(info_pane.dilate(0.1), Vec3::new(0.3, 0.3, 0.3), 9.0);
        
        for i in 0..3 {
            let selection_rect = info_pane.grid_child(i, 0, 3, 1);
            if selection_rect.contains(inputs.mouse_pos) {
                self.selection = i;
            }
            if i == self.selection {
                buf.draw_rect(selection_rect, Vec3::new(1.0, 1.0, 0.0), 10.0);
            }
            buf_uv.draw_sprite(selection_rect.dilate(-0.05), spell_sprite(self.spells[i as usize]), 12.0);
        }

        buf.draw_rect(inputs.screen_rect, Vec3::new(0.0, 0.0, 0.0), 1.0);

        if self.t > 0.5 && inputs.events.iter().any(|e| match e {
            KEvent::MouseLeft(false) => true,
            _ => false,
        }) {
            if info_pane.contains(inputs.mouse_pos) {
                return (SceneOutcome::Pop(SceneSignal::SpellChoice(self.spells[self.selection as usize])), buf, Some(buf_uv));
            }
        }
         
        (SceneOutcome::None, buf, Some(buf_uv))
    }
}