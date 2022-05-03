use crate::kgui::*;
use crate::kmath::*;
use crate::application::*;
use crate::manifest::*;
use crate::wave_game::*;
use crate::renderer::*;
use crate::rendererUV::*;

pub struct Victory {
    pub t: f32,
}

impl Scene for Victory {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);
        let info_pane = inputs.screen_rect.dilate(-0.3).fit_center_square();

        self.t += inputs.dt as f32;
        
        buf.draw_rect(inputs.screen_rect, Vec3::new(0.0, 0.0, 0.0), 1.0);
        buf_uv.draw_sprite(info_pane, WINNER, 2.0);

        if self.t > 0.5 && inputs.events.iter().any(|e| match e {
            KEvent::MouseLeft(false) => true,
            KEvent::Keyboard(_, false) => true,
            _ => false,
        }) {
            return (SceneOutcome::Push(Box::new(WaveGame::new())), buf, Some(buf_uv));
        }
         
        (SceneOutcome::None, buf, Some(buf_uv))
    }
}