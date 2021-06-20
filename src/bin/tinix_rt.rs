#![no_std]
#![no_main]
use tinix::*;
use user::*;

use graphics::*;

entry_point!(main);


pub fn main(args : &user::Arguments) -> usize {
    vga::clear_screen(Color::Blue);

    draw_filled_rect((0,0),(128,128), Color::LightGrey);

    loop {
        vga::clear_screen(Color::Blue);
        draw_string_f!(0,0,Color::White,"tick: 0");
        vga::vsync_wait();
    }
    
    0
}