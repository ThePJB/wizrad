use crate::kgui::*;
use crate::kmath::*;
use crate::application::*;
use crate::manifest::*;
use crate::wave_game::*;
use crate::renderer::*;
use crate::rendererUV::*;

pub struct Victory {

}

impl Scene for Victory {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);
        let info_pane = inputs.screen_rect.dilate(-0.3).fit_center_square();
        
        buf.draw_rect(inputs.screen_rect, Vec3::new(0.0, 0.0, 0.0), 1.0);
        buf_uv.draw_sprite(info_pane, WINNER, 2.0);

        if inputs.events.iter().any(|e| match e {
            KEvent::Keyboard(_, true) => true,
            KEvent::Keyboard(_, false) => false,
            KEvent::MouseLeft(_) => true,
            KEvent::MouseMiddle(_) => true,
            KEvent::MouseRight(_) => true,
            KEvent::MouseMotion(_) => false,
        }) {
            return (SceneOutcome::Push(Box::new(WaveGame::new())), buf, Some(buf_uv));
        }
         
        (SceneOutcome::None, buf, Some(buf_uv))
    }
}