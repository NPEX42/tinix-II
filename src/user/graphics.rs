pub mod widgets;

pub use crate::kernel::drivers::vga_dr as vga;

pub use crate::kernel::drivers::vga_dr::Color16 as Color;

use crate::user::math::*;

pub use crate::kernel::drivers::vga_dr::draw_line as draw_line;
pub use crate::kernel::drivers::vga_dr::draw as draw;
pub use crate::kernel::drivers::vga_dr::draw_str as draw_str;
pub use crate::kernel::drivers::vga_dr::draw_chr as draw_chr;
pub use crate::kernel::drivers::vga_dr::clear_screen as clear_screen;


pub const WIDTH  : usize = 640;
pub const HEIGHT : usize = 480;


pub use core::fmt::*;

pub fn font_height() -> usize {
    vga::fonts::TEXT_8X8_FONT.character_height as usize
}

pub fn font_width() -> usize {
    8
}

pub fn draw_rect(pos : (isize, isize), dim : (isize, isize), color : Color) {
    vga::draw_line((pos.0, pos.1), (pos.0 + dim.0, pos.1), color);
    vga::draw_line((pos.0 + dim.0, pos.1), (pos.0 + dim.0, pos.1 + dim.1), color);
    vga::draw_line((pos.0 + dim.0, pos.1 + dim.1), (pos.0, pos.1 + dim.1), color);
    vga::draw_line((pos.0, pos.1 + dim.1), (pos.0, pos.1), color);
}

pub fn draw_filled_rect(pos : (isize, isize), dim : (isize, isize), color : Color) {
    for y in pos.1 ..= dim.1 {
        vga::draw_line((pos.0, y),(pos.0 + dim.0, y),color);
    }
}

pub fn draw_triangle(p0 : (isize, isize), p1 : (isize, isize), p2 : (isize, isize), color : Color) {
    vga::draw_line(p0, p1, color);
    vga::draw_line(p1, p2, color);
    vga::draw_line(p2, p0, color);
}

pub fn fill_triangle(p0 : (isize, isize), p1 : (isize, isize), p2 : (isize, isize), color : Color) {
    let min_y = mini(mini(p0.1, p1.1),p2.1);
    let max_y = maxi(maxi(p0.1, p1.1),p2.1);

    let min_x = mini(mini(p0.0, p1.0),p2.0);
    let max_x = maxi(maxi(p0.0, p1.0),p2.0);

    // Debugging
    draw_rect((min_x, min_y),(max_x, max_y),Color::Blue);
    draw_triangle(p0,p1,p2,color);
}

pub fn cross(pos : (isize, isize), size : isize, color : Color) {
    let half_size = size / 2;
    draw_line(
        (pos.0 - half_size, pos.1 - half_size),
        (pos.0 + half_size, pos.1 + half_size),
        color
    );

    draw_line(
        (pos.0 - half_size, pos.1 - half_size),
        (pos.0 + half_size, pos.1 + half_size),
        color
    );
}

#[derive(Debug, Copy, Clone)]
pub struct Image16<'a> {
    height  : u8,
    width   : u8,

    data : &'a [u8]
}

impl<'a> Image16<'a> {
    pub fn from(raw : &'a [u8]) -> Image16 {
        Image16 {
            width : raw[0],
            height : raw[1],

            data : &raw[2..]
        }
    }

    pub fn draw(&self, ox : usize, oy : usize) {
        for y in 0..self.height as u16 {
            for x in 0..self.width {
                let index = (y * self.width as u16 + x as u16) as usize;
                draw( ox+x  as usize, oy+y as usize, COLORS[self.data[index] as usize % 16])
            }
        }
    }
}

pub const COLORS : &[Color; 16] = &[
    Color::Black,
    Color::Blue,
    Color::Green,
    Color::Cyan,
    Color::Red,
    Color::Magenta,
    Color::Brown,
    Color::LightGrey,
    Color::DarkGrey,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::Pink,
    Color::Yellow,
    Color::White,
];



#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::user::graphics::_print(format_args!($($arg)*)) }
}

pub fn _print(args : Arguments) {
        crate::io::devices::console::_print(args);
}



