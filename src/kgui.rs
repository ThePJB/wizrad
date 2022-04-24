use crate::kmath::*;

use std::collections::HashSet;
use std::time::{Instant, Duration};

use glutin::event::VirtualKeyCode;

use glutin::event::ElementState;
use glutin::event::MouseButton;
use glutin::event::Event;
use glutin::event::WindowEvent::KeyboardInput;
use glutin::event::WindowEvent::MouseInput;
use glutin::event::WindowEvent::CursorMoved;
use glutin::event::WindowEvent::Resized;

// Do I dare???
#[derive(Clone, Copy)]
pub enum KEvent {
    Keyboard(VirtualKeyCode, bool),
    MouseLeft(bool),
    MouseRight(bool),
    MouseMiddle(bool),
    MouseMotion(Vec2),
}

#[derive(Clone)]
pub struct FrameInputState {
    pub screen_rect: Rect,
    pub mouse_pos: Vec2,
    pub held_keys: HashSet<VirtualKeyCode>,
    pub held_lmb: bool,
    pub held_rmb: bool,
    pub held_mmb: bool,
    pub events: Vec<KEvent>,
    pub t: f64,
    pub dt: f64,
    pub frame: u32,
}

pub struct EventAggregator {
    xres: f32,
    yres: f32,
    t_last: Instant,
    current: FrameInputState,
}

impl EventAggregator {
    pub fn new(xres: f32, yres: f32) -> EventAggregator {
        EventAggregator { 
            xres, 
            yres, 
            t_last: Instant::now(),
            current: FrameInputState { 
                screen_rect: Rect::new(0.0, 0.0, xres/yres, 1.0, ), 
                mouse_pos: Vec2::new(0.0, 0.0), 
                held_keys: HashSet::new(),
                held_lmb: false, 
                held_rmb: false, 
                held_mmb: false, 
                events: Vec::new(),
                t: 0.0,
                dt: 0.0,
                frame: 0,
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) -> Option<FrameInputState> {
        match event {
            Event::WindowEvent {event, ..} => match event {
                KeyboardInput { 
                    input: glutin::event::KeyboardInput { 
                        virtual_keycode: Some(virtual_code), 
                        state, 
                    ..},
                ..} => {
                    self.current.events.push(KEvent::Keyboard(*virtual_code, *state == ElementState::Pressed));
                    if *state == ElementState::Pressed {
                        self.current.held_keys.insert(*virtual_code);
                    } else {
                        self.current.held_keys.remove(&virtual_code);
                    }
                },


                MouseInput { button: glutin::event::MouseButton::Left, state, ..} => {
                    self.current.events.push(KEvent::MouseLeft(*state == ElementState::Pressed));
                    if *state == ElementState::Pressed {
                        self.current.held_lmb = true;
                    } else {
                        self.current.held_lmb = false;
                    }
                },
                MouseInput { button: glutin::event::MouseButton::Middle, state, ..} => {
                    self.current.events.push(KEvent::MouseMiddle(*state == ElementState::Pressed));
                    if *state == ElementState::Pressed {
                        self.current.held_mmb = true;
                    } else {
                        self.current.held_mmb = false;
                    }
                },
                MouseInput { button: glutin::event::MouseButton::Right, state, ..} => {
                    self.current.events.push(KEvent::MouseRight(*state == ElementState::Pressed));
                    if *state == ElementState::Pressed {
                        self.current.held_rmb = true;
                    } else {
                        self.current.held_rmb = false;
                    }
                },


                // Mouse motion
                CursorMoved {
                    position: pos,
                    ..
                } => {
                    let old_cursor_pos = self.current.mouse_pos;
                    let new_cursor_pos = Vec2::new(pos.x as f32 / self.yres, pos.y as f32 / self.yres);
                    self.current.events.push(KEvent::MouseMotion(new_cursor_pos - old_cursor_pos));
                    self.current.mouse_pos = new_cursor_pos;
                },

                // Resize
                Resized(physical_size) => {
                    self.xres = physical_size.width as f32;
                    self.yres = physical_size.height as f32;
                    self.current.screen_rect = Rect::new(0.0, 0.0, self.xres / self.yres, 1.0);
                },


                // (resize and quit need to be handled by the application)
                _ => {},
                
            },
            Event::MainEventsCleared => {
                let t_now = Instant::now();
                let dt = t_now.duration_since(self.t_last).as_secs_f64();
                self.current.dt = dt;
                self.current.t += dt;
                self.t_last = t_now;
                self.current.frame += 1;
                let state = self.current.clone();
                self.current.events = Vec::new();
                return Some(state);
            },
            _ => {},
        }

        None
    }
}