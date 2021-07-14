use core::fmt::Write;

use crate::{input::serial_read, io::{IoReader, IoWriter}, kernel::hardware::uart::*};
pub struct Serial;

impl IoReader<u8> for Serial {
    fn read(&mut self) -> Option<u8> {
        serial_read()
    }
}

impl IoWriter<u8> for Serial {
    fn write(&mut self, item : u8) {
        write_u8(item)
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_fmt(format_args!("{}\r\n", s))
    }
}


