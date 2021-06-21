pub mod arch;
pub mod drivers;
pub mod hardware;

use bootloader::BootInfo;

pub fn boot(_boot_info : &'static BootInfo) {
    drivers::vga_dr::init().expect("Couldn't Initialize The VGA Driver...");
    arch::x64::init().expect("Couldn't Initialize x64 Specific Components...");
    hardware::pic::init().expect("Couldn't Initialize PIC8259...");
    hardware::pit::init().expect("Couldn't Initialize PIT...");
    arch::set_interrupt(0, crate::user::time::update).expect("Couldn't Setup IRQ0..");
    crate::user::input::init().expect("Unable To Start the Input System");
}

#[derive(Debug, Copy, Clone)]
pub struct InitError(&'static str);

pub type InitResult<T> = Result<T, InitError>;
