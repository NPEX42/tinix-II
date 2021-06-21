pub mod arch;
pub mod drivers;
pub mod hardware;

use bootloader::BootInfo;

pub fn boot(_boot_info : &'static BootInfo) {
    drivers::vga_dr::init();
    arch::x64::init().expect("Couldn't Initialize x64 Specific Components...");
    hardware::pic::init().expect("Couldn't Initialize PIC8259...");
    hardware::pit::init().expect("Couldn't Initialize PIT...");
    arch::x64::idt::set_handler(0, crate::user::time::update);
}

#[derive(Debug, Copy, Clone)]
pub struct InitError(&'static str);

pub type InitResult<T> = Result<T, InitError>;
