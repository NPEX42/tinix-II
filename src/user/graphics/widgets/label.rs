use super::*;
use crate::graphics::{*, self};
pub struct Label {
    foreground : Color,
    background : Color,

    text_width  : usize,
    text_height : usize,

    text : &'static str
}

impl Label {
    pub fn new(text : &'static str, foreground : Color, background : Color) -> Label {
        Label {
            text, foreground, background,

            text_height : graphics::font_height(),
            text_width : text.len() * graphics::font_width(),
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.text_width + 8, self.text_height + 4)
    }
}

impl Renderable for Label {
    fn draw(&self, x : usize, y : usize) {
        draw_filled_rect((x as isize, y as isize), ((self.text_width + 8) as isize, (self.text_height + 4) as isize), self.background);
        draw_str(x + 4, y + 2, self.text, self.foreground)
    }
}

