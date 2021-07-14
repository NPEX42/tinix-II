#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
#![allow(deprecated)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


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
/// Define Our own panic Handler
#[panic_handler]
fn on_panic(_info : &PanicInfo) -> ! {
    without_interrupts(|| {
        crate::io::devices::console::clear();
        crate::io::devices::console::home();
        log!("== PANIC ==\nInfo: \n{}", _info);
        log!("\n");
    });
    loop {user::time::sleep_ticks(1000)}
}

/// Define an entry Point that
/// A) Boots the system
/// B) Calls the given function
pub macro entry_point($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        use tinix::input::*;
        $crate::kernel::drivers::file_systems::check_sizes();



        let main : fn(&'static bootloader::BootInfo, args : &$crate::user::Arguments) -> (usize) = $path;
        
        $crate::kernel::boot(boot_info);

        main(boot_info, &$crate::user::Arguments::empty());
        
        loop {user::time::sleep_ticks(100)}
    }
}


pub macro custom_boot($path : path) {

    bootloader::entry_point!(tinix_start);

    pub fn tinix_start(boot_info : &'static bootloader::BootInfo) -> ! {
        use x86_64::VirtAddr;
        $crate::kernel::drivers::file_systems::check_sizes();
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
        panic!("OUT OF MEMORY!\n {:?}", layout);

}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION", "?.?.?")
}

pub mod test {

}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[macro_export]
macro_rules! size_of {
    ($item : ty) => {
        {core::mem::size_of::<$item>()}
    };
}
