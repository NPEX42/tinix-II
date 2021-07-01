#![no_std]
#![no_main]
use tinix::{*, graphics::widgets::Renderable, input::serial_println, sys::mem};
use user::*;

use graphics::*;
use x86_64::{VirtAddr, structures::paging::{Page, PageTable}};

use data::boxed::Box;
use data::vec;
use vec::Vec;
use data::rc::*;

use io::IoReader;

custom_boot!(main);


pub fn main(boot : &'static bootloader::BootInfo) {
    loop {
        log!("Total:{} Free:{} Used:{}, Keyboard Head: {:?}           \r", mem::total(), mem::free(), mem::used(), io::devices::keyboard::KeyBoard.read());

        time::sleep(0.033);
    }
}