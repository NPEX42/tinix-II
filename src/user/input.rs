use core::fmt::Arguments;
use x86_64::instructions::interrupts::without_interrupts;

pub fn serial_read() -> Option<u8> {
    Some(crate::kernel::hardware::uart::read_u8())
}

pub fn serial_write(byte : u8){
    crate::kernel::hardware::uart::write_u8(byte);
}

pub macro serial_print {
    ($($arg:tt)*) => { $crate::user::input::_print(format_args!($($arg)*)) }
}

pub macro serial_println {
    ($($arg:tt)*) => { $crate::user::input::serial_print!("{}\r\n",format_args!($($arg)*)) }
}

pub fn _print(args : Arguments) {
    without_interrupts(|| {
        crate::kernel::hardware::uart::write_str(args);
    });
}
    