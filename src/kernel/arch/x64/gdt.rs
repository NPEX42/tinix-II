use x86_64::structures::gdt::*;
use x86_64::structures::tss::*;
use x86_64::VirtAddr;
use x86_64::instructions::segmentation::set_cs;
use x86_64::instructions::tables::load_tss;
use lazy_static::lazy_static;
use crate::kernel::InitResult;

pub const DOUBLE_FAULT_IST_INDEX : u16 = 0;

pub fn init() -> InitResult<()> {
    GDT.0.load();

    unsafe {
        set_cs(GDT.1.kernel_code);
        load_tss(GDT.1.tss);
    }

    Ok(())
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 8192 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };


    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {kernel_code, tss})
    };
}

pub struct Selectors {
    kernel_code : SegmentSelector,
    tss         : SegmentSelector,
}