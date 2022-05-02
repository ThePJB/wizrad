mod renderer;
mod rendererUV;
mod kimg;
mod kmath;
mod application;
mod manifest;
mod kgui;
mod components;
mod particles;
mod entity_definitions;
mod entity;
mod spell;

mod wave_game;
mod spell_menu;
mod tutorial;
mod victory;

use application::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::env;




fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&event_loop);
    
    event_loop.run(move |event, _, control_flow| {
        application.handle_event(&event);
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..}
            => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });
}