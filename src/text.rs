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

    text_width: i32,
}

impl<'a, T> TextRenderer<'a, T> {
    pub fn new(
        ttf_ctx: &'a Sdl2TtfContext,
        texture_creator: &'a TextureCreator<T>,
        font: &str,
        font_size: u16,
        window_width: u32,
    ) -> Self {
        let font = ttf_ctx
            .load_font(Path::new(font), font_size)
            .expect("Could not load font");
        TextRenderer {
            font: font,
            texture_creator: texture_creator,
            text_width: (window_width - 12) as i32,
        }
    }

    fn draw_text(&self, canvas: &mut WindowCanvas, string: &str, x: i32, y: i32, color: Color) {
        if string.len() == 0 {
            return;
        }

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

    pub fn render(&self, canvas: &mut WindowCanvas, string: &str, x: i32, y: i32, color: Color) {
        let (width, _height) = self.font.size_of(string).unwrap();
        if x + (width as i32) > self.text_width {
            let splits: Vec<&str> = string.split(' ').collect();
            let mut truncated = String::new();
            let mut len: u32 = 0;
            let (space_width, _) = self.font.size_of(" ").unwrap();

            // Add words until we run out of space.
            for word in splits.iter() {
                let (sw, _sh) = self.font.size_of(word).unwrap();
                if x + ((len + sw) as i32) > self.text_width {
                    // Add characters until we run out of space.
                    for (i, c) in word.chars().enumerate() {
                        let (cw, _ch) = self.font.size_of_char(c).unwrap();
                        if x + ((len + cw) as i32) > self.text_width {
                            // Remove two characters so the whitespace will also be removed
                            if i == 1 {
                                truncated.pop();
                            }
                            truncated.pop();
                            truncated.push_str("…");
                            break;
                        }
                        truncated.push(c);
                        len += cw;
                    }
                    break;
                } else {
                    truncated.push_str(word);
                    truncated.push_str(" ");
                    len += sw + space_width;
                }
            }
            self.draw_text(canvas, &truncated, x, y, color);
        } else {
            self.draw_text(canvas, string, x, y, color);
        }
    }

    pub fn render_song(
        &self,
        canvas: &mut WindowCanvas,
        song: &Song,
        x: i32,
        y: i32,
        color: Color,
    ) {
        self.render(canvas, &song.title, x, y, color);

        let y = y + self.font.height() - 16;
        self.render(canvas, &song.artist, x, y, color);

        let y = y + self.font.height() - 16;
        self.render(canvas, &song.album, x, y, color);

        let y = y + self.font.height() - 16;
        let playback_text = format!("{} / {}", song.position, song.duration);
        self.render(canvas, &playback_text, x, y, color);
    }
}
