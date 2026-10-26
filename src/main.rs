use std::time;

use minifb::{Key, Window, WindowOptions};

pub mod math;
use math::Vec2i;
pub mod renderer;
use renderer::Renderer;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const UPDATE_RATE: usize = 60;

fn main() {
    let mut window = Window::new("rusterizer!!", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(UPDATE_RATE);

    let mut renderer = Renderer::new(WIDTH, HEIGHT);


    //do once stuff
    let mut test: Vec<[Vec2i; 3]> = Vec::with_capacity(250);
    for i in 0..250{
        test.push([Vec2i{x: rand::random_range(0..WIDTH as i32), y: rand::random_range(0..HEIGHT as i32)},
            Vec2i{x: rand::random_range(0..WIDTH as i32), y: rand::random_range(0..HEIGHT as i32)},
            Vec2i{x: rand::random_range(0..WIDTH as i32), y: rand::random_range(0..HEIGHT as i32)}]);
        //test[i]=Vec2i{x: rand::random_range(0..WIDTH as i32), y: rand::random_range(0..HEIGHT as i32)};
    }
    
    let t0 = time::Instant::now();

    for i in 0..250{
        renderer.scan_draw_triangle_2d(test[i][0], test[i][1], test[i][2], 0xFFFFFF);
    }

    renderer.clear();
    let t1 = time::Instant::now();

    for i in 0..250{
        renderer.draw_triangle_2d(test[i][0], test[i][1], test[i][2], rand::random_range(0..0xFFFFFF));
    }

    let t2 = time::Instant::now();

    println!("scanline approach: {}\nbarycentric approach: {}",
        t1.duration_since(t0).as_millis(),
        t2.duration_since(t1).as_millis()
        );

    let mut delta_time: u128 = 0;

    let mut time_last = time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_now = time::Instant::now();

        delta_time += time_now.duration_since(time_last).as_millis();
        time_last = time_now;

        //println!("{:?}", delta_time);//for debuging

        while delta_time > UPDATE_RATE as u128 {
            delta_time -= UPDATE_RATE as u128;
        }

        //renderer.clear();
        window
            .update_with_buffer(renderer.get_render_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}
