pub mod x64;

pub fn set_interrupt(irq : u8, f : fn(u8)) -> Result<(),&'static str> {
    x64::idt::set_handler(irq, f);
    Ok(())
} 