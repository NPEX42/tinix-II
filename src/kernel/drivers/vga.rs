use volatile::Volatile;
use core::ops::{Deref, DerefMut};
use crate::kernel::hardware::vga as vga_hw;
use crate::kernel::hardware::*;


pub mod libs {
    use lazy_static::lazy_static;
    use spin::*;

    pub use vga::colors::Color16;
    pub use vga::writers::{Graphics640x480x16, GraphicsWriter};
    pub use vga::fonts;

    lazy_static! {
        static ref GFX_MODE : Mutex<Graphics640x480x16> = Mutex::new(
            Graphics640x480x16::new()
        );
    }

    pub fn init() {
        GFX_MODE.lock().set_mode();
    }

    pub fn clear_screen(color : Color16) {
        GFX_MODE.lock().clear_screen(color);
    }

    pub fn draw(x : usize, y : usize, color : Color16) {
        GFX_MODE.lock().set_pixel(x,y,color);
    }

    pub fn draw_str(x : usize, y : usize, text : &str, color : Color16) {
        let mode = GFX_MODE.lock();
        for (offset, chr) in text.chars().enumerate() {
                mode.draw_character(x + offset * 8, y, chr, Color16::White);
        }
    }

    pub fn draw_chr(x : usize, y : usize, chr : char, color : Color16) {
        GFX_MODE.lock().draw_character(x, y, chr, Color16::White);
    }

    pub fn draw_line(start : (isize, isize), end : (isize, isize), color : Color16) {
        GFX_MODE.lock().draw_line(start, end, color);
    }
}





pub const TEXT_MODE_START : usize = 0xb8000;

pub enum VgaError {
    BAD_INDEX = 0,
}

/// The standard color palette in VGA text mode
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

const COLORS: [Color; 16] = [
    Color::Black,
    Color::Blue,
    Color::Green,
    Color::Cyan,
    Color::Red,
    Color::Magenta,
    Color::Brown,
    Color::LightGray,
    Color::DarkGray,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::Pink,
    Color::Yellow,
    Color::White,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ColorAttribute(u8);

impl ColorAttribute {
    pub fn new(fg : Color, bg : Color) -> ColorAttribute {
     ColorAttribute((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    codepoint : u8,
    color : ColorAttribute,
}

impl Char {
    pub fn new(codepoint : u8, fg : Color, bg : Color) -> Self {
        Self {
            codepoint,
            color : ColorAttribute::new(fg, bg)
        }
    }
}


#[derive(Clone)]
#[repr(transparent)]
pub struct TextBuffer {
    data : [[Volatile<Char> ; 80] ; 25]
}

impl TextBuffer {
    pub fn get() -> &'static mut TextBuffer {
        unsafe {&mut *(0xb8000 as *mut TextBuffer)}
    }

    pub fn read(&mut self, x : usize, y : usize) -> Char {
        self.data[y][x].read()
    }

    pub fn write(&mut self, x : usize, y : usize, data : Char) {
        self.data[y][x].write(data);
    }
}


impl Deref for Char {
    type Target = Char;

    fn deref(&self) -> &Char {
        self
    }
}

impl DerefMut for Char {
    fn deref_mut(&mut self) -> &mut Char {
        self
    }
}

pub enum VgaMode {
    TextMode, GraphicsMode
}


pub struct Vga {
    vga_controller : vga_hw::VgaController,
}

impl Vga {
    pub fn get() -> Vga {
        Vga {
            vga_controller : vga_hw::VgaController::new()
        }
    }

    pub fn set_mode(mode : VgaMode) {
        match mode {
            VgaMode::TextMode => unsafe {_enable_text_mode()},
            VgaMode::GraphicsMode => {_enable_graphics_mode()}
        }
    }
}

const VGA_MISC_DATA : u16 = 0x3C0;

#[doc(hidden)]
pub unsafe fn _enable_text_mode() {
    let mut vga = Vga::get().vga_controller;
    let mut crt = Vga::get().vga_controller.crt_controller;

    set_indexed_reg(VGA_MISC_DATA, 0x10, 0x0C);
    set_indexed_reg(VGA_MISC_DATA, 0x11, 0x00);
    set_indexed_reg(VGA_MISC_DATA, 0x12, 0x0F);
    set_indexed_reg(VGA_MISC_DATA, 0x13, 0x08);
    set_indexed_reg(VGA_MISC_DATA, 0x14, 0x00);

    set_reg(0x3C2, 0x67);

    set_coupled_reg(0x3C4, 0x01, 0x00);
    set_coupled_reg(0x3C4, 0x03, 0x00);
    set_coupled_reg(0x3C4, 0x04, 0x07);

    set_coupled_reg(0x3CE, 0x05, 0x10);
    set_coupled_reg(0x3CE, 0x06, 0x0E);

    set_coupled_reg(0x3D4, 0x00, 0x5F);
    set_coupled_reg(0x3D4, 0x01, 0x4F);
    set_coupled_reg(0x3D4, 0x02, 0x50);
    set_coupled_reg(0x3D4, 0x03, 0x82);
    set_coupled_reg(0x3D4, 0x04, 0x55);
    set_coupled_reg(0x3D4, 0x05, 0x81);
    set_coupled_reg(0x3D4, 0x06, 0xBF);
    set_coupled_reg(0x3D4, 0x07, 0x1F);
    set_coupled_reg(0x3D4, 0x08, 0x00);
    set_coupled_reg(0x3D4, 0x09, 0x4F);
    set_coupled_reg(0x3D4, 0x10, 0x9C);
    set_coupled_reg(0x3D4, 0x11, 0x8E);
    set_coupled_reg(0x3D4, 0x12, 0x8F);
    set_coupled_reg(0x3D4, 0x13, 0x28);
    set_coupled_reg(0x3D4, 0x14, 0x1F);
    set_coupled_reg(0x3D4, 0x15, 0x96);
    set_coupled_reg(0x3D4, 0x16, 0xB9);
    set_coupled_reg(0x3D4, 0x17, 0xA3);

}


#[doc(hidden)]
pub fn _enable_graphics_mode() {

}

pub fn set_reg(port : u16, data : u8) {
    unsafe {
        let mut reg = PortWO::new(port);
        reg.write(data);
    }
}


pub fn set_indexed_reg(port : u16, index : u8, data : u8) {
    unsafe {
        let mut ind_rst : PortRO<u8> = PortRO::new(0x3DA);
        ind_rst.read();
        let mut reg = PortWO::new(port);
        reg.write(index);
        reg.write(data);
    }
}

pub fn set_coupled_reg(base_port : u16,  index : u8, data : u8) {
    unsafe {
        let mut ind_reg = PortWO::new(base_port);
        let mut dat_reg = PortWO::new(base_port + 1);

        ind_reg.write(index);
        dat_reg.write(data);
    }
}
