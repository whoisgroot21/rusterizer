use std::time;

use minifb::{Window, WindowOptions, Key};

mod math;
use math::*;
mod renderer;
use renderer::*;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const UPDATE_RATE: usize = 60;

fn main() {
        let mut window = Window::new(
        "rusterizer!!",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
        ).unwrap();

        window.set_target_fps(UPDATE_RATE);

        let mut renderer = Renderer::new(WIDTH, HEIGHT);

        let mut delta_time:u128 = 0;

        let mut time_last = time::Instant::now();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let time_now = time::Instant::now();
            
            delta_time += time_now.duration_since(time_last).as_millis();
            time_last = time_now;
            
            //println!("{:?}", delta_time);//for debuging
            
            while delta_time > UPDATE_RATE as u128{
                delta_time-=UPDATE_RATE as u128;
            }
            
            renderer.clear();
            renderer.draw_line(10,20,10,10, 0xFFFFFF);

            window.update_with_buffer(renderer.get_render_data(), WIDTH, HEIGHT).unwrap();
        }
}
