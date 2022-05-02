use crate::kgui::*;
use crate::kmath::*;
use crate::application::*;
use crate::manifest::*;
use crate::renderer::*;
use crate::rendererUV::*;

pub struct Tutorial {

}

impl Scene for Tutorial {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);
        let info_pane = inputs.screen_rect.dilate(-0.3).fit_aspect_ratio(2.0 / 3.0);
        
        buf.draw_rect(inputs.screen_rect, Vec3::new(0.0, 0.0, 0.0), 1.0);
        buf_uv.draw_sprite(info_pane.grid_child(0, 0, 2, 3), TUT_WASD, 2.0);
        buf_uv.draw_sprite(info_pane.grid_child(1, 0, 2, 3), TUT_DODGE, 2.0);
        buf_uv.draw_sprite(info_pane.grid_child(0, 1, 2, 3), TUT_QE, 2.0);
        buf_uv.draw_sprite(info_pane.grid_child(1, 1, 2, 3), TUT_BROWSE, 2.0);
        buf_uv.draw_sprite(info_pane.grid_child(0, 2, 2, 3), TUT_LMB, 2.0);
        buf_uv.draw_sprite(info_pane.grid_child(1, 2, 2, 3), TUT_CAST, 2.0);

        if inputs.events.iter().any(|e| match e {
            KEvent::Keyboard(_, false) => true,
            KEvent::MouseLeft(false) => true,
            _ => false,
        }) {
            return (SceneOutcome::Pop(SceneSignal::JustPop), buf, Some(buf_uv));
        }
         
        (SceneOutcome::None, buf, Some(buf_uv))
    }
}