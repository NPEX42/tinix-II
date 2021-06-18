use core::fmt::*;

pub use uart_16550::*;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref SERIAL : Mutex<SerialPort> = Mutex::new(unsafe {SerialPort::new(0x3f8)});
}

pub fn write_u8(byte : u8) {
    SERIAL.lock().send(byte);
}

pub fn read_u8() -> u8 {
    SERIAL.lock().receive()
}

pub fn write_str(args : Arguments) {
    SERIAL.lock().write_fmt(args);
}
