#![no_std]
#![no_main]
use tinix::*;
use user::*;

use graphics::*;

entry_point!(main);
static IMG : &[u8] = include_bytes!("img.bin");


pub fn main(args : &user::Arguments) -> usize {
    vga::clear_screen(Color::Black);

    let spr = Image16::from(IMG);

    spr.draw(0,0);


    //println!("{:?}", spr);

    0
}