pub mod arch;
pub mod drivers;
pub mod hardware;

use bootloader::BootInfo;

pub fn boot(_boot_info : &'static BootInfo) {
    drivers::vga::libs::init();
    arch::idt::init();
    hardware::pic::init();
    hardware::pit::init();
    arch::idt::set_handler(0, crate::user::time::update);
}

