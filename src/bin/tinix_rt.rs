#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use bootloader::BootInfo;
use tinix::input::serial_println;
use tinix::kernel::drivers::file_systems::{Block, File, open_file};
use tinix::{Arguments, entry_point, size_of};
use tinix::{ConstPointer, custom_boot, kernel::drivers::file_systems::{file_table::{FileTable}}, log};
custom_boot!(main);

pub fn main(_args : &'static BootInfo)  {
    

    log!("Yeet, Yeet, Complete!\n");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
use tinix::println;
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}