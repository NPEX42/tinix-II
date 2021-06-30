pub mod widgets;

pub use crate::kernel::drivers::vga_dr as vga;

pub use crate::kernel::drivers::vga_dr::Color16 as Color;

use crate::user::math::*;

pub use crate::kernel::drivers::vga_dr::draw_line as draw_line;
pub use crate::kernel::drivers::vga_dr::draw as draw;
pub use crate::kernel::drivers::vga_dr::draw_str as draw_str;
pub use crate::kernel::drivers::vga_dr::draw_chr as draw_chr;
pub use crate::kernel::drivers::vga_dr::clear_screen as clear_screen;
use x86_64::instructions::interrupts::without_interrupts;


pub const WIDTH  : usize = 640;
pub const HEIGHT : usize = 480;


pub use core::fmt::*;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref WRITER : Mutex<VgaWriter> = Mutex::new(VgaWriter::new(0,0, Color::White));
}

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

#[macro_export]
macro_rules! draw_string_f {
    ($x:expr, $y:expr,$c : expr, $($arg:tt)*) => { 
        $crate::user::graphics::_draw_string_f($x, $y, $c, format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! reset_pos {
    () => {
        $crate::user::graphics::_reset_pos();
    };
}


#[doc(hidden)]
pub fn _reset_pos() {
    without_interrupts(|| {
        WRITER.lock().reset_pos()
    });
}


#[doc(hidden)]
pub fn _print(args: Arguments) {
    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _draw_string_f(x : usize, y : usize, color : Color, args : Arguments) {
    without_interrupts(|| {
        VgaWriter::new(x,y,color).write_fmt(args).expect("FORMAT ERROR");
    });
}

pub struct VgaWriter {
    x : usize, y : usize, color : Color
}

impl VgaWriter {
    pub fn new(x : usize, y : usize, color : Color) -> VgaWriter {
        VgaWriter {
            x, y, color
        }
    }

    pub fn write_str(&mut self, txt : &str) {
        for (_, chr) in txt.chars().enumerate() {
            if chr != '\n' && self.x < 640 {
                self.write_chr(chr);
                self.x += font_width();
            } else {
                self.x = 1;
                self.y += font_height();
            } 
            
        }
    }

    fn write_chr(&mut self, chr : char) {
        draw_chr(self.x, self.y, chr, self.color);
    }

    fn reset_pos(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s : &str) -> Result {
        self.write_str(s);
        Ok(())
    }
}