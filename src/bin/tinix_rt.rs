#![no_std]
#![no_main]
use tinix::{*, graphics::widgets::Renderable, input::serial_println};
use user::*;

use graphics::*;
use x86_64::{VirtAddr, structures::paging::{Page, PageTable}};

use data::boxed::Box;
use data::vec;
use vec::Vec;
use data::rc::*;

custom_boot!(main);


pub fn main(boot : &'static bootloader::BootInfo) {
    loop {
        time::sleep(1.0);
        vga::clear_screen(Color::Blue);
        draw_string_f!(0,0,Color::LightRed, "Time: {}", time::get_rtc());
        draw_string_f!(0,20,Color::LightRed, "Free: {}, Used: {}, Total: {}", sys::mem::free(), sys::mem::used(), sys::mem::total());
    }
}