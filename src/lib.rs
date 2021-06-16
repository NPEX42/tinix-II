#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
#![allow(deprecated)]
#![feature(alloc_error_handler)] 

use core::panic::PanicInfo;

pub mod kernel;
pub mod user;
pub mod std;


#[panic_handler]
fn on_panic(_info : &PanicInfo) -> ! {
    loop {}
}

pub macro entry_point($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        let main : fn(args : &$crate::user::Arguments) -> (usize) = $path;
        
        $crate::kernel::boot(boot_info);

        main(&$crate::user::Arguments::empty());

        loop {}
    }
}