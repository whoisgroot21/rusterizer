#[derive(Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct Mat4x4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4
}


impl Vec2 {
    #[inline(always)]
    pub fn lerp(self, other: Vec2, t: f32) -> Vec2 {
        Vec2 {
            x: self.x + other.x * t,
            y: self.y + other.y * t,
        }
    }
}

impl Vec4{
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self{
        Self{x,y,z,w}
    }

    #[inline(always)]
    pub fn dot(&self, rhs: &Self)-> f32{
        return self.x*rhs.x+self.y*rhs.y+self.z*rhs.z+self.w*rhs.w;
    }
}

impl Mat4x4 {
    pub fn mul_with_vec(&self, rhs: &Vec4)-> Vec4{
        Vec4{
            x: self.x_axis.dot(rhs),
            y: self.y_axis.dot(rhs),
            z: self.z_axis.dot(rhs),
            w: self.w_axis.dot(rhs)
        }
    }
}