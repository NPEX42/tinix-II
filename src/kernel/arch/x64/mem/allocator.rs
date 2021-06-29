use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, instructions::interrupts::without_interrupts, structures::paging::{Size4KiB, FrameAllocator, PhysFrame}};
use crate::{graphics::{Color, clear_screen, widgets::Renderable}, time};
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


pub struct NullFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for NullFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootFrameAllocator {
    pub unsafe fn init(memory_map : &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next : 0,
        }
    } 


    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = 
            regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());

        let frame_addresses = addr_ranges
            .flat_map(|r| r.step_by(4096));
        
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    pub fn frame_count(&self) -> usize {
        let frames = self.usable_frames();
        frames.count()
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}




use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}


use x86_64::{
    structures::paging::{
        mapper::MapToError, Mapper, Page, PageTableFlags,
    },
    VirtAddr,
};

use super::heap::*;

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    use crate::input::*;
    let mut pb = crate::graphics::widgets::progress_bar::ProgressBar::new(
        "Memory Init Progress: ",
        crate::heap::HEAP_SIZE, 
        Color::Blue, 
        Color::Cyan, 
        128
    );
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    let mut bytes_inited = 0;
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
        bytes_inited += 4096;
        let prog_len = 25;
        serial_print!("Initializing {} Bytes - [", HEAP_SIZE);
        let mut remaining = prog_len;
        let fill = bytes_inited as f32 / HEAP_SIZE as f32;
        for i in 0..=((prog_len as f32 * fill) as usize) {
            serial_print!("=");
            remaining -= 1;
        }

        for i in 0..=remaining {
            serial_print!(" ");
        }
        if page != page_range.last().unwrap() {
        serial_print!("] - {:02.2}%\r", fill * 100.0)
        }
    }
    serial_println!("] - OK                    ");

    unsafe {
        without_interrupts(|| {
            ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
        });
    }
    Ok(())
}