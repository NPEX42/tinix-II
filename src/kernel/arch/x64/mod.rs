pub mod idt;
pub mod gdt;

use crate::kernel::InitResult;

pub fn init() -> InitResult<()> {
    if let Err(e) = gdt::init() {return Err(e)}
    if let Err(e) = idt::init() {return Err(e)}

    Ok(())
}