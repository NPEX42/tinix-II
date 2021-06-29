use x86_64::structures::idt::*;
use x86_64::structures::gdt::*;
use super::*;
use crate::kernel::InitResult;

use lazy_static::lazy_static;
use spin::Mutex;

pub fn init() -> InitResult<()> {
    IDT.load();
    Ok(())
}

fn default_handler(irq : u8) {
    //crate::input::serial_println!("Fired IRQ #{}", irq);
}

// Translate IRQ into system interrupt
fn interrupt_index(irq: u8) -> u8 {
    (crate::kernel::hardware::pic::PIC_1_OFFSET + irq) as u8
}

lazy_static! {
    pub static ref IRQ_HANDLERS: Mutex<[fn(u8); 16]> = Mutex::new([default_handler; 16]);
    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault);

        idt.non_maskable_interrupt.set_handler_fn(nmi);

        idt[interrupt_index(0) as usize].set_handler_fn(irq0);
        idt[interrupt_index(1) as usize].set_handler_fn(irq1);
        idt[interrupt_index(2) as usize].set_handler_fn(irq2);
        idt[interrupt_index(3) as usize].set_handler_fn(irq3);
        idt[interrupt_index(4) as usize].set_handler_fn(irq4);
        idt[interrupt_index(5) as usize].set_handler_fn(irq5);
        idt[interrupt_index(6) as usize].set_handler_fn(irq6);
        idt[interrupt_index(7) as usize].set_handler_fn(irq7);


        idt[interrupt_index(128) as usize].set_handler_fn(swi_0);

        idt
    };
}

macro_rules! irq_handler {
    ($handler:ident, $irq:expr) => {
        extern "x86-interrupt" fn $handler(_stack_frame : InterruptStackFrame) {
            let handlers = IRQ_HANDLERS.lock();
            handlers[$irq]($irq);
            crate::kernel::hardware::pic::notify_end_of_interrupt(interrupt_index($irq));
        }        
    };
}

irq_handler!(irq0, 0);
irq_handler!(irq1, 1);
irq_handler!(irq2, 2);
irq_handler!(irq3, 3);
irq_handler!(irq4, 4);
irq_handler!(irq5, 5);
irq_handler!(irq6, 6);
irq_handler!(irq7, 7);
irq_handler!(irq8, 8);
irq_handler!(irq9, 9);
irq_handler!(irq10, 10);
irq_handler!(irq11, 11);
irq_handler!(irq12, 12);
irq_handler!(irq13, 13);
irq_handler!(irq14, 14);
irq_handler!(irq15, 15);

pub fn set_handler(irq : u8, func : fn(u8)) {
    IRQ_HANDLERS.lock()[irq as usize] = func;
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}, Error: {:x}", stack_frame, _error_code);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::print!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, ec : PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    use crate::println;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", ec);
    println!("{:#?}", stack_frame);
    loop {crate::time::sleep_ticks(10)}
}

extern "x86-interrupt" fn general_protection_fault(stack_frame: InterruptStackFrame, ec : u64) {
    use crate::println;
    println!("GP Fault Error Code: {:?}", ec);
    println!("{:#?}", stack_frame);
    loop {crate::time::sleep_ticks(10)}
}

extern "x86-interrupt" fn nmi(stack_frame: InterruptStackFrame) {
    crate::input::serial_println!("NMI Fired...");
} 


extern "x86-interrupt" fn swi_0(stack_frame: InterruptStackFrame) {
    crate::input::serial_println!("SWI0 Fired...");
}