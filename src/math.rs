
#[derive(Clone, Copy)]
pub struct Vec2{
	pub x: f32,
	pub y: f32
}

pub struct Vec3{
	pub x: f32,
	pub y: f32,
	pub z: f32
}

pub struct Vec4{
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}


impl Vec2{


	#[inline(always)]
	pub fn lerp(self, other: Vec2, t: f32) -> Vec2 {
		Vec2{
			x: self.x+other.x*t,
			y: self.y+other.y*t
		}
	}
}