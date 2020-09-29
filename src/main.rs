extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::Read;
use std::time::Duration;

mod song;
use song::SongReader;

mod text;
use text::TextRenderer;

#[derive(Deserialize)]
struct Config {
    #[serde(with = "hex")]
    color_now_playing: Vec<u8>,
    #[serde(with = "hex")]
    color_song: Vec<u8>,
    filename: String,
    font: String,
    font_size: u16,
    window_width: u32,
    window_height: u32,
}

fn default_config() -> Config {
    Config {
        color_now_playing: vec![255, 128, 0],
        color_song: vec![255, 255, 255],
        filename: String::from("now_playing.txt"),
        font: String::from("Silver.ttf"),
        font_size: 36,
        window_width: 320,
        window_height: 140,
    }
}

fn load_config() -> Result<Config, io::Error> {
    let mut f = File::open("config.toml")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let config: Config = toml::from_str(&s)?;
    Ok(config)
}

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            default_config()
        }
    };

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2::ttf::init().expect("Failed to initialize ttf");

    let window = video_ctx
        .window("Now Playing", config.window_width, config.window_height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let texture_creator = canvas.texture_creator();

    let text_renderer = TextRenderer::new(
        &ttf_ctx,
        &texture_creator,
        &config.font,
        config.font_size,
        config.window_width,
    );

    let mut song_reader = SongReader::from(&config.filename);

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

        let color = Color::RGB(
            config.color_now_playing[0],
            config.color_now_playing[1],
            config.color_now_playing[2],
        );
        text_renderer.render(&mut canvas, "Now Playing:", 8, 4, color);
        let color = Color::RGB(
            config.color_song[0],
            config.color_song[1],
            config.color_song[2],
        );
        text_renderer.render_song(&mut canvas, &song, 8, 32, color);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
