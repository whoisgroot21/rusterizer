use crate::math::Vec2i; //{Vec3, Vec4};

pub struct Renderer {
    width: usize,
    height: usize,
    color_buffer: Box<[u32]>,
    depth_buffer: Box<[f32]>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0 as u32; width * height].into_boxed_slice(),
            depth_buffer: vec![0 as f32; width * height].into_boxed_slice(),
        }
    }

    pub fn get_render_data(&self) -> &[u32] {
        &self.color_buffer
    }

    pub fn clear(&mut self) {
        for i in 0..self.width * self.height {
            self.color_buffer[i] = 0;
            self.depth_buffer[i] = 1.; //[0,1] normalized plane
        }
    }

    #[inline(always)]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        let idx = x + y * self.width;
        if idx > self.width * self.height {
            return;
        }
        self.color_buffer[idx] = color;
    }

    //Bresenham's algo
    pub fn draw_line_2d(&mut self, p1: Vec2i, p2: Vec2i, color: u32) {
        let dx = (p2.x-p1.x).abs();
        let dy = (p2.y-p1.y).abs();

        let mut err = dx-dy;

        let mut x = p1.x;
        let mut y = p1.y;

        let x_inc = if p2.x > p1.x { 1 } else { -1 };
        let y_inc = if p2.y > p1.y { 1 } else { -1 };
        loop {
            self.put_pixel(x as usize, y as usize, color);
            if x == p2.x && y == p2.y {
                break;
            }

            let e = err;

            if e>-dx{
                err-=dy;
                x+=x_inc;
            }

            if e<dy{
                err+=dx;
                y+=y_inc;
            }
        }
    }

    /*
    //hybrid DDA
    pub fn draw_line_2d(&mut self, p1: Vec2i, p2: Vec2i, color: u32) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        if dx == 0 && dy == 0 {
            self.put_pixel(p1.x as usize, p2.y as usize, color);
            return;
        }

        if dx.abs() >= dy.abs() {
            let y_inc = dy as f32 / dx.abs() as f32;
            let x_inc = dx / dx.abs();

            let mut x = p1.x;
            let mut y = p1.y as f32;
            for _ in 0..=dx.abs() {
                self.put_pixel(x as usize, y.round() as usize, color);
                y += y_inc;
                x += x_inc;
            }
        } else {
            let y_inc = dy / dy.abs();
            let x_inc = dx as f32 / dy.abs() as f32;

            let mut x = p1.x as f32;
            let mut y = p1.y;
            for _ in 0..=dy.abs() {
                self.put_pixel(x.round() as usize, y as usize, color);
                y += y_inc;
                x += x_inc;
            }
        }
    }
    */
    /*
    //DDA
    pub fn draw_line_2d(&mut self, p1.x: i32, p1.y: i32, p2.x: i32, p2.y: i32, color: u32){
        let dx = p2.x-p1.x;
        let dy = p2.y-p1.y;

        let steps = std::cmp::max(dx.abs(), dy.abs());

        if steps==0{
            self.put_pixel(p1.x as usize, p1.y as usize, color);
            return;
        }

        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;

        let mut x = p1.x as f32;
        let mut y = p1.y as f32;
        for i in 0..=steps{
            self.put_pixel(x.round() as usize, y.round() as usize, color);
            y+=y_inc;x+=x_inc;
        }
    }
    */

    //pub fn draw_line_3d(p1: Vec3, p2: Vec3, color: Vec4){
    //
    //}

    #[inline(always)]
    pub fn draw_triangle_wireframe_2d(&mut self, p1: Vec2i, p2: Vec2i, p3: Vec2i, color: u32) {
        self.draw_line_2d(p1, p2, color);
        self.draw_line_2d(p2, p3, color);
        self.draw_line_2d(p1, p3, color);
    }


    
    
    //barycentric coords approach
    //better for gpu based/parallel processing renderers

    #[inline(always)]
    fn sign_of_weight(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32)-> i32{
        //return (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1);
        return (y2-y1)*(x2+x1) + (y3-y2)*(x3+x2) + (y1-y3)*(x1+x3);
    }

    pub fn draw_triangle_2d(&mut self, p1: Vec2i, p2: Vec2i, p3: Vec2i, color: u32) {
        //for testing
        //self.draw_triangle_wireframe_2d(p1, p2, p3, color);
        let x1 = p1.x.min(p2.x).min(p3.x);
        let x2 = p1.x.max(p2.x).max(p3.x);
        let y1 = p1.y.min(p2.y).min(p3.y);
        let y2 = p1.y.max(p2.y).max(p3.y);

        let w = Self::sign_of_weight(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        for x in x1..=x2{
            for y in y1..=y2{
                let w1 = Self::sign_of_weight(p2.x, p2.y, p3.x, p3.y, x, y);
                let w2 = Self::sign_of_weight(p3.x, p3.y, p1.x, p1.y, x, y);
                let w3 = Self::sign_of_weight(p1.x, p1.y, p2.x, p2.y, x, y);
                if w>0 && (w1<0 || w2<0 || w3<0){continue;}
                if w<0 && (w1>0 || w2>0 || w3>0){continue;}
                
                self.put_pixel(x as usize, y as usize, color);
            }
        }
    }
    //*/

    //better for single threaded, cpu-only renderers
    //like ours
    //scanline approach
    pub fn scan_draw_triangle_2d(&mut self, mut p1: Vec2i, mut p2: Vec2i, mut p3: Vec2i, color: u32) {
        if p1.y>p2.y{
            let tp = p1;
            p1 = p2;
            p2 = tp;
        }
        if p1.y>p3.y{
            let tp = p3;
            p3 = p1;
            p1 = tp;
        }
        if p2.y>p3.y{
            let tp = p2;
            p2 = p3;
            p3 = tp;
        }

        //lerp 1-2
        let mut x1 = p1.x as f32;
        let mut x2 = p1.x as f32;
        let x1_inc = (p2.x-p1.x) as f32 / (p2.y-p1.y).abs() as f32;
        let x2_inc = (p3.x-p1.x) as f32 / (p3.y-p1.y).abs() as f32;
        for y in p1.y..p2.y{
            self.draw_line_2d(Vec2i{x: x1 as i32, y}, Vec2i{x: x2 as i32, y}, color);
            x1 += x1_inc;
            x2 += x2_inc;
        }
        //lerp 2-3
        let mut x1 = p2.x as f32;
        let x2_inc = (p3.x-p1.x) as f32 / (p3.y-p1.y).abs() as f32;
        let x1_inc = (p3.x-p2.x) as f32 / (p3.y-p2.y).abs() as f32;
        for y in p2.y..=p3.y{
            self.draw_line_2d(Vec2i{x: x1 as i32, y}, Vec2i{x: x2 as i32, y}, color);
            x1 += x1_inc;
            x2 += x2_inc;
        }
        
        //for testing
        //self.draw_triangle_wireframe_2d(p1, p2, p3, color);
    }


}
