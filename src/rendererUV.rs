use crate::kmath::*;
use crate::kimg::*;

use glow::*;

use std::fmt;

#[derive(Debug)]
pub struct TriangleBufferUV {
    screen_rect: Rect,
    pub tris: Vec<Triangle3UV>,
    atlas_w: i32,
    atlas_h: i32,
}

impl TriangleBufferUV {
    pub fn new(screen_rect: Rect, atlas_w: i32, atlas_h: i32) -> TriangleBufferUV {
        TriangleBufferUV { screen_rect, tris: Vec::new(), atlas_w, atlas_h }
    }

    fn push_triangle(&mut self, mut tri: Triangle3UV) {
        tri.a.pos.x  = (tri.a.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.a.pos.y  = (tri.a.pos.y - self.screen_rect.y) / self.screen_rect.h;

        tri.b.pos.x  = (tri.b.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.b.pos.y  = (tri.b.pos.y - self.screen_rect.y) / self.screen_rect.h;

        tri.c.pos.x  = (tri.c.pos.x - self.screen_rect.x) / self.screen_rect.w;
        tri.c.pos.y  = (tri.c.pos.y - self.screen_rect.y) / self.screen_rect.h;

        self.tris.push(tri);
    }

    pub fn draw_sprite(&mut self, r: Rect, i: i32, depth: f32) {
        let colour = Vec3::new(1.0, 1.0, 1.0);
        let x = i / self.atlas_w;
        let y = i % self.atlas_h;
        let uv_w = 1.0 / self.atlas_w as f32;
        let uv_h = 1.0 / self.atlas_h as f32;
        let uv = Rect::new(x as f32 * uv_w, y as f32 * uv_h, uv_w, uv_h);

        let v1 = Vert3UV {
            pos: Vec3::new(r.x, r.y, depth),
            colour: colour,
            uv: uv.tl(),
        };
        let v2 = Vert3UV {
            pos: Vec3::new(r.x, r.y + r.h, depth),
            colour: colour,
            uv: uv.tr(),
        };
        let v3 = Vert3UV {
            pos: Vec3::new(r.x + r.w, r.y + r.h, depth),
            colour: colour,
            uv: uv.br(),
        };
        let v4 = Vert3UV {
            pos: Vec3::new(r.x + r.w, r.y, depth),
            colour: colour,
            uv: uv.bl(),
        };
        self.push_triangle(Triangle3UV{ a: v1, b: v4, c: v3 });
        self.push_triangle(Triangle3UV{ a: v1, b: v3, c: v2 });
    }
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct Triangle3UV {
    a: Vert3UV,
    b: Vert3UV,
    c: Vert3UV,
}

impl fmt::Debug for Triangle3UV {
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
struct Vert3UV {
    pos: Vec3,
    colour: Vec3,
    uv: Vec2,
}

pub struct RendererUV {
    vbo: NativeBuffer,
    vao: NativeVertexArray,
    shader: NativeProgram,

    atlas: NativeTexture,

}

impl RendererUV {
    pub fn new(gl: &glow::Context, shader: NativeProgram, atlas: &str) -> RendererUV {
        
        unsafe {
            let image = ImageBufferA::new_from_file(atlas); // needs rgba
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, image.w as i32, image.h as i32, 0, RGBA, glow::UNSIGNED_BYTE, Some(&image.bytes_transpose()));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.generate_mipmap(glow::TEXTURE_2D);

            // We construct a buffer and upload the data
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // We now construct a vertex array to describe the format of the input buffer
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*2*3 + 4*2, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 4*2*3 + 4*2, 4*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 4*2*3 + 4*2, 2*4*3);
            gl.enable_vertex_attrib_array(2);
    
            RendererUV {
                vao,
                vbo,
                shader,
                atlas: texture,
            }
        }
    }

    pub fn present(&mut self, gl: &glow::Context, triangles: TriangleBufferUV) {
        unsafe {
            gl.use_program(Some(self.shader));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.atlas));

            let gpu_bytes: &[u8] = core::slice::from_raw_parts(
                triangles.tris.as_ptr() as *const u8,
                3 * 4 * 8 * triangles.tris.len(),
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
            gl.delete_texture(self.atlas);
        }
    }
}