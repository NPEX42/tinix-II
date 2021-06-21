pub mod idt;
pub mod gdt;

use crate::kernel::InitResult;

pub fn init() -> InitResult<()> {
    gdt::init();
    idt::init();

    Ok(())
}