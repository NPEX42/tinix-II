#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
#![allow(deprecated)]
#![feature(alloc_error_handler)]
#![feature(asm)]

#[warn(missing_docs)]

#[cfg(feature = "liballoc")]
pub extern crate alloc;

use core::panic::PanicInfo;
use x86_64::instructions::interrupts::without_interrupts;

pub mod kernel;
pub mod user;
pub mod std;


pub use kernel::arch::x64::mem::*;
pub use user::*;

use x86_64::VirtAddr;

/// Define Our own panic Handler
#[panic_handler]
fn on_panic(_info : &PanicInfo) -> ! {
    without_interrupts(|| {
        user::graphics::clear_screen(user::graphics::Color::Red);
        println!("== PANIC ==\n [ {} ]", _info);

        input::serial_print!("{}", _info);
    });
    loop {user::time::sleep_ticks(1000)}
}

/// Define an entry Point that
/// A) Boots the system
/// B) Calls the given function
pub macro entry_point($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        let main : fn(&'static bootloader::BootInfo, args : &$crate::user::Arguments) -> (usize) = $path;
        
        $crate::kernel::boot(boot_info);

        main(boot_info, &$crate::user::Arguments::empty());
        
        loop {}
    }
}


pub macro custom_boot($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        use x86_64::VirtAddr;
        use $crate::*;
        let main : fn(&'static bootloader::BootInfo) = $path;
        

        $crate::kernel::boot(boot_info);

        main(boot_info);
        
        loop {
            $crate::user::time::sleep(1.0);
        }
    }
}




#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION", "?.?.?")
}