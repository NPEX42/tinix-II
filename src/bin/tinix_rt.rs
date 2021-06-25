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
    vga::clear_screen(Color::Blue);
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
}