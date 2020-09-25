extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

mod song;
use song::SongReader;

mod text;
use text::TextRenderer;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2::ttf::init().expect("Failed to initialize ttf");

    let window = video_ctx
        .window("Now Playing", 320, 200)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let texture_creator = canvas.texture_creator();

    let text_renderer = TextRenderer::new(&ttf_ctx, &texture_creator);

    let mut song_reader = SongReader::from("now_playing.txt");

    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let song = match song_reader.update() {
            Err(_) => song_reader.get_song(),
            Ok(song) => song,
        };

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        text_renderer.render(&mut canvas, "Now Playing:", 8, 4, Color::RGB(255, 128, 0));
        text_renderer.render_song(&mut canvas, &song, 8, 32);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
