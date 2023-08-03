extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
 
pub fn main() {
    let clock=Instant::now();
    let mut t: f64 = 0.0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("DeltaTime", 1366, 768)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    'running: loop {
        //t = (clock.elapsed().as_secs()) as f64;
        t += (clock.elapsed().subsec_millis() as f64) / 1000.0;

        i = ((((t*0.92/40.0).sin())+1.0)*(255.0/2.0)) as u8;
        g = ((((t*2.14/40.0).sin())+1.0)*(255.0/2.0)) as u8;
        b = ((((t*3.34/40.0).sin())+1.0)*(255.0/2.0)) as u8;
        canvas.set_draw_color(Color::RGB(i, g, b));
        
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}