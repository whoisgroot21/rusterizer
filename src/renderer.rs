use crate::math::Vec2;

pub struct Renderer{
	width: usize,
	height: usize,
	color_buffer: Box<[u32]>,
	depth_buffer: Box<[f32]>
}

impl Renderer{
	pub fn new(width: usize, height: usize) -> Self{
		Self{
			width, height,
			color_buffer: vec![0 as u32; width*height].into_boxed_slice(),
			depth_buffer: vec![0 as f32; width*height].into_boxed_slice(),
		}
	}

	pub fn get_render_data(&self) -> &[u32]{
		&self.color_buffer
	}

	pub fn clear(&mut self){
		for i in 0..self.width*self.height{
			self.color_buffer[i] = 0;
			self.depth_buffer[i]= 1.;//[0,1] normalized plane
		}
	}

	#[inline(always)]
	pub fn put_pixel(&mut self, x: usize, y: usize, color: u32){
		let idx = x + y *self.width;
		if idx>self.width*self.height {return;}
		self.color_buffer[idx] = color;
	}

	pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32){
		let m = (y2-y1) as f32/(x2-x1) as f32;
		if (x1-x2).abs()>=(y1-y2).abs(){
			if x1<=x2{
				for x in x1..=x2{
					let y = m*(x-x1) as f32 + y1 as f32;
					self.put_pixel(x as usize, y as usize, color);
				}
			}
			else{
				for x in x2..=x1{
					let y = m*(x-x2) as f32 + y2 as f32;
					self.put_pixel(x as usize, y as usize, color);
				}
			}
		}
		else{
			if y1<=y2{
				for y in y1..y2{
					let x = (y-y1) as f32/m + x1 as f32;
					self.put_pixel(x as usize, y as usize, color);
				}
			}
			else{
				for y in y2..y1{
					let x = (y-y2) as f32/m + x2 as f32;
					self.put_pixel(x as usize, y as usize, color);
				}
			}
		}
	}

	//pub fn draw_triangle(&mut self, ){
		//
	//}
}