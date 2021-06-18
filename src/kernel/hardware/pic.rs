use super::InitError;
use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// Map PIC interrupts to 0x20 through 0x2f.
static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
pub fn init() -> Result<(),InitError> {
    unsafe {PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
    Ok(())
}

pub fn notify_end_of_interrupt(index : u8) {
    unsafe {PICS.lock().notify_end_of_interrupt(index)};
}