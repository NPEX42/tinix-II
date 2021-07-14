use crate::kernel::InitResult;
use x86_64::instructions::{interrupts, port::Port};


const PIT_DIVIDER: usize = 1193;

pub fn init() -> InitResult<()> {
    let divider = if PIT_DIVIDER < 65536 { PIT_DIVIDER } else { 0 };
    set_reload(divider as u16);
    Ok(())
}

fn set_reload(divider : u16) {
    interrupts::without_interrupts(|| {
        let bytes = divider.to_le_bytes();
        let mut cmd: Port<u8> = Port::new(0x43);
        let mut data: Port<u8> = Port::new(0x40);
        unsafe {
            cmd.write(0x36);
            data.write(bytes[0]);
            data.write(bytes[1]);
        }
    });
}