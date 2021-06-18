#![no_std]
#![no_main]
use tinix::*;
use user::*;

use graphics::*;

entry_point!(main);


pub fn main(args : &user::Arguments) -> usize {
    vga::clear_screen(Color::Black);
    println!("Hello World!");
    0
}