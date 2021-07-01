pub mod x64;

pub fn set_interrupt(irq : u8, f : fn(u8)) -> Result<(),&'static str> {
    disable_irq(irq);
    x64::idt::set_handler(irq, f);
    enable_irq(irq);
    Ok(())
} 

pub fn enable_irq(irq : u8) {
    x64::idt::clear_irq_mask(irq);
}

pub fn disable_irq(irq : u8) {
    x64::idt::set_irq_mask(irq);
}