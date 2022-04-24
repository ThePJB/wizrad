use itertools::Itertools;

/***************************************************
 * Easing
 ***************************************************/
pub fn unlerp(x: f32, t1: f32, t2: f32) -> f32 {
    (x - t1) / (t2 - t1)
}

pub fn gradient(t: f32, colours: Vec<(Vec3, f32)>) -> Vec3 {
    // find nearest 2 neighbours in colours vec and interp between them
    for ((c1, t1), (c2, t2)) in colours.iter().tuple_windows() {
        if t >= *t1 && t <= *t2 {
            return c1.lerp(*c2, unlerp(t, *t1, *t2));
        }
    }

    Vec3::new(1.0, 1.0, 1.0)
}

/***************************************************
 * RNG
 ***************************************************/

pub fn khash(mut state: u32) -> u32 {
    state = (state ^ 2747636419) * 2654435769;
    state = (state ^ (state >> 16)) * 2654435769;
    state = (state ^ (state >> 16)) * 2654435769;
    state
}

pub fn krand(seed: u32) -> f32 {
    khash(seed) as f32 / 4294967295.0
}

pub fn kuniform(seed: u32, min: f32, max: f32) -> f32 {
    min + (khash(seed) as f32 / 4294967295.0) * (max - min)
}

/***************************************************
 * Vec
 ***************************************************/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x / scalar, self.y / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
    pub fn normalize(&self) -> Vec2 { let m = self.magnitude(); if m == 0.0 { *self } else { self.div_scalar(self.magnitude()) }}
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { Vec2::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t)) }
    pub fn rotate(&self, radians: f32) -> Vec2 { 
        Vec2::new(
            self.x * radians.cos() - self.y * radians.sin(), 
            self.x * radians.sin() + self.y * radians.cos()
        ) 
    }
    pub fn offset_r_theta(&self, r: f32, theta: f32) -> Vec2 {
        *self + Vec2::new(r, 0.0).rotate(theta)
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Vec2 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self.mul_scalar(-1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }
    pub fn normalize(&self) -> Vec3 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 { Vec3::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t)) }
    pub fn dist(&self, other: Vec3) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn dot(&self, other: Vec3) -> f32 {self.x*other.x + self.y*other.y + self.z*other.z} // is squ dist lol
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
    pub fn rotate_about_vec3(&self, axis: Vec3, theta: f32) -> Vec3 {
        *self*theta.cos() + (axis.cross(*self)*theta.sin()) + axis * (axis.dot(*self)*(1.0 - theta.cos()))
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.mul_scalar(-1.0)
    }
}

impl std::ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}


/***************************************************
 * Shapes
 ***************************************************/

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{x,y,w,h}
    }
    pub fn child(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(
            self.x + x*self.w,
            self.y + y*self.h,
            self.w * w,
            self.h * h,
        )
    }
    pub fn grid_child(&self, x: i32, y: i32, w: i32, h: i32) -> Rect {
        let r_w = self.w / w as f32;
        let r_h = self.h / h as f32;

        Rect::new(
            self.x + r_w * x as f32,
            self.y + r_h * y as f32,
            r_w,
            r_h,
        )
    }
    pub fn fit_center_square(&self) -> Rect {
        let s = self.w.min(self.h);
        Rect::new_centered(self.x + self.w / 2.0, self.y + self.h / 2.0, s, s)
    }
    pub fn fit_aspect_ratio(&self, a: f32) -> Rect {
        let our_a = self.w / self.h;
        if our_a < a {
            // big a means wide
            // they want wider
            let other_h = our_a / a * self.h;
            let other_y = self.y + (self.h - other_h) / 2.0;
            Rect::new(self.x, other_y, self.w, other_h)
        } else {
            // they want taller
            let other_w = a / our_a * self.w;
            let other_x = self.x + (self.w - other_w) / 2.0;
            Rect::new(other_x, self.y, other_w, self.h)
        }
    }
    pub fn centroid(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }
    pub fn new_centered(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(x-w/2.0, y-h/2.0, w, h)
    }
    pub fn translate(&self, v: Vec2) -> Rect {
        return Rect::new(self.x + v.x, self.y + v.y, self.w, self.h);
    }
    pub fn dilate(&self, d: f32) -> Rect {
        return Rect::new(self.x - d, self.y - d, self.w + 2.0*d, self.h + 2.0*d);
    }
    pub fn left(self) -> f32 {
        self.x
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn top(self) -> f32 {
        self.y
    }
    pub fn bot(self) -> f32 {
        self.y + self.h
    }
    pub fn tl(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn tr(self) -> Vec2 {
        Vec2::new(self.x + self.w, self.y)
    }
    pub fn bl(self) -> Vec2 {
        Vec2::new(self.x, self.y + self.h)
    }
    pub fn br(self) -> Vec2 {
        Vec2::new(self.x + self.w, self.y + self.h)
    }
    pub fn contains(self, point: Vec2) -> bool {
        self.x < point.x && self.x + self.w > point.x &&
        self.y < point.y && self.y + self.h > point.y
    }
    pub fn relative_point(self, point: Vec2) -> Vec2 {
        Vec2::new((point.x - self.x) / self.w, (point.y - self.y) / self.h)
    }
    pub fn grid_square(self, point: Vec2, w: i32, h: i32) -> (i32, i32) {
        ((w as f32 * point.x) as i32, (h as f32 * point.y) as i32)
    }
    pub fn tri_child(&self, which: usize) -> Triangle {
        match which {
            0 => Triangle::new(self.tl(), self.tr(), self.centroid()),
            1 => Triangle::new(self.tr(), self.br(), self.centroid()),
            2 => Triangle::new(self.br(), self.bl(), self.centroid()),
            3 => Triangle::new(self.bl(), self.tl(), self.centroid()),
            _ => panic!("bad triangle number"),
        }
    }
}

pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
        Triangle {a, b, c}
    }

    pub fn dilate(&self, d: f32) -> Triangle {
        let centroid = Vec2::new((self.a.x + self.b.x + self.c.x) / 3.0, (self.a.y + self.b.y + self.c.y) / 3.0);
        Triangle::new(
            self.a + (self.a - centroid) * d,
            self.b + (self.b - centroid) * d,
            self.c + (self.c - centroid) * d,
        )
    }

    pub fn contains(&self, p: Vec2) -> bool {
        let denom = self.a.x * (self.b.y - self.c.y) + self.a.y * (self.c.x - self.b.x) + self.b.x*self.c.y - self.b.y * self.c.x;
        let t1 = (p.x * (self.c.y - self.a.y) + p.y * (self.a.x - self.c.x) - self.a.x*self.c.y + self.a.y*self.c.x) / denom;
        let t2 = (p.x * (self.b.y - self.a.y) + p.y * (self.a.x - self.b.x) - self.a.x*self.b.y + self.a.y*self.b.x) / -denom;
        let s = t1 + t2;
 
         return 0.0 <= t1 && t1 <= 1.0 && 0.0 <= t2 && t2 <= 1.0 && s <= 1.0;
    }
}