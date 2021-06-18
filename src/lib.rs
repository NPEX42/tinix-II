#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
#![allow(deprecated)]
#![feature(alloc_error_handler)] 

#[warn(missing_docs)]


use core::panic::PanicInfo;

mod kernel;
pub mod user;
pub mod std;


/// Define Our own panic Handler
#[panic_handler]
fn on_panic(_info : &PanicInfo) -> ! {
    user::graphics::clear_screen(user::graphics::Color::Red);
    println!("== PANIC == [ {} ]", _info);
    loop {}
}

/// Define an entry Point that
/// A) Boots the system
/// B) Calls the given function
pub macro entry_point($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        let main : fn(args : &$crate::user::Arguments) -> (usize) = $path;
        
        $crate::kernel::boot(boot_info);

        main(&$crate::user::Arguments::empty());
        
        loop {}
    }
}