use glow::*;
use crate::kgui::EventAggregator;
use crate::kgui::FrameInputState;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::kimg::*;
use crate::wave_game::*;
use crate::tutorial::*;
use crate::victory::*;
use crate::spell::*;
use glutin::event::{Event, WindowEvent};

pub enum SceneOutcome {
    Push(Box<dyn Scene>),
    Pop(SceneSignal),
    QuitProgram,
    None,
}

pub enum SceneSignal {
    SpellChoice(Spell),
    JustPop,
}

pub trait Scene {
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome;
    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>);
}

pub struct Application {
    gl: glow::Context,
    window: glutin::WindowedContext<glutin::PossiblyCurrent>,

    renderer: Renderer,
    rendererUV: RendererUV,
    event_aggregator: EventAggregator,

    pub xres: f32,
    pub yres: f32,
    
    scene_stack: Vec<Box<dyn Scene>>,
}

pub fn load_file(paths: &[&str]) -> String {
    for path in paths {
        if let Ok(s) = std::fs::read_to_string(path) {
            return s
        }
    }
    panic!("couldn't find any of {:?}", paths)
}

impl Application {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Application {
        let default_xres = 1600.0;
        let default_yres = 900.0;

        let (gl, window) = unsafe { opengl_boilerplate(default_xres, default_yres, event_loop) };

        let bv = &[
            "src/basic.vert",
            "../../src/basic.vert",
            "basic.vert",
        ];
        let bf = &[
            "src/basic.frag",
            "../../src/basic.frag",
            "basic.frag",
        ];
        
        let uvv = &[
            "src/uv.vert",
            "../../src/uv.vert",
            "uv.vert",
        ];
        let uvf = &[
            "src/uv.frag",
            "../../src/uv.frag",
            "uv.frag",
        ];
                
        let basic_shader = make_shader(&gl, bv, bf);
        let uv_shader = make_shader(&gl, uvv, uvf);

        let atlas = ImageBufferA::new_from_file("src/atlas.png")
            .or(ImageBufferA::new_from_file("../../src/atlas.png")
            .or(ImageBufferA::new_from_file("atlas.png")))
            .expect("couldn't load atlas from ./atlas.png");

        let renderer = Renderer::new(&gl, basic_shader);
        let rendererUV = RendererUV::new(&gl, uv_shader, atlas);

        let mut scene_stack: Vec<Box<dyn Scene>> = Vec::new();
        scene_stack.push(Box::new(Victory{t: 0.0}));
        scene_stack.push(Box::new(WaveGame::new()));
        scene_stack.push(Box::new(Tutorial{}));

        Application {
            gl,
            window,
            renderer,
            rendererUV,
            event_aggregator: EventAggregator::new(default_xres, default_yres),

            xres: default_xres,
            yres: default_yres,

            scene_stack,
        }
    }

    

    pub fn handle_scene_outcome(&mut self, so: SceneOutcome) {
        match so {
            SceneOutcome::Push(scene) => {
                self.scene_stack.push(scene);
            },
            SceneOutcome::Pop(signal) => {
                self.scene_stack.pop();
                if self.scene_stack.len() == 0 {
                    self.destroy();
                    std::process::exit(0);
                } else {
                    let stack_idx = self.scene_stack.len() - 1;
                    let so = self.scene_stack[stack_idx].handle_signal(signal);
                    self.handle_scene_outcome(so);
                }
            },
            SceneOutcome::QuitProgram => {
                self.destroy();
                std::process::exit(0);
            }
            SceneOutcome::None => {},
        }
    }

    pub fn handle_event(&mut self, event: &glutin::event::Event<()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    self.window.resize(*physical_size);
                    self.xres = physical_size.width as f32;
                    self.yres = physical_size.height as f32;
                    unsafe {self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32)};
                },
                _ => {},
            _ => {},
            }
            _ => {},
        }

        if let Some(inputs) = self.event_aggregator.handle_event(event) {
            let stack_idx = self.scene_stack.len()-1; 
            
            unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); } 

            let (so, tris, op_triuvs) = self.scene_stack[stack_idx].frame(inputs);
            self.renderer.present(&self.gl, tris);
            if let Some(tri_uvs) = op_triuvs {
                self.rendererUV.present(&self.gl, tri_uvs);
            }
            self.window.swap_buffers().unwrap();

            self.handle_scene_outcome(so);
        }
    }

    pub fn destroy(&mut self) {
        self.renderer.destroy(&self.gl);
    }
}

fn  make_shader(gl: &glow::Context, vert_paths: &[&str], frag_paths: &[&str]) -> glow::Program {
    unsafe {
        let program = gl.create_program().expect("Cannot create program");
        let shader_version = "#version 410";
        let shader_sources = [
            (glow::VERTEX_SHADER, load_file(vert_paths)),
            (glow::FRAGMENT_SHADER, load_file(frag_paths)),
            ];
        let mut shaders = Vec::with_capacity(shader_sources.len());
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        
        program
    }
}

unsafe fn opengl_boilerplate(xres: f32, yres: f32, event_loop: &glutin::event_loop::EventLoop<()>) -> (glow::Context, glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("wizrad")
        .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
    let window = glutin::ContextBuilder::new()
        // .with_depth_buffer(0)
        // .with_srgb(true)
        // .with_stencil_buffer(0)
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();


    let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    gl.enable(DEPTH_TEST);
    // gl.enable(CULL_FACE);
    gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    gl.enable(BLEND);
    gl.debug_message_callback(|a, b, c, d, msg| {
        println!("{} {} {} {} msg: {}", a, b, c, d, msg);
    });

    (gl, window)
}