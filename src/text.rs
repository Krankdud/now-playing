use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::path::Path;

use crate::song::Song;

pub struct TextRenderer<'a, T> {
    font: Font<'a, 'a>,
    texture_creator: &'a TextureCreator<T>,
}

impl<'a, T> TextRenderer<'a, T> {
    pub fn new(ttf_ctx: &'a Sdl2TtfContext, texture_creator: &'a TextureCreator<T>) -> Self {
        let font = ttf_ctx
            .load_font(Path::new("Silver.ttf"), 36)
            .expect("Could not load font");
        TextRenderer {
            font: font,
            texture_creator: texture_creator,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, string: &str, x: i32, y: i32, color: Color) {
        let text = self
            .font
            .render(string)
            .solid(color)
            .expect("Could not render text");
        let texture = text.as_texture(self.texture_creator).unwrap();
        let mut rect = text.rect();
        rect.set_x(x);
        rect.set_y(y);
        canvas.copy(&texture, None, rect).unwrap();
    }

    pub fn render_song(&self, canvas: &mut WindowCanvas, song: &Song, x: i32, y: i32) {
        let text = self
            .font
            .render(&song.title)
            .solid(Color::RGB(255, 255, 255))
            .expect("Could not render text");
        let texture = text.as_texture(self.texture_creator).unwrap();
        let mut rect = text.rect();
        rect.set_x(x);
        rect.set_y(y);
        canvas.copy(&texture, None, rect).unwrap();

        let y = y + self.font.height() - 16;
        let text = self
            .font
            .render(&song.artist)
            .solid(Color::RGB(255, 255, 255))
            .expect("Could not render text");
        let texture = text.as_texture(self.texture_creator).unwrap();
        let mut rect = text.rect();
        rect.set_x(x);
        rect.set_y(y);
        canvas.copy(&texture, None, rect).unwrap();

        let y = y + self.font.height() - 16;
        let text = self
            .font
            .render(&song.album)
            .solid(Color::RGB(255, 255, 255))
            .expect("Could not render text");
        let texture = text.as_texture(self.texture_creator).unwrap();
        let mut rect = text.rect();
        rect.set_x(x);
        rect.set_y(y);
        canvas.copy(&texture, None, rect).unwrap();

        let y = y + self.font.height() - 16;
        let playback_text = format!("{} / {}", song.position, song.duration);
        let text = self
            .font
            .render(&playback_text)
            .solid(Color::RGB(255, 255, 255))
            .expect("Could not render text");
        let texture = text.as_texture(self.texture_creator).unwrap();
        let mut rect = text.rect();
        rect.set_x(x);
        rect.set_y(y);
        canvas.copy(&texture, None, rect).unwrap();
    }
}
