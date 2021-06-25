pub mod mapper;
pub mod allocator;
pub mod heap;

use bootloader::BootInfo;
use crate::kernel::InitResult;

use x86_64::{VirtAddr, structures::paging::{OffsetPageTable, Page, PageTable}};
use crate::input::{serial_print, serial_println};


pub unsafe fn init(physical_memory_offset: VirtAddr) -> InitResult<OffsetPageTable<'static>> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    Ok(OffsetPageTable::new(level_4_table, physical_memory_offset))
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    //list_all_used(physical_memory_offset.as_u64(),0,  &(*page_table_ptr));

    &mut *page_table_ptr // unsafe
}


pub unsafe fn list_all_used(offset : u64,depth : usize, table : &PageTable) {
    for (i, frame) in table.iter().enumerate() {
        if !frame.is_unused() {
            if depth >= 1 {
                serial_print!("|");
            }
            for _ in 1..depth {
                serial_print!("-");
            }
            serial_println!("{}: D{} Frame: 0x{:0x}",i, depth, frame.addr());

            list_all_used(offset, depth + 1, &get_page_table(offset, frame.addr().as_u64()));
        }
        if i >= 512 {return};
        if depth >= 4 {return};
    }
}

pub unsafe fn get_page_table(offset : u64, address : u64) -> &'static mut PageTable {
    let virt = VirtAddr::new_truncate(address + offset);
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr // unsafe
}