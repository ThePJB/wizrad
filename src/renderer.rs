use crate::kmath::*;
use glow::*;
use std::fmt;

pub struct TriangleBuffer {
    pub screen_rect: Rect,
    pub tris: Vec<Triangle3>,
}

impl TriangleBuffer {
    pub fn new(screen_rect: Rect) -> TriangleBuffer {
        TriangleBuffer { screen_rect, tris: Vec::new() }
    }

    fn push_triangle(&mut self, mut tri: Triangle3) {
        tri.a.pos.x  = (tri.a.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.a.pos.y  = (tri.a.pos.y - self.screen_rect.y) / self.screen_rect.h;

        tri.b.pos.x  = (tri.b.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.b.pos.y  = (tri.b.pos.y - self.screen_rect.y) / self.screen_rect.h;

        tri.c.pos.x  = (tri.c.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.c.pos.y  = (tri.c.pos.y - self.screen_rect.y) / self.screen_rect.h;

        self.tris.push(tri);
    }

    pub fn draw_rect(&mut self, r: Rect, colour: Vec3, depth: f32) {
        let v1 = Vert3 {
            pos: Vec3::new(r.x, r.y, depth),
            colour,
        };
        let v2 = Vert3 {
            pos: Vec3::new(r.x, r.y + r.h, depth),
            colour,
        };
        let v3 = Vert3 {
            pos: Vec3::new(r.x + r.w, r.y + r.h, depth),
            colour,
        };
        let v4 = Vert3 {
            pos: Vec3::new(r.x + r.w, r.y, depth),
            colour,
        };
        self.push_triangle(Triangle3{ a: v1, b: v4, c: v3 });
        self.push_triangle(Triangle3{ a: v1, b: v3, c: v2 });
    }

    pub fn draw_tri(&mut self, tri: Triangle, colour: Vec3, depth: f32) {
        let a = Vert3 {
            pos: Vec3::new(tri.a.x, tri.a.y, depth),
            colour,
        };
        let b = Vert3 {
            pos: Vec3::new(tri.b.x, tri.b.y, depth),
            colour,
        };
        let c = Vert3 {
            pos: Vec3::new(tri.c.x, tri.c.y, depth),
            colour,
        };
        self.push_triangle(Triangle3 {a, b, c});
    }
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct Triangle3 {
    a: Vert3,
    b: Vert3,
    c: Vert3,
}

impl fmt::Debug for Triangle3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos: ({},{},{}), ({},{},{}), ({},{},{}) colour: ({},{},{})", 
            self.a.pos.x,
            self.a.pos.y,
            self.a.pos.z,
            self.b.pos.x,
            self.b.pos.y,
            self.b.pos.z,
            self.c.pos.x,
            self.c.pos.y,
            self.c.pos.z,
            self.a.colour.x,
            self.a.colour.y,
            self.a.colour.z,
         )
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Vert3 {
    pos: Vec3,
    colour: Vec3,
}

pub struct Renderer {
    vbo: NativeBuffer,
    vao: NativeVertexArray,
    shader: NativeProgram,
}

impl Renderer {
    pub fn new(gl: &glow::Context, shader: NativeProgram) -> Renderer {
        unsafe {
            
            // We construct a buffer and upload the data
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // We now construct a vertex array to describe the format of the input buffer
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*2*3, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 4*2*3, 4*3);
            gl.enable_vertex_attrib_array(1);
    
            Renderer {
                vao,
                vbo,
                shader,
            }
        }
    }

    pub fn present(&mut self, gl: &glow::Context, triangles: TriangleBuffer) {
        unsafe {
            gl.use_program(Some(self.shader));
            let gpu_bytes: &[u8] = core::slice::from_raw_parts(
                triangles.tris.as_ptr() as *const u8,
                3 * 4 * 6 * triangles.tris.len(),
            ); // 3 for points in triangle, 4 for bytes in float, 6 for floats in vertex
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, gpu_bytes, glow::DYNAMIC_DRAW);
            gl.draw_arrays(glow::TRIANGLES, 0, triangles.tris.len() as i32 * 3);
            //gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_vertex_array(self.vao);
        }
    }
}