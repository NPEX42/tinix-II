use core::ops::Range;

use alloc::boxed::Box;
use spin::Mutex;
use ::vga::{colors::TextModeColor, writers::{ScreenCharacter, Text80x25, TextWriter}};
use x86_64::instructions::interrupts::without_interrupts;

use crate::{graphics::*, io::IoWriter};
use lazy_static::lazy_static;

pub const PRINTABLE_RANGE : Range<u8> = 0x20..0x7E;
pub const UNPRINTABLE_BLOCK : u8 = 0xFE;
pub const BACKSPACE : u8 = 0x08;
pub const DELETE : u8 = 0x7F;
pub const NEW_LINE : u8 = b'\n';
pub const CARRIAGE_RETURN : u8 = b'\r';
pub const TAB : u8 = b'\t';
pub const ESCAPE : u8 = 0x1B;
pub const SPACE : u8 = b' ';

pub const TAB_STOP : usize = 4;
pub const WIDTH : usize = 80;
pub const HEIGHT : usize = 25;

pub const DEFAULT_TEXT_COLOR : TextModeColor = TextModeColor::new(Color::White, Color::Blue);
pub const ERROR_TEXT_COLOR : TextModeColor = TextModeColor::new(Color::Black, Color::Red);

pub const BLANK : ScreenCharacter = ScreenCharacter::new(b' ', DEFAULT_TEXT_COLOR);

lazy_static! {
    static ref STDOUT : Mutex<Console> = Mutex::new(Console::new(DEFAULT_TEXT_COLOR));
    static ref STDERR : Mutex<Console> = Mutex::new(Console::new(ERROR_TEXT_COLOR));
}

pub struct Console {
    writer : Text80x25,
    x : usize,
    y : usize,
    foreground : TextModeColor,
}

impl Console {
    pub fn new(color : TextModeColor) -> Console {
        let writer = Text80x25::new();
        writer.set_mode();
        Console {
            x : 0,
            y : 0,
            writer,
            foreground : color
        }
    }

    pub fn write_byte(&mut self, byte : u8) {
        match byte {
            0x20..=0x7E => {
                self.writer.write_character(self.x, self.y, ScreenCharacter::new(byte, self.foreground));
                self.x += 1;
            },
            NEW_LINE  => {self.new_line()},
            CARRIAGE_RETURN => {self.carriage_return()},
            DELETE => {self.write_byte(BACKSPACE)},
            BACKSPACE => {if self.x > 0 {self.x -= 1;} self.erase_current() },
            _ => {self.write_byte(UNPRINTABLE_BLOCK)}
        }
        
    }

    fn erase_current(&mut self) {
        self.writer.write_character(self.x, self.y, BLANK)
    }

    fn new_line(&mut self) {
        self.y += 1;
        if self.y >= HEIGHT {
            unsafe {
                self.shift_contents_up();
            }
            

        }
        self.carriage_return()
    }

    fn clear(&mut self) {
        self.writer.fill_screen(BLANK);
    }
    /// [WARNING] This function performs potentially risky raw pointer operations,
    /// it must ONLY be called by newline(), and must be acknowledged by an unsafe call.
     unsafe fn shift_contents_up(&mut self) {
            let buffer = self.writer.get_frame_buffer().1;
            for y in 1..HEIGHT {
                for x in 0..WIDTH {
                    let above : isize = (y as isize - 1) * WIDTH as isize + x as isize;
                    let current_offset : isize = (y as isize) * WIDTH as isize + x as isize;
                    *buffer.offset(above) = *buffer.offset(current_offset);
                }
            }
    }

    fn carriage_return(&mut self) {
        self.x = 0;
    }

    pub fn home(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    fn tab(&mut self) {
        for i in 0..=TAB_STOP {
            self.write_byte(SPACE);
        }
    }

    pub fn enable_cursor(&mut self) {
        self.writer.enable_cursor();
    }

    pub fn disable_cursor(&mut self) {
        self.writer.disable_cursor();
    }

    pub fn position_cursor(&mut self, x : usize, y : usize) {
        self.writer.set_cursor_position(x, y)
    }

    pub fn clear_current_row(&mut self) {
        self.carriage_return();
        for _ in 0..WIDTH {
            self.write_byte(SPACE);
        }
    }
}

pub fn _print(args : Arguments) {
    without_interrupts(||{
        STDOUT.lock().write_fmt(args).expect("Error Writing to STDOUT");
    })
}

impl IoWriter<&str> for Console {
    fn write(&mut self, item : &str) {
        for byte in item.as_bytes() {
            if self.x >= WIDTH {self.write_byte(NEW_LINE); self.write_byte(CARRIAGE_RETURN);}
            self.write_byte(*byte);
        } 
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        self.write(s);
        Ok(())
    }
}




pub fn clear() {
    without_interrupts(|| {
        STDOUT.lock().clear();
    })
}

pub fn home() {
    without_interrupts(|| {
        STDOUT.lock().home();
    })
}