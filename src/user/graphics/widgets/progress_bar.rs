use super::*;
use crate::{graphics::*};

pub struct ProgressBar {
    text : &'static str,
    min : usize,
    max : usize,
    fill : f32,
    scale : usize,
    
    empty_color : Color,
    filled_color : Color,
}

impl Renderable for ProgressBar {
    fn draw(&self, x : usize, y : usize) {
        let lbl = label::Label::new(self.text, Color::White, Color::Blue);
        lbl.draw(x, y);
        let (pb_ox, pb_oy) = lbl.dimensions();
        draw_filled_rect(
            ((pb_ox + x) as isize, (y) as isize),
            ((self.scale as f32) as isize,pb_oy as isize),
            self.empty_color
        );

        draw_filled_rect(
            ((pb_ox + x) as isize, (y) as isize),
            ((self.scale as f32 * self.fill) as isize,pb_oy as isize),
            self.filled_color
        );

       
    }
}

impl ProgressBar {
    pub fn new(text : &'static str, max : usize, fill : Color, base : Color, scale : usize) -> Self {
        Self {
            filled_color : fill,
            empty_color : base,
            max,
            scale,
            text,
            min : 0,
            fill : 0.0

        }
    }

    pub fn set_value(&mut self, value : usize) {
        self.fill = (value as f32 / self.max as f32) as f32
    }

    pub fn min(&self) -> usize {
        self.min
    }
}