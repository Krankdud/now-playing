extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2::ttf::init().expect("Failed to initialize ttf");

    let font = ttf_ctx
        .load_font(Path::new("Silver.ttf"), 32)
        .expect("Failed to load font");

    let window = video_ctx
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let texture_creator = canvas.texture_creator();

    let text = font
        .render("Hello world")
        .solid(Color::RGB(255, 255, 255))
        .expect("Could not render text");
    let texture = text.as_texture(&texture_creator).unwrap();

    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.copy(&texture, None, text.rect()).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
