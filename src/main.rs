extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Failed to create renderer: {}", err),
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
